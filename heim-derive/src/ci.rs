use proc_macro::TokenStream;

pub fn skip_ci(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func: syn::ItemFn = syn::parse(item).unwrap();
    let cfg = proc_macro2::TokenStream::from(attr);
    let attrs = &func.attrs;
    let vis = &func.vis;
    let constness = &func.sig.constness;
    let unsafety = &func.sig.unsafety;
    let asyncness = &func.sig.asyncness;
    let abi = &func.sig.abi;
    let ident = &func.sig.ident;
    let body = &func.block;

    let ident_repr = format!("{}", &func.sig.ident);

    let expanded = quote::quote! {
        #(#attrs)*
        #vis #constness #unsafety #asyncness #abi fn #ident() {
            let in_ci = ::std::env::vars()
                .any(|(key, _)| {
                    match key.as_str() {
                        // Azure Pipelines
                        "TF_BUILD" => true,
                        // Github Actions
                        "GITHUB_ACTIONS" => true,
                        _ => false
                    }
                });

            if cfg!(#cfg) && in_ci {
                eprintln!("test {} ... will be ignored because of CI environment", #ident_repr);
            } else {
                #body
            }
        }
    };

    expanded.into()
}
