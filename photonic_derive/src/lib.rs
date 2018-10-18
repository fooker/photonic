extern crate proc_macro;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use self::proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(Node)]
pub fn derive_node(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;

    let derived = quote! {
        impl ::photonic::core::Node for #ident {
            fn attributes(&self) -> Vec<&photonic::attributes::Attribute> {
                return vec![];
            }
        }
    };

    TokenStream::from(derived)
}
