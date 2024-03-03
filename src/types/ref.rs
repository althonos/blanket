use syn::parse_quote;

use crate::derive::Receiver;
use crate::derive::WrapperType;

struct RefType;

impl WrapperType for RefType {
    const NAME: &'static str = "Ref";
    const RECEIVERS: &'static [Receiver] = &[Receiver::Ref];
    fn wrap(ty: &syn::Ident) -> syn::Type {
        parse_quote!(&#ty)
    }
}

pub fn derive(trait_: &syn::ItemTrait) -> syn::Result<syn::ItemImpl> {
    RefType::derive(trait_)
}

#[cfg(test)]
mod tests {
    mod derive {

        use syn::parse_quote;

        #[test]
        fn empty() {
            let trait_ = parse_quote!(
                trait Trait {}
            );
            assert_eq!(
                super::super::derive(&trait_).unwrap(),
                parse_quote!(
                    #[automatically_derived]
                    impl<T: Trait + ?Sized> Trait for &T {}
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
                    impl<T: Trait + ?Sized> Trait for &T {
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
                trait Trait {
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
                    impl<T, MT: MyTrait<T> + ?Sized> MyTrait<T> for &MT {}
                )
            );
        }

        #[test]
        fn generics_bounded() {
            let trait_ = parse_quote!(
                trait MyTrait<T: 'static + Send> {}
            );
            let derived = super::super::derive(&trait_).unwrap();

            assert_eq!(
                derived,
                parse_quote!(
                    #[automatically_derived]
                    impl<T: 'static + Send, MT: MyTrait<T> + ?Sized> MyTrait<T> for &MT {}
                )
            );
        }

        #[test]
        fn generics_lifetime() {
            let trait_ = parse_quote!(
                trait MyTrait<'a, 'b: 'a, T: 'static + Send> {}
            );
            let derived = super::super::derive(&trait_).unwrap();

            assert_eq!(
                derived,
                parse_quote!(
                    #[automatically_derived]
                    impl<'a, 'b: 'a, T: 'static + Send, MT: MyTrait<'a, 'b, T> + ?Sized>
                        MyTrait<'a, 'b, T> for &MT
                    {
                    }
                )
            );
        }

        #[test]
        fn associated_types() {
            let trait_ = parse_quote!(
                trait MyTrait {
                    type Return;
                }
            );
            let derived = super::super::derive(&trait_).unwrap();

            assert_eq!(
                derived,
                parse_quote!(
                    #[automatically_derived]
                    impl<MT: MyTrait + ?Sized> MyTrait for &MT {
                        type Return = <MT as MyTrait>::Return;
                    }
                )
            );
        }

        #[test]
        fn associated_function() {
            let trait_ = parse_quote!(
                trait MyTrait {
                    fn run();
                }
            );
            let derived = super::super::derive(&trait_).unwrap();

            assert_eq!(
                derived,
                parse_quote!(
                    #[automatically_derived]
                    impl<MT: MyTrait + ?Sized> MyTrait for &MT {
                        #[inline]
                        fn run() {
                            <MT as MyTrait>::run()
                        }
                    }
                )
            );
        }

        #[test]
        fn associated_function_return_type() {
            let trait_ = parse_quote!(
                trait MyTrait {
                    fn run() -> usize;
                }
            );
            let derived = super::super::derive(&trait_).unwrap();

            assert_eq!(
                derived,
                parse_quote!(
                    #[automatically_derived]
                    impl<MT: MyTrait + ?Sized> MyTrait for &MT {
                        #[inline]
                        fn run() -> usize {
                            <MT as MyTrait>::run()
                        }
                    }
                )
            );
        }

        #[test]
        fn associated_function_generics() {
            let trait_ = parse_quote!(
                trait MyTrait<T> {
                    fn run();
                }
            );
            let derived = super::super::derive(&trait_).unwrap();

            assert_eq!(
                derived,
                parse_quote!(
                    #[automatically_derived]
                    impl<T, MT: MyTrait<T> + ?Sized> MyTrait<T> for &MT {
                        #[inline]
                        fn run() {
                            <MT as MyTrait<T>>::run()
                        }
                    }
                )
            );
        }

        #[test]
        fn associated_types_bound() {
            let trait_ = parse_quote!(
                trait MyTrait {
                    type Return: Clone;
                }
            );
            let derived = super::super::derive(&trait_).unwrap();

            assert_eq!(
                derived,
                parse_quote!(
                    #[automatically_derived]
                    impl<MT: MyTrait + ?Sized> MyTrait for &MT {
                        type Return = <MT as MyTrait>::Return;
                    }
                )
            );
        }

        #[test]
        fn associated_types_dodgy_name() {
            let trait_ = parse_quote!(
                trait MyTrait {
                    type r#type;
                }
            );
            let derived = super::super::derive(&trait_).unwrap();

            assert_eq!(
                derived,
                parse_quote!(
                    #[automatically_derived]
                    impl<MT: MyTrait + ?Sized> MyTrait for &MT {
                        type r#type = <MT as MyTrait>::r#type;
                    }
                )
            );
        }

        #[test]
        fn associated_types_attrs() {
            let trait_ = parse_quote!(
                trait MyTrait {
                    #[cfg(target_arch = "wasm32")]
                    type Return;
                    #[cfg(not(target_arch = "wasm32"))]
                    type Return: Send;
                }
            );
            let derived = super::super::derive(&trait_).unwrap();

            assert_eq!(
                derived,
                parse_quote!(
                    #[automatically_derived]
                    impl<MT: MyTrait + ?Sized> MyTrait for &MT {
                        #[cfg(target_arch = "wasm32")]
                        type Return = <MT as MyTrait>::Return;
                        #[cfg(not(target_arch = "wasm32"))]
                        type Return = <MT as MyTrait>::Return;
                    }
                )
            );
        }

        #[test]
        fn associated_types_and_generics() {
            let trait_ = parse_quote!(
                trait MyTrait<T> {
                    type Return;
                }
            );
            let derived = super::super::derive(&trait_).unwrap();

            assert_eq!(
                derived,
                parse_quote!(
                    #[automatically_derived]
                    impl<T, MT: MyTrait<T> + ?Sized> MyTrait<T> for &MT {
                        type Return = <MT as MyTrait<T>>::Return;
                    }
                )
            );
        }

        #[test]
        fn associated_type_generics() {
            let trait_ = parse_quote!(
                trait MyTrait {
                    type Return<T>;
                }
            );
            let derived = super::super::derive(&trait_).unwrap();

            assert_eq!(
                derived,
                parse_quote!(
                    #[automatically_derived]
                    impl<MT: MyTrait + ?Sized> MyTrait for &MT {
                        type Return<T> = <MT as MyTrait>::Return<T>;
                    }
                )
            );
        }

        #[test]
        fn associated_type_generics_bounded() {
            let trait_ = parse_quote!(
                trait MyTrait {
                    type Return<T: 'static + Send>;
                }
            );
            let derived = super::super::derive(&trait_).unwrap();

            assert_eq!(
                derived,
                parse_quote!(
                    #[automatically_derived]
                    impl<MT: MyTrait + ?Sized> MyTrait for &MT {
                        type Return<T: 'static + Send> = <MT as MyTrait>::Return<T>;
                    }
                )
            );
        }

        #[test]
        fn associated_type_generics_lifetimes() {
            let trait_ = parse_quote!(
                trait MyTrait {
                    type Return<'a>
                    where
                        Self: 'a;
                }
            );
            let derived = super::super::derive(&trait_).unwrap();

            assert_eq!(
                derived,
                parse_quote!(
                    #[automatically_derived]
                    impl<MT: MyTrait + ?Sized> MyTrait for &MT {
                        type Return<'a> = <MT as MyTrait>::Return<'a>
                        where
                            Self: 'a;
                    }
                )
            );
        }
    }
}
