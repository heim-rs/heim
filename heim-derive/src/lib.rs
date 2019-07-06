#![recursion_limit = "128"]

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse;
use syn::spanned::Spanned;

mod unit;

#[derive(Debug)]
struct ImplTarget {
    target: syn::Path,
    cfg: syn::Meta,
}

impl parse::Parse for ImplTarget {
    fn parse(input: parse::ParseStream) -> parse::Result<Self> {
        let target: syn::Path = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let cfg: syn::Meta = input.parse()?;
        Ok(ImplTarget {
            target,
            cfg,
        })
    }
}

/// Augument OS-specific trait with boilerplate-generation.
///
/// Automatically implements this trait for target struct,
/// generates all opaque methods and attaches #\[cfg()\] attribute.
///
/// Should be used as following:
///
/// ```norun
/// #[heim_derive::os_ext_for(crate::CpuTimes, cfg(target_os = "linux"))]
/// pub trait CpuTimesExt {
///     fn foo(&self) -> u32;
/// }
/// ```
///
/// Will generate the code similar to following:
///
/// ```norun
/// pub trait CpuTimesExt {
///     fn foo(&self) -> u32;
/// }
///
/// #[cfg(target_os = "linux")
/// impl CpuTimesExt for crate::CpuTimes {
///     fn foo(&self) -> u32 {
///         self.as_ref().foo()
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn os_ext_for(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ImplTarget {
        target: target_struct,
        cfg: cfg_attr,
    } = syn::parse_macro_input!(attr as ImplTarget);

    let trait_def: syn::ItemTrait = syn::parse(item).unwrap();
    let trait_name = trait_def.ident.clone();

    // Generating methods for each trait method.
    // Resulting method will have the same name and will call `self.as_ref().<name>()`
    let methods = trait_def
        .items
        .iter()
        .filter_map(|item| match item {
            syn::TraitItem::Method(method) => Some(method),
            _ => None,
        })
        .map(|source| {
            let name = &source.sig.ident;
            let output = &source.sig.decl.output;
            quote::quote_spanned! {source.span()=>
                fn #name(&self) #output {
                    self.as_ref().#name()
                }
            }
        });

    let implementation = quote::quote! {
        #(#methods)*
    };

    // TODO: Add doc attribute with `cfg_attr` in it,
    // so it would be easier to understand that trait implementation exists only for specific targets
    let expanded = quote::quote! {
        #trait_def

        #[#cfg_attr]
        impl #trait_name for #target_struct {
            #implementation
        }
    };

    TokenStream::from(expanded)
}

/// Augument wrapper around OS-specific implementation struct with conversion traits.
///
/// Will auto-generate `AsRef` and `From` for underline struct with `#\[doc(hidden)\]` attribute
#[proc_macro_derive(ImplWrap)]
pub fn impl_wrap(input: TokenStream) -> TokenStream {
    let struct_type: syn::ItemStruct = syn::parse(input).unwrap();
    let struct_ident = &struct_type.ident;
    let generics = &struct_type.generics;
    let field = match struct_type.fields {
        // Only newtype structs are supported
        syn::Fields::Unnamed(ref unnamed) if unnamed.unnamed.len() == 1 => unnamed.unnamed[0].ty.clone(),
        // TODO: Nice compile error
        _ => unimplemented!(),
    };

    let expanded = quote::quote! {
        #[doc(hidden)]
        impl#generics AsRef<#field> for #struct_ident#generics {
            fn as_ref(&self) -> &#field {
                &self.0
            }
        }

        #[doc(hidden)]
        impl#generics AsMut<#field> for #struct_ident#generics {
            fn as_mut(&mut self) -> &mut #field {
                &mut self.0
            }
        }

        #[doc(hidden)]
        impl#generics From<#field> for #struct_ident#generics {
            fn from(inner: #field) -> #struct_ident {
                #struct_ident(inner)
            }
        }
    };

    expanded.into()
}

#[derive(Debug)]
enum GetterType {
    Copy,
    AsRef,
    AsStr,
}

fn attr_name(attr: &syn::Attribute) -> Option<syn::Ident> {
    attr.interpret_meta().map(|v| v.name())
}

