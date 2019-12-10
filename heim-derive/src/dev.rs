use proc_macro::TokenStream;
use syn::spanned::Spanned;

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

            futures_executor::block_on(async {
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
            futures_executor::block_on(async {
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
                let _ = futures_executor::block_on(async {
                    #body
                });
            });
        }
    };

    result.into()
}
