use proc_macro::TokenStream;
use syn::spanned::Spanned;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum GetterType {
    Copy,
    AsRef,
    AsStr,
}

/// Parse the struct field attributes, find the `#[getter]` one
/// and determine what getter type should be used.
fn getter_type(field: &syn::Field) -> GetterType {
    let inner = field
        .attrs
        .iter()
        .filter(|attr| match attr.path.get_ident() {
            None => false,
            Some(ident) => ident == "getter",
        })
        .last()
        .and_then(|v| v.parse_meta().ok());

    match inner {
        Some(syn::Meta::List(list)) => list
            .nested
            .iter()
            .find_map(|meta_item| match meta_item {
                syn::NestedMeta::Meta(syn::Meta::Path(path)) if path.is_ident("as_ref") => {
                    Some(GetterType::AsRef)
                }
                syn::NestedMeta::Meta(syn::Meta::Path(path)) if path.is_ident("as_str") => {
                    Some(GetterType::AsStr)
                }
                _ => None,
            })
            .unwrap_or(GetterType::Copy),
        _ => GetterType::Copy,
    }
}

pub(crate) fn parse(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let struct_name = &ast.ident;

    let methods = match ast.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => fields
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
            .collect::<Vec<_>>(),
        _ => {
            let tokens = quote::quote_spanned! { ast.span() =>
                compile_error!("Can be applied to struct only");
            };

            return TokenStream::from(tokens);
        }
    };

    let expanded = quote::quote! {
        impl #struct_name {
            #(#methods)*
        }
    };

    expanded.into()
}