// Based on the `getset` crate code
fn getter_type(field: &syn::Field) -> GetterType {
    let inner = field
        .attrs
        .iter()
        .filter(|v| attr_name(v).expect("Could not get attribute") == "getter")
        .last()
        .and_then(|v| v.parse_meta().ok());

    match inner {
        Some(syn::Meta::List(list)) => list
            .nested
            .iter()
            .filter_map(|meta_item| match meta_item {
                syn::NestedMeta::Meta(meta) if meta.name() == "as_ref" => Some(GetterType::AsRef),
                syn::NestedMeta::Meta(meta) if meta.name() == "as_str" => Some(GetterType::AsStr),
                _ => None,
            })
            .next()
            .unwrap_or(GetterType::Copy),
        _ => GetterType::Copy,
    }
}

/// Generates getters for all struct fields.
///
/// This is quite similar to `getset` or other crates, but highly specific for `heim` case.
///
/// OS-specific structs are usually very thin and contains `Copy`-able fields, therefore
/// there is no need to return reference to them, it is easier to return a copy (ex. field is u32),
/// that's why generated getters are returning data copies.
///
/// Unfortunately, `getset` crate does not allows this behavior at the moment.
/// If it will, it is better to remove that macro at all.
#[proc_macro_derive(Getter, attributes(getter))]
pub fn impl_getters(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let struct_name = &ast.ident;

    let methods = if let syn::Data::Struct(syn::DataStruct {
        ref fields, ..
    }) = ast.data
    {
        fields
            .iter()
            .map(|field| {
                let name = &field.ident;
                let type_ = &field.ty;

                match getter_type(&field) {
                    GetterType::Copy => {
                        quote::quote! {
                            pub fn #name(&self) -> #type_ {
                                self.#name
                            }
                        }
                    }
                    GetterType::AsRef => {
                        quote::quote! {
                            pub fn #name(&self) -> &#type_ {
                                self.#name.as_ref()
                            }
                        }
                    }
                    GetterType::AsStr => {
                        quote::quote! {
                            pub fn #name(&self) -> &str {
                                self.#name.as_str()
                            }
                        }
                    }
                }
            })
            .collect::<Vec<_>>()
    } else {
        unimplemented!()
    };

    let expanded = quote::quote! {
        impl #struct_name {
            #(#methods)*
        }
    };

    expanded.into()
}

#[proc_macro_derive(Unit)]
pub fn unit(input: TokenStream) -> TokenStream {
    let struct_type: syn::ItemStruct = syn::parse(input).unwrap();
    let field = match struct_type.fields {
        // Only newtype structs are supported
        syn::Fields::Unnamed(ref unnamed) if unnamed.unnamed.len() == 1 => unnamed.unnamed[0].ty.clone(),
        // TODO: Nice compile error
        _ => unimplemented!(),
    };

    let implementation = unit::implementation(&struct_type, &field);
    let ops = unit::ops(&struct_type, &field);

    let expanded = quote::quote! {
        #implementation
        #ops
    };

    expanded.into()
}

/// Used for `#[runtime::test]`-annotated functions
///
/// Will not run the annotated function if it is called in the CI environment.
///
/// It is important to put it **before** the `#[runtime::test]` attribute, like that:
///
/// ```text
/// #[heim_derive::skip_ci]
/// #[runtime::test]
/// async fn test_foo() {}
/// ```
///
/// Supported CI:
///  * Azure Pipelines
///  * Cirrus CI
#[proc_macro_attribute]
pub fn skip_ci(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func: syn::ItemFn = syn::parse(item).unwrap();
    let cfg = proc_macro2::TokenStream::from(attr);
    let attrs = &func.attrs;
    let vis = &func.vis;
    let constness = &func.constness;
    let unsafety = &func.unsafety;
    let asyncness = &func.asyncness;
    let abi = &func.abi;
    let ident = &func.ident;
    let body = &func.block;

    let ident_repr = format!("{}", &func.ident);

    let expanded = quote::quote! {
        #(#attrs)*
        #vis #constness #unsafety #asyncness #abi fn #ident() {
            #[cfg(test)]
            async fn inner() {
                #body
            }

            #[cfg(test)]
            fn inner_run() {
                let _ = async {
                    inner().await
                };
            }

            let in_ci = ::std::env::vars()
                .any(|(key, _)| {
                    match key.as_str() {
                        "CIRRUS_CI" => true,
                        "TF_BUILD" => true,
                        _ => false
                    }
                });

            if cfg!(#cfg) && in_ci {
                eprintln!("test {} ... will be ignored because of CI environment", #ident_repr);
            } else {
                inner_run();
            }
        }
    };

    expanded.into()
}
