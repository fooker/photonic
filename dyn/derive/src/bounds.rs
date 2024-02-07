pub trait StructReceiver {
    type FieldReceiver;

    fn fields(&self) -> &[Self::FieldReceiver];
}

pub fn with_where_predicates(generics: &syn::Generics,
                             predicates: impl IntoIterator<Item=syn::WherePredicate>) -> syn::Generics {
    let mut generics = generics.clone();
    generics.make_where_clause()
        .predicates
        .extend(predicates.into_iter());

    return generics;
}

pub fn with_where_predicates_from_fields<F>(generics: &syn::Generics,
                                            fields: &[F],
                                            mapper: impl Fn(&F) -> Vec<syn::WherePredicate>) -> syn::Generics {
    let predicates = fields.into_iter()
        .flat_map(mapper);

    return with_where_predicates(generics, predicates);
}