mod node;
mod attr;

mod bounds;

#[proc_macro_derive(DynamicNode, attributes(photonic))]
pub fn derive_dynamic_node(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = syn::parse_macro_input!(input as syn::DeriveInput);
    return node::expand(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into();
}

#[proc_macro_derive(DynamicAttr, attributes(photonic))]
pub fn derive_dynamic_attr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = syn::parse_macro_input!(input as syn::DeriveInput);
    return attr::expand(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into();
}
