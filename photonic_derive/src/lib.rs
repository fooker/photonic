// The `quote!` macro requires deep recursion.
#![recursion_limit = "512"]

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use self::proc_macro::TokenStream;
use syn::*;
use syn::export::Span;
use syn::punctuated::Punctuated;

#[derive(Debug)]
#[allow(unused)]
struct NodeField {
    ident: Ident,
    name: String,
}

#[derive(Debug)]
#[allow(unused)]
struct ValueField {
    ident: Ident,
    name: String,
}

fn collect_meta_fields(fields: &Punctuated<Field, Token![,]>) -> (Vec<NodeField>, Vec<ValueField>) {
    let (mut nodes, mut values) = (vec![], vec![]);

    for (i, field) in fields.iter().enumerate() {
        for value in field.attrs.iter() {
            let ident = field.ident.clone()
                             .unwrap_or_else(|| Ident::new(&format!("{}", i), Span::call_site()));

            let meta = value.interpret_meta();
            if let Some(Meta::List(ref meta)) = meta {
                if meta.ident == "node" {
                    let mut name: Option<String> = None;

                    for item in meta.nested.iter() {
                        match item {
                            NestedMeta::Meta(Meta::NameValue(ref m)) if m.ident == "name" => {
                                if let Lit::Str(ref s) = m.lit {
                                    name = Some(s.value());
                                } else {
                                    // FIXME: Error reporting: only string supported for name
                                }
                            }
                            _ => {
                                // FIXME: Error reporting: unknown property or literal
                            }
                        }
                    }

                    nodes.push(NodeField {
                        ident: ident.clone(),
                        name: name.unwrap_or_else(|| ident.to_string()),
                    });
                }

                if meta.ident == "value" {
                    let mut name: Option<String> = None;

                    for item in meta.nested.iter() {
                        match item {
                            NestedMeta::Meta(Meta::NameValue(ref m)) if m.ident == "name" => {
                                if let Lit::Str(ref s) = m.lit {
                                    name = Some(s.value());
                                } else {
                                    // FIXME: Error reporting: only string supported for name
                                }
                            }
                            _ => {
                                // FIXME: Error reporting: unknown property or literal
                            }
                        }
                    }

                    values.push(ValueField {
                        ident: ident.clone(),
                        name: name.unwrap_or_else(|| ident.to_string()),
                    });
                }
            } else {
                // FIXME: Error reporting?
            }
        }
    }

    return (nodes, values);
}

/// Iterates over all fields of the struct and returns all owned boxed value
fn collect_meta(input: &DeriveInput) -> (Vec<NodeField>, Vec<ValueField>) {
    if let Data::Struct(ref data) = input.data {
        match data.fields {
            Fields::Named(ref data) => {
                return collect_meta_fields(&data.named);
            }

            Fields::Unnamed(ref data) => {
                return collect_meta_fields(&data.unnamed);
            }

            Fields::Unit => {
                return (vec![], vec![]);
            }
        }
    } else {
        // FIXME: Error reporting: Only struct is supported for node derive
        unimplemented!()
    }
}

#[proc_macro_derive(Inspection, attributes(node, value))]
#[allow(unused)]
pub fn derive_inspection(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;
    let (nodes, values) = collect_meta(&input);

    // FIXME: Pre-generate the vectors or use something lazy
    let nodes = nodes.iter().map(|NodeField { ref name, ref ident }| quote! {
        ::photonic::inspection::NodeRef{
            name: #name,
            ptr: self.#ident.as_ref(),
        }
    });

    let values = values.iter().map(|ValueField { ref name, ref ident }| quote! {
        ::photonic::inspection::ValueRef{
            name: #name,
            ptr: ::photonic::inspection::ValuePtr::Float(&self.#ident)
        }
    });

    return TokenStream::from(quote! {
        #[automatically_derived]
        impl ::photonic::inspection::Inspection for #ident {
            fn children(&self) -> Vec<photonic::inspection::NodeRef> {
                return vec![#(#nodes),*];
            }

            fn values(&self) -> Vec<photonic::inspection::ValueRef> {
                return vec![#(#values),*];
            }
        }
    });
}
