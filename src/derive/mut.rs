use syn::{parse_quote, spanned::Spanned};

use crate::utils::{
    deref_expr, generics_declaration_to_generics, signature_to_method_call, trait_to_generic_ident,
};

pub fn derive(trait_: &syn::ItemTrait) -> syn::Result<syn::ItemImpl> {
    // build an identifier for the generic type used for the implementation
    let trait_ident = &trait_.ident;
    let generic_type = trait_to_generic_ident(&trait_);

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
            if let Some(r) = m.sig.receiver() {
                let err = if r.colon_token.is_some() {
                    Some("cannot derive `Mut` for a trait declaring methods with arbitrary receiver types")
                } else if r.reference.is_none() {
                    Some("cannot derive `Mut` for a trait declaring `self` methods")
                } else {
                    None
                };
                if let Some(msg) = err {
                    return Err(syn::Error::new(r.span(), msg));
                }
            }

            let mut call = signature_to_method_call(&m.sig)?;
            call.receiver = Box::new(deref_expr(deref_expr(*call.receiver)));

            let signature = &m.sig;
            let item = parse_quote!(#[inline] #signature { #call });
            methods.push(item)
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
        impl #impl_generics #trait_ident #trait_generic_names for &mut #generic_type #where_clause {
            #(#assoc_types)*
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

        #[test]
        fn generics_bounded() {
            let trait_ = parse_quote!(
                trait Trait<T: 'static + Send> {}
            );
            let derived = super::super::derive(&trait_).unwrap();

            assert_eq!(
                derived,
                parse_quote!(
                    #[automatically_derived]
                    impl<T: 'static + Send, T_: Trait<T> + ?Sized> Trait<T> for &mut T_ {}
                )
            );
        }

        #[test]
        fn generics_lifetime() {
            let trait_ = parse_quote!(
                trait Trait<'a, 'b: 'a, T: 'static + Send> {}
            );
            let derived = super::super::derive(&trait_).unwrap();

            assert_eq!(
                derived,
                parse_quote!(
                    #[automatically_derived]
                    impl<'a, 'b: 'a, T: 'static + Send, T_: Trait<'a, 'b, T> + ?Sized>
                        Trait<'a, 'b, T> for &mut T_
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
                    impl<MT: MyTrait + ?Sized> MyTrait for &mut MT {
                        type Return = <MT as MyTrait>::Return;
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
                    impl<MT: MyTrait + ?Sized> MyTrait for &mut MT {
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
                    impl<MT: MyTrait + ?Sized> MyTrait for &mut MT {
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
                    impl<MT: MyTrait + ?Sized> MyTrait for &mut MT {
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
                    impl<T, MT: MyTrait<T> + ?Sized> MyTrait<T> for &mut MT {
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
                    impl<MT: MyTrait + ?Sized> MyTrait for &mut MT {
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
                    impl<MT: MyTrait + ?Sized> MyTrait for &mut MT {
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
                    impl<MT: MyTrait + ?Sized> MyTrait for &mut MT {
                        type Return<'a> = <MT as MyTrait>::Return<'a>
                        where
                            Self: 'a;
                    }
                )
            );
        }
    }
}
