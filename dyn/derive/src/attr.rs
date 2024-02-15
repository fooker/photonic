use darling::{ast, FromDeriveInput, FromField};
use darling::util::Flag;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;
use crate::bounds;

fn field_bound(field: &FieldReceiver) -> Vec<syn::WherePredicate> {
    let ty = &field.ty;
    return if field.is_dynamic() {
        vec![ parse_quote!(#ty: ::photonic_dyn::dynamic::Dynamic + 'static) ]
    } else {
        vec![ parse_quote!(#ty: ::photonic_dyn::serde::de::DeserializeOwned + 'static) ]
    }
}

fn expand_config(parent: &syn::Ident,
                 generics: &syn::Generics,
                 fields: &[FieldReceiver]) -> syn::Result<TokenStream> {
    let generics = bounds::with_where_predicates_from_fields(generics, fields, field_bound);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let field_names = fields.iter()
        .map(|field| field.ident.as_ref().expect("Fields should never be unnamed"))
        .collect::<Vec<_>>();

    let field_types = fields.iter().map(|field| {
        let ty = &field.ty;
        return if field.is_dynamic() {
            quote!(<#ty as ::photonic_dyn::dynamic::Dynamic>::Config)
        } else {
            quote!(#ty)
        }
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
        }
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

enum Style {
    Free,
    Bound,
}

pub fn expand_style(ident: &syn::Ident, generics: &syn::Generics, fields: &[FieldReceiver], style: Style) -> syn::Result<TokenStream> {
    let config = expand_config(ident, generics, fields)?;

    let base_decl_ty: syn::Type = match style {
        Style::Free => parse_quote!(::photonic::FreeAttrDecl),
        Style::Bound => parse_quote!(::photonic::BoundAttrDecl),
    };

    let factory_ty: syn::Type = match style {
        Style::Free => parse_quote!(::photonic_dyn::registry::FreeAttrFactory),
        Style::Bound => parse_quote!(::photonic_dyn::registry::BoundAttrFactory),
    };

    let generics = bounds::with_where_predicates(generics, [
        parse_quote!(Self: #base_decl_ty),
    ]);

    let generics = bounds::with_where_predicates_from_fields(&generics, &fields, field_bound);

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    return Ok(parse_quote! {
        impl #impl_generics ::photonic_dyn::DynamicAttr for #ident #type_generics #where_clause
        {
            const KIND: &'static str = <<#ident #type_generics as #base_decl_ty>::Attr as ::photonic::Node>::KIND;

            type Output<Builder> = #factory_ty <Builder>;

            fn factory<Builder>() -> Self::Output
                where Builder: ::photonic_dyn::builder::AttrBuilder,
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

pub fn expand(input: &mut syn::DeriveInput) -> syn::Result<TokenStream> {
    let receiver: StructReceiver = StructReceiver::from_derive_input(&input)?;

    let fields = receiver.data.take_struct().expect("Should never be an enum").fields;

    let free = receiver.free.is_present().then(|| expand_style(&receiver.ident, &receiver.generics, &fields, Style::Free)).transpose()?;
    let bound = receiver.bound.is_present().then(|| expand_style(&receiver.ident, &receiver.generics, &fields, Style::Bound)).transpose()?;

    return Ok(parse_quote!{
        #free
        #bound
    });
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(photonic), supports(struct_named), and_then = Self::check)]
struct StructReceiver {
    ident: syn::Ident,

    generics: syn::Generics,

    data: ast::Data<(), FieldReceiver>,

    free: Flag,
    bound: Flag,
}

impl StructReceiver {
    fn check(self) -> darling::Result<Self> {
        if !self.free.is_present() && !self.bound.is_present() {
            return Err(darling::Error::custom("Field must be at least marked as bound or free").with_span(&self.free.span()));
        }

        return Ok(self);
    }

    pub fn is_free(&self) -> bool {
        return self.free.is_present();
    }

    pub fn is_bound(&self) -> bool {
        return self.bound.is_present();
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(photonic), and_then = Self::check)]
struct FieldReceiver {
    ident: Option<syn::Ident>,

    ty: syn::Type,

    attr: Flag,
    input: Flag,
}

impl FieldReceiver {
    fn check(self) -> darling::Result<Self> {
        if self.attr.is_present() && self.input.is_present() {
            return Err(darling::Error::custom("Field can not be attr and input").with_span(&self.attr.span()));
        }

        return Ok(self);
    }

    pub fn is_dynamic(&self) -> bool {
        return self.attr.is_present() || self.input.is_present();
    }
}