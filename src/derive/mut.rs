use syn::parse_quote;
use syn::spanned::Spanned;

use crate::utils::deref_expr;
use crate::utils::signature_to_method_call;

pub fn derive(trait_: &syn::ItemTrait) -> syn::Result<syn::ItemImpl> {
    let name = &trait_.ident;
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

    Ok(parse_quote!(
        #[automatically_derived]
        impl<B: #name + ?Sized> #name for &mut B {
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
                    impl<B: MyTrait + ?Sized> MyTrait for &mut B {}
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
                    impl<B: MyTrait + ?Sized> MyTrait for &mut B {
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
    }
}
