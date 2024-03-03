use syn::parse_quote;

use crate::derive::Receiver;
use crate::derive::WrapperType;

struct CowType;

impl WrapperType for CowType {
    const NAME: &'static str = "Cow";
    const RECEIVERS: &'static [Receiver] = &[Receiver::Ref];
    const BOUNDS: &'static [&'static str] = &["ToOwned"];
    fn wrap(ty: &syn::Ident) -> syn::Type {
        parse_quote!(std::borrow::Cow<'_, #ty>)
    }
}

pub fn derive(trait_: &syn::ItemTrait) -> syn::Result<syn::ItemImpl> {
    CowType::derive(trait_)
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
                    impl<MT: MyTrait + ?Sized + ToOwned> MyTrait for std::borrow::Cow<'_, MT> {}
                )
            );
        }

        #[test]
        fn receiver_ref() {
            let trait_ = parse_quote!(
                trait Trait {
                    fn my_method(&self);
                }
            );
            assert_eq!(
                super::super::derive(&trait_).unwrap(),
                parse_quote!(
                    #[automatically_derived]
                    impl<T: Trait + ?Sized + ToOwned> Trait for std::borrow::Cow<'_, T> {
                        #[inline]
                        fn my_method(&self) {
                            (*(*self)).my_method()
                        }
                    }
                )
            );
        }

        #[test]
        fn receiver_mut() {
            let trait_ = parse_quote!(
                trait Trait {
                    fn my_method(&mut self);
                }
            );
            assert!(super::super::derive(&trait_).is_err());
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
                trait Trait {
                    fn my_method(self: Box<Self>);
                }
            );
            assert!(super::super::derive(&trait_).is_err());
        }

        #[test]
        fn generics() {
            let trait_ = parse_quote!(
                trait MyTrait<T> {}
            );
            let derived = super::super::derive(&trait_).unwrap();

            assert_eq!(
                derived,
                parse_quote!(
                    #[automatically_derived]
                    impl<T, MT: MyTrait<T> + ?Sized + ToOwned> MyTrait<T> for std::borrow::Cow<'_, MT> {}
                )
            );
        }
    }
}
