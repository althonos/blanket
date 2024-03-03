use syn::parse_quote;
use syn::spanned::Spanned;

use crate::utils::deref_expr;
use crate::utils::generics_declaration_to_generics;
use crate::utils::signature_to_associated_function_call;
use crate::utils::signature_to_method_call;
use crate::utils::trait_to_generic_ident;

/// The different receivers supported on a method.
#[derive(Debug, PartialEq)]
pub enum Receiver {
    Arbitrary,
    Ref,
    Mut,
    Owned,
}

/// A marker trait for types wrapping a single other type.
pub trait WrapperType {
    /// A short name for the type being wrapper.
    const NAME: &'static str;

    /// The receivers allowed for this wrapper type.
    const RECEIVERS: &'static [Receiver];

    /// Wrap the given identifier into the wrapper type.
    fn wrap(ty: &syn::Ident) -> syn::Type;

    /// Check that the given receiver is supported for the wrapper type.
    fn check_receiver(r: &syn::Receiver) -> syn::Result<()> {
        let receivers = Self::RECEIVERS;
        let err = if r.colon_token.is_some() && !receivers.contains(&Receiver::Arbitrary) {
            Some(format!(
                "cannot derive `{}` for a trait declaring methods with arbitrary receiver types",
                Self::NAME
            ))
        } else if r.mutability.is_some() && !receivers.contains(&Receiver::Mut) {
            Some(format!(
                "cannot derive `{}` for a trait declaring `&mut self` methods",
                Self::NAME
            ))
        } else if r.reference.is_none() && !receivers.contains(&Receiver::Owned) {
            Some(format!(
                "cannot derive `{}` for a trait declaring `self` methods",
                Self::NAME
            ))
        } else {
            None
        };
        if let Some(msg) = err {
            Err(syn::Error::new(r.span(), msg))
        } else {
            Ok(())
        }
    }

    /// Generate the derived implementation for the given trait.
    fn derive(trait_: &syn::ItemTrait) -> syn::Result<syn::ItemImpl> {
        // build an identifier for the generic type used for the implementation
        let trait_ident = &trait_.ident;
        let generic_type = trait_to_generic_ident(&trait_);
        let wrapper_type = Self::wrap(&generic_type);

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

        // build the methods
        let mut methods: Vec<syn::ImplItemFn> = Vec::new();
        let mut assoc_types: Vec<syn::ImplItemType> = Vec::new();
        for item in trait_.items.iter() {
            if let syn::TraitItem::Fn(ref m) = item {
                methods.push(Self::derive_method(
                    m,
                    &trait_ident,
                    &generic_type,
                    &trait_generic_names,
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

        // check if any method has a `Self` receiver, which would mean we cannot
        // relax the `Sized` trait requirement
        let mut sized = false;
        for item in trait_.items.iter() {
            if let syn::TraitItem::Fn(ref m) = item {
                if let Some(r) = m.sig.receiver() {
                    sized |= r.reference.is_none();
                }
            }
        }

        // Add generic type for the type we are creating ourselves
        if sized {
            impl_generics.params.push(syn::GenericParam::Type(
                parse_quote!(#generic_type: #trait_ident #trait_generic_names),
            ));
        } else {
            impl_generics.params.push(syn::GenericParam::Type(
                parse_quote!(#generic_type: #trait_ident #trait_generic_names + ?Sized),
            ));
        }

        Ok(parse_quote!(
            #[automatically_derived]
            impl #impl_generics #trait_ident #trait_generic_names for #wrapper_type #where_clause {
                #(#assoc_types)*
                #(#methods)*
            }
        ))
    }

    /// Generate the derived implementation for a single method of a trait.
    fn derive_method(
        m: &syn::TraitItemFn,
        trait_ident: &syn::Ident,
        generic_type: &syn::Ident,
        trait_generic_names: &syn::Generics,
    ) -> syn::Result<syn::ImplItemFn> {
        let mut call: syn::Expr = if let Some(r) = m.sig.receiver() {
            Self::check_receiver(r)?;
            let mut call = signature_to_method_call(&m.sig)?;
            if r.reference.is_some() {
                call.receiver = Box::new(deref_expr(deref_expr(*call.receiver)));
            } else {
                call.receiver = Box::new(deref_expr(*call.receiver));
            }
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
}
