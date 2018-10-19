// The `quote!` macro requires deep recursion.
#![recursion_limit = "512"]

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use self::proc_macro::{TokenStream};
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
struct AttrField {
    ident: Ident,
    name: String,
}

fn collect_meta_fields(fields: &Punctuated<Field, Token![,]>) -> (Vec<NodeField>, Vec<AttrField>) {
    let (mut nodes, mut attrs) = (vec![], vec![]);

    for (i, field) in fields.iter().enumerate() {
        for attr in field.attrs.iter() {
            let ident = field.ident.clone()
                             .unwrap_or_else(|| Ident::new(&format!("{}", i), Span::call_site()));

            let meta = attr.interpret_meta();
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

                if meta.ident == "attr" {
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

                    attrs.push(AttrField {
                        ident: ident.clone(),
                        name: name.unwrap_or_else(|| ident.to_string()),
                    });
                }
            } else {
                // FIXME: Error reporting?
            }
        }
    }

    return (nodes, attrs);
}

/// Iterates over all fields of the struct and returns all owned boxed attributes
fn collect_meta(input: &DeriveInput) -> (Vec<NodeField>, Vec<AttrField>) {
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

#[proc_macro_derive(Node, attributes(node, attr))]
#[allow(unused)]
pub fn derive_node(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;
    let class = ident.to_string();
    let (nodes, attrs) = collect_meta(&input);

    // FIXME: Pre-generate the vectors or use something lazy
    let nodes = nodes.iter().map(|NodeField { ref name, ref ident }| quote! {
        ::photonic::reflection::NodeRef{
            ptr: self.#ident.as_ref(),
            name: #name,
        }
    });

    let attrs = attrs.iter().map(|AttrField { ref name, ref ident }| quote! {
        ::photonic::reflection::AttributeRef{
            ptr: self.#ident.as_ref(),
            name: #name,
        }
    });

    return TokenStream::from(quote! {
        #[automatically_derived]
        impl ::photonic::core::Node for #ident {
            fn class(&self) -> &str {
                return #class;
            }

            fn childs(&self) -> Vec<photonic::reflection::NodeRef> {
                return vec![#(#nodes),*];
            }

            fn attributes(&self) -> Vec<photonic::reflection::AttributeRef> {
                return vec![#(#attrs),*];
            }
        }
    });
}
