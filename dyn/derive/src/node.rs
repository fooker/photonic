use darling::{ast, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_quote};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(photonic), supports(struct_named))]
struct StructReceiver {
    ident: syn::Ident,

    generics: syn::Generics,

    data: ast::Data<(), FieldReceiver>,
}

#[derive(Debug, FromField)]
#[darling(attributes(photonic))]
#[darling(and_then = Self::check)]
struct FieldReceiver {
    ident: Option<syn::Ident>,

    ty: syn::Type,

    node: darling::util::Flag,
    attr: darling::util::Flag,
}

impl FieldReceiver {
    fn check(self) -> darling::Result<Self> {
        if self.node.is_present() && self.attr.is_present() {
            return Err(darling::Error::custom("Field can not be attr and node").with_span(&self.node.span()));
        } else {
            return Ok(self);
        };
    }
}

pub fn expand(input: &mut DeriveInput) -> syn::Result<TokenStream> {
    let mut receiver: StructReceiver = StructReceiver::from_derive_input(&input)?;

    let ident = receiver.ident;

    let fields = receiver.data.take_struct().expect("Should never be an enum").fields;

    let config_nodes = fields.iter()
        .filter(|field| field.node.is_present())
        .map(|field| {
            let ident = field.ident.as_ref().expect("Should never be unnamed field");
            let ty = &field.ty;
            return quote!(#ident: <#ty as ::photonic_dyn::Dynamic>::Config,);
        });

    let config_attrs = fields.iter()
        .filter(|field| field.attr.is_present())
        .map(|field| {
            let ident = field.ident.as_ref().expect("Should never be unnamed field");
            let ty = &field.ty;
            return quote!(#ident: <#ty as ::photonic_dyn::Dynamic>::Config,);
        });

    let config_literals = fields.iter()
        .filter(|field| !field.node.is_present() && !field.attr.is_present())
        .map(|field| {
            let ident = field.ident.as_ref().expect("Should never be unnamed field");
            let ty = &field.ty;
            return quote!(#ident: #ty,);
        });

    let initialize = fields.iter()
        .map(|field| {
            let ident = field.ident.as_ref().expect("Should never be unnamed field");
            let ty = &field.ty;

            if field.node.is_present() || field.attr.is_present() {
                return quote!(#ident: <#ty as ::photonic_dyn::Dynamic>::from_config(
                    builder,
                    stringify!(#ident),
                    config.#ident)?,
                );
            } else {
                return quote!(#ident: config.#ident.into(),);
            }
        });

    {
        let predicates = &mut receiver.generics.where_clause
            .get_or_insert_with(|| parse_quote!(where))
            .predicates;

        for field in fields.iter() {
            let ty = &field.ty;
            if field.node.is_present() || field.attr.is_present() {
                predicates.push(parse_quote!(#ty: ::photonic_dyn::Dynamic + 'static));
            } else {
                predicates.push(parse_quote!(#ty: ::photonic_dyn::serde::de::DeserializeOwned));
            }
        }
    }

    let config_generics = receiver.generics.clone();

    {
        let predicates = &mut receiver.generics.where_clause
            .get_or_insert_with(|| parse_quote!(where))
            .predicates;
        predicates.push(parse_quote!(::palette::rgb::Rgb: ::palette::FromColor<<<Self as ::photonic::NodeDecl>::Node as ::photonic::Node>::Element>));
        predicates.push(parse_quote!(Self: ::photonic::NodeDecl));
    }

    let (derive_impl_generics, derive_type_generics, derive_where_clause) = receiver.generics.split_for_impl();

    let (config_impl_generics, config_type_generics, config_where_clause) = config_generics.split_for_impl();

    return Ok(quote! {
        impl #derive_impl_generics ::photonic_dyn::DynamicNode for #ident #derive_type_generics #derive_where_clause
        {
            const KIND: &'static str = <<#ident #derive_type_generics as ::photonic::NodeDecl>::Node as ::photonic::Node>::KIND;

            fn factory<Builder>() -> ::photonic_dyn::registry::NodeFactory<Builder>
                where Builder: ::photonic_dyn::builder::NodeBuilder,
            {
                #[derive(::photonic_dyn::serde::Deserialize)]
                #[serde(crate = "::photonic_dyn::serde")]
                pub struct Config #config_impl_generics #config_where_clause {
                    #(
                        #[serde(bound="")]
                        #config_attrs
                    )*

                    #(
                        #[serde(bound="")]
                        #config_nodes
                    )*

                    #(
                        #[serde(bound="")]
                        #config_literals
                    )*
                }

                let factory = |config: ::photonic_dyn::config::Anything, builder: &mut Builder| -> ::anyhow::Result<::photonic_dyn::boxed::BoxedNodeDecl> {
                    let config: Config #config_type_generics = ::photonic_dyn::serde::Deserialize::deserialize(config)?;

                    return ::anyhow::Result::Ok(Box::new(Self {
                        #(#initialize)*
                    }));
                };

                return Box::new(factory);
            }
        }
    }.into());
}
