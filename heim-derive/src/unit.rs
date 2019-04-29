use proc_macro2::{TokenStream};
use syn::{ItemStruct, Type};

pub fn implementation(struct_item: &ItemStruct, inner: &Type) -> TokenStream {
    let ident = &struct_item.ident;
    let generics = &struct_item.generics;

    quote::quote! {
        impl#generics #ident#generics {
            pub fn new(inner: #inner) -> Self#generics {
                Self(inner)
            }

            pub fn get(self) -> #inner {
                self.0
            }
        }

        impl#generics ::std::convert::AsRef<#inner> for #ident {
            fn as_ref(&self) -> &#inner {
                &self.0
            }
        }

        impl#generics ::std::convert::AsMut<#inner> for #ident {
            fn as_mut(&mut self) -> &mut #inner {
                &mut self.0
            }
        }
    }
}

pub fn ops(struct_item: &ItemStruct, inner: &Type) -> TokenStream {
    let ident = &struct_item.ident;
    let generics = &struct_item.generics;

    quote::quote! {
        impl#generics ::std::ops::Add<#ident> for #ident#generics {
            type Output = #ident;

            fn add(self, other: #ident) -> Self::Output {
                #ident(self.0 + other.0)
            }
        }

        impl#generics ::std::ops::Add<#inner> for #ident#generics {
            type Output = #ident;

            fn add(self, other: #inner) -> Self::Output {
                #ident(self.0 + other)
            }
        }

        impl#generics ::std::ops::Sub<#ident> for #ident#generics {
            type Output = #ident;

            fn sub(self, other: #ident) -> Self::Output {
                #ident(self.0 - other.0)
            }
        }

        impl#generics ::std::ops::Sub<#inner> for #ident#generics {
            type Output = #ident;

            fn sub(self, other: #inner) -> Self::Output {
                #ident(self.0 - other)
            }
        }

        impl#generics ::std::ops::Div<#ident> for #ident#generics {
            type Output = #ident;

            fn div(self, other: #ident) -> Self::Output {
                #ident(self.0 / other.0)
            }
        }

        impl#generics ::std::ops::Div<#inner> for #ident#generics {
            type Output = #ident;

            fn div(self, other: #inner) -> Self::Output {
                #ident(self.0 / other)
            }
        }
    }
}
