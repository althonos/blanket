use syn::parse_quote;
use syn::spanned::Spanned;

use crate::utils::deref_expr;
use crate::utils::signature_to_method_call;
use crate::utils::trait_to_generic_ident;

pub fn derive(trait_: &syn::ItemTrait) -> syn::Result<syn::ItemImpl> {
    // build the methods
    let mut methods: Vec<syn::ImplItemMethod> = Vec::new();
    for item in trait_.items.iter() {
        if let syn::TraitItem::Method(ref m) = item {
            if let Some(receiver) = m.sig.receiver() {
                match receiver {
                    syn::FnArg::Receiver(r) if r.reference.is_none() => {
                        let msg = "cannot derive `Mut` for a trait declaring `self` methods";
                        return Err(syn::Error::new(r.span(), msg));
                    }
                    syn::FnArg::Typed(pat) => {
                        let msg = "cannot derive `Mut` for a trait declaring methods with arbitrary receiver types";
                        return Err(syn::Error::new(pat.span(), msg));
                    }
                    _ => (),
                }
            }

            let mut call = signature_to_method_call(&m.sig)?;
            call.receiver = Box::new(deref_expr(deref_expr(*call.receiver)));

            let signature = &m.sig;
            let item = parse_quote!(#[inline] #signature { #call });
            methods.push(item)
        }
    }

    // build an identifier for the generic type used for the implementation
    let trait_ident = &trait_.ident;
    let generic_type = trait_to_generic_ident(&trait_);

    // build the generics for the impl block:
    // we use the same generics as the trait itself, plus
    // a generic type that implements the trait for which we provide the
    // blanket implementation
    let trait_generics = &trait_.generics;
    let mut impl_generics = trait_generics.clone();
    impl_generics.params.push(syn::GenericParam::Type(
        parse_quote!(#generic_type: #trait_ident #trait_generics + ?Sized),
    ));

    Ok(parse_quote!(
        #[automatically_derived]
        impl #impl_generics #trait_ident #trait_generics for &mut #generic_type {
            #(#methods)*
        }
    ))
}

#[cfg(test)]
mod tests {
    mod derive {

        use syn::parse_quote;

        #[test]
        fn empty() {
            let trait_ = parse_quote!(
                trait MyTrait {}
            );
            let derived = super::super::derive(&trait_).unwrap();
            assert_eq!(
                derived,
                parse_quote!(
                    #[automatically_derived]
                    impl<MT: MyTrait + ?Sized> MyTrait for &mut MT {}
                )
            );
        }

        #[test]
        fn receiver_mut() {
            let trait_ = parse_quote!(
                trait MyTrait {
                    fn my_method(&mut self);
                }
            );
            assert_eq!(
                super::super::derive(&trait_).unwrap(),
                parse_quote!(
                    #[automatically_derived]
                    impl<MT: MyTrait + ?Sized> MyTrait for &mut MT {
                        #[inline]
                        fn my_method(&mut self) {
                            (*(*self)).my_method()
                        }
                    }
                )
            );
        }

        #[test]
        fn receiver_self() {
            let trait_ = parse_quote!(
                trait MyTrait {
                    fn my_method(self);
                }
            );
            assert!(super::super::derive(&trait_).is_err());
        }

        #[test]
        fn receiver_arbitrary() {
            let trait_ = parse_quote!(
                trait MyTrait {
                    fn my_method(self: Box<Self>);
                }
            );
            assert!(super::super::derive(&trait_).is_err());
        }

        #[test]
        fn generics() {
            let trait_ = parse_quote!(
                trait Trait<T> {}
            );
            let derived = super::super::derive(&trait_).unwrap();

            assert_eq!(
                derived,
                parse_quote!(
                    #[automatically_derived]
                    impl<T, T_: Trait<T> + ?Sized> Trait<T> for &mut T_ {}
                )
            );
        }
    }
}
