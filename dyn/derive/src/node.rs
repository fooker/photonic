use darling::util::Flag;
use darling::{ast, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;

use crate::bounds;

fn field_bound(field: &FieldReceiver) -> Vec<syn::WherePredicate> {
    let ty = &field.ty;
    return if field.is_dynamic() {
        vec![parse_quote!(#ty: ::photonic_dyn::dynamic::Dynamic + 'static)]
    } else {
        vec![parse_quote!(#ty: ::photonic_dyn::serde::de::DeserializeOwned + 'static)]
    };
}

fn expand_config(parent: &syn::Ident, generics: &syn::Generics, fields: &[FieldReceiver]) -> syn::Result<TokenStream> {
    let generics = bounds::with_where_predicates_from_fields(generics, fields, field_bound);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let field_names =
        fields.iter().map(|field| field.ident.as_ref().expect("Fields should never be unnamed")).collect::<Vec<_>>();

    let field_types = fields.iter().map(|field| {
        let ty = &field.ty;
        return if field.is_dynamic() {
            quote!(<#ty as ::photonic_dyn::dynamic::Dynamic>::Config)
        } else {
            quote!(#ty)
        };
    });

    let field_values = fields.iter().map(|field| {
        let ident = &field.ident;
        let ty = &field.ty;
        return if field.is_dynamic() {
            quote!(<#ty as ::photonic_dyn::dynamic::Dynamic>::from_config(
                builder,
                stringify!(#ident),
                self.#ident)?
            )
        } else {
            quote!(self.#ident)
        };
    });

    return Ok(parse_quote! {
        #[derive(::photonic_dyn::serde::Deserialize)]
        #[serde(crate = "::photonic_dyn::serde")]
        struct Config #impl_generics #where_clause {
            #(
                #[serde(bound="")]
                #field_names: #field_types,
            )*
        }

        impl #impl_generics Config #ty_generics #where_clause {
            pub fn build<__Builder>(self, builder: &mut __Builder) -> ::anyhow::Result<#parent #ty_generics>
                where __Builder: ::photonic_dyn::builder::NodeBuilder,
            {
                return Ok(#parent {
                    #( #field_names: #field_values, )*
                });
            }
        }
    });
}

pub fn expand(input: &mut syn::DeriveInput) -> syn::Result<TokenStream> {
    let receiver: StructReceiver = StructReceiver::from_derive_input(&input)?;

    let ident = receiver.ident;

    let fields = receiver.data.take_struct().expect("Should never be an enum").fields;

    let config = expand_config(&ident, &receiver.generics, &fields)?;

    let generics = bounds::with_where_predicates(&receiver.generics, [
        parse_quote!(::palette::rgb::Rgb: ::palette::FromColor<<<Self as ::photonic::NodeDecl>::Node as ::photonic::Node>::Element>),
        parse_quote!(Self: ::photonic::NodeDecl),
    ]);

    let generics = bounds::with_where_predicates_from_fields(&generics, &fields, field_bound);

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    return Ok(parse_quote! {
        impl #impl_generics ::photonic_dyn::DynamicNode for #ident #type_generics #where_clause
        {
            const KIND: &'static str = <<#ident #type_generics as ::photonic::NodeDecl>::Node as ::photonic::Node>::KIND;

            fn factory<Builder>() -> ::photonic_dyn::registry::NodeFactory<Builder>
                where Builder: ::photonic_dyn::builder::NodeBuilder,
            {
                #config

                let factory = |config: ::photonic_dyn::config::Anything, builder: &mut Builder| -> ::anyhow::Result<::photonic_dyn::boxed::BoxedNodeDecl> {
                    let config: Config #type_generics = ::photonic_dyn::serde::Deserialize::deserialize(config)?;

                    return ::anyhow::Result::Ok(Box::new(config.build(builder)?));
                };

                return Box::new(factory);
            }
        }
    });
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(photonic), supports(struct_named))]
struct StructReceiver {
    ident: syn::Ident,

    generics: syn::Generics,

    data: ast::Data<(), FieldReceiver>,
}

#[derive(Debug, FromField)]
#[darling(attributes(photonic), and_then = Self::check)]
struct FieldReceiver {
    ident: Option<syn::Ident>,

    ty: syn::Type,

    node: Flag,
    attr: Flag,
}

impl FieldReceiver {
    fn check(self) -> darling::Result<Self> {
        if self.node.is_present() && self.attr.is_present() {
            return Err(darling::Error::custom("Field can not be node and attr").with_span(&self.node.span()));
        }

        return Ok(self);
    }

    pub fn is_dynamic(&self) -> bool {
        return self.node.is_present() || self.attr.is_present();
    }
}
