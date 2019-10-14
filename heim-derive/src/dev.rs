use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse;
use syn::spanned::Spanned;

pub fn impl_wrap(input: TokenStream) -> TokenStream {
    let struct_type: syn::ItemStruct = syn::parse(input).unwrap();
    let struct_ident = &struct_type.ident;
    let generics = &struct_type.generics;
    let field = match struct_type.fields {
        // Only newtype structs are supported
        syn::Fields::Unnamed(ref unnamed) if unnamed.unnamed.len() == 1 => &unnamed.unnamed[0].ty,
        _ => {
            let tokens = quote::quote_spanned! { struct_type.span() =>
                compile_error!("Can only be applied to a newtype struct with one field in it");
            };
            return TokenStream::from(tokens);
        }
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

struct ImplTarget {
    target: syn::Path,
    cfg: syn::Meta,
}

impl parse::Parse for ImplTarget {
    fn parse(input: parse::ParseStream) -> parse::Result<Self> {
        let target: syn::Path = input.parse()?;
        let _ = input.parse::<syn::Token![,]>()?;
        let cfg: syn::Meta = input.parse()?;
        Ok(ImplTarget { target, cfg })
    }
}

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
            let sig = &source.sig;
            let name = &source.sig.ident;

            // Collect all inputs,
            // fetch the pattern part (as in `a: i32` we need the `a` only)
            // and pass them into the inner function
            let args = &source
                .sig
                .inputs
                .iter()
                .filter_map(|input| match input {
                    syn::FnArg::Receiver(..) => None,
                    syn::FnArg::Typed(pat_type) => Some(pat_type.pat.to_token_stream()),
                })
                .collect::<Vec<_>>();
            quote::quote_spanned! {source.span()=>
                #sig {
                    self.as_ref().#name(#(#args),*)
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

#[cfg(not(test))]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let ret = &input.sig.output;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;

    if name != "main" {
        let tokens = quote::quote_spanned! { name.span() =>
          compile_error!("only the main function can be tagged with #[heim_derive::main]");
        };
        return TokenStream::from(tokens);
    }

    if input.sig.asyncness.is_none() {
        let tokens = quote::quote_spanned! { input.span() =>
          compile_error!("the async keyword is missing from the function declaration");
        };
        return TokenStream::from(tokens);
    }

    let result = quote::quote! {
        fn main() #ret {
            #(#attrs)*
            async fn main() #ret {
                #body
            }

            async_std::task::block_on(async {
                main().await
            })
        }

    };

    result.into()
}

pub fn test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let ret = &input.sig.output;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;

    if input.sig.asyncness.is_none() {
        let tokens = quote::quote_spanned! { input.span() =>
          compile_error!("the async keyword is missing from the function declaration");
        };
        return TokenStream::from(tokens);
    }

    let result = quote::quote! {
        #[test]
        #(#attrs)*
        fn #name() #ret {
            async_std::task::block_on(async {
                #body
            })
        }
    };

    result.into()
}

pub fn bench(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;

    if input.sig.asyncness.is_none() {
        let tokens = quote::quote_spanned! { input.span() =>
          compile_error!("the async keyword is missing from the function declaration");
        };
        return TokenStream::from(tokens);
    }

    let result = quote::quote! {
        #[bench]
        #(#attrs)*
        fn #name(b: &mut test::Bencher) {
            b.iter(|| {
                let _ = async_std::task::block_on(async {
                    #body
                });
            });
        }
    };

    result.into()
}
