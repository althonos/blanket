use syn::parse_quote;
use syn::spanned::Spanned;

use crate::utils::deref_expr;
use crate::utils::generics_declaration_to_generics;
use crate::utils::signature_to_associated_function_call;
use crate::utils::signature_to_method_call;
use crate::utils::trait_to_generic_ident;

/// Derive the delegate function for an `impl` block.
pub fn derive_impl_item_fn<F>(
    m: &syn::TraitItemFn,
    trait_ident: &syn::Ident,
    generic_type: &syn::Ident,
    trait_generic_names: &syn::Generics,
    check_receiver: F,
) -> syn::Result<syn::ImplItemFn>
where
    F: Fn(&syn::Receiver) -> syn::Result<()>,
{
    let mut call: syn::Expr = if let Some(r) = m.sig.receiver() {
        check_receiver(r)?;
        let mut call = signature_to_method_call(&m.sig)?;
        call.receiver = Box::new(deref_expr(deref_expr(*call.receiver)));
        call.into()
    } else {
        let call = signature_to_associated_function_call(
            &m.sig,
            &trait_ident,
            &generic_type,
            &trait_generic_names,
        )?;
        call.into()
    };

    if let Some(async_) = m.sig.asyncness {
        let span = async_.span();
        call = syn::ExprAwait {
            attrs: Vec::new(),
            base: Box::new(call),
            dot_token: syn::Token![.](span),
            await_token: syn::Token![await](span),
        }
        .into();
    }

    let signature = &m.sig;
    Ok(syn::parse_quote!(#[inline] #signature { #call }))
}

/// Derive the implementation for
pub fn derive_impl<F, G>(
    trait_: &syn::ItemTrait,
    check_receiver: F,
    generate_wrapper_type: G,
) -> syn::Result<syn::ItemImpl>
where
    F: Fn(&syn::Receiver) -> syn::Result<()>,
    G: Fn(&syn::Ident) -> syn::Type,
{
    // build an identifier for the generic type used for the implementation
    let trait_ident = &trait_.ident;
    let generic_type = trait_to_generic_ident(&trait_);
    let wrapper_type = generate_wrapper_type(&generic_type);

    // build the generics for the impl block:
    // we use the same generics as the trait itself, plus
    // a generic type that implements the trait for which we provide the
    // blanket implementation
    let trait_generics = &trait_.generics;
    let where_clause = &trait_.generics.where_clause;
    let mut impl_generics = trait_generics.clone();

    // we must however remove the generic type bounds, to avoid repeating them
    let mut trait_generic_names = trait_generics.clone();
    trait_generic_names.params = generics_declaration_to_generics(&trait_generics.params)?;

    impl_generics.params.push(syn::GenericParam::Type(
        parse_quote!(#generic_type: #trait_ident #trait_generic_names + ?Sized),
    ));

    // build the methods
    let mut methods: Vec<syn::ImplItemFn> = Vec::new();
    let mut assoc_types: Vec<syn::ImplItemType> = Vec::new();
    for item in trait_.items.iter() {
        if let syn::TraitItem::Fn(ref m) = item {
            methods.push(derive_impl_item_fn(
                m,
                &trait_ident,
                &generic_type,
                &trait_generic_names,
                &check_receiver,
            )?)
        }

        if let syn::TraitItem::Type(t) = item {
            let t_ident = &t.ident;
            let attrs = &t.attrs;

            let t_generics = &t.generics;
            let where_clause = &t.generics.where_clause;
            let mut t_generic_names = t_generics.clone();
            t_generic_names.params = generics_declaration_to_generics(&t_generics.params)?;

            let item = parse_quote!( #(#attrs)* type #t_ident #t_generics = <#generic_type as #trait_ident #trait_generic_names>::#t_ident #t_generic_names #where_clause ; );
            assoc_types.push(item);
        }
    }

    Ok(parse_quote!(
        #[automatically_derived]
        impl #impl_generics #trait_ident #trait_generic_names for #wrapper_type #where_clause {
            #(#assoc_types)*
            #(#methods)*
        }
    ))
}
