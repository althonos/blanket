#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Derive {
    Box,
    Ref,
}

impl Derive {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Box" => Some(Derive::Box),
            "Ref" => Some(Derive::Ref),
            _ => None
        }
    }

    pub fn from_path(p: &syn::Path) -> Option<Self> {
        p.segments
            .first()
            .and_then(|s| Self::from_str(&s.ident.to_string()) )
    }

    pub fn defer_trait_methods(&self, trait_: &syn::ItemTrait) -> syn::Result<syn::ItemImpl> {
        match self {
            Derive::Box => self::r#box::defer_trait_methods(trait_),
            Derive::Ref => self::r#ref::defer_trait_methods(trait_),
        }
    }
}


mod r#box {

    use syn::parse_quote;

    use crate::utils::deref_expr;
    use crate::utils::signature_to_method_call;

    pub fn defer_trait_methods(trait_: &syn::ItemTrait) -> syn::Result<syn::ItemImpl> {

        let name = &trait_.ident;
        let mut methods: Vec<syn::ImplItemMethod> = Vec::new();

        for item in trait_.items.iter() {
            if let syn::TraitItem::Method(ref m) = item {
                let signature = &m.sig;
                let mut call = signature_to_method_call(&m.sig)?;
                call.receiver = Box::new(deref_expr(deref_expr(*call.receiver)));
                let item = parse_quote!(#signature { #call });
                methods.push(item)
            }
        }

        Ok(parse_quote!(
            #[automatically_derived]
            impl<T: #name> #name for Box<T> {
                #(#methods)*
            }
        ))
    }
}


mod r#ref {

    use syn::parse_quote;
    use syn::spanned::Spanned;

    use crate::utils::deref_expr;
    use crate::utils::signature_to_method_call;

    pub fn defer_trait_methods(trait_: &syn::ItemTrait) -> syn::Result<syn::ItemImpl> {

        let name = &trait_.ident;
        let mut methods: Vec<syn::ImplItemMethod> = Vec::new();

        for item in trait_.items.iter() {
            if let syn::TraitItem::Method(ref m) = item {

                if let Some(receiver) = m.sig.receiver() {
                    match receiver {
                         syn::FnArg::Receiver(r) if r.mutability.is_some() => {
                             let msg = "cannot derive `Ref` for a trait declaring `&mut self` methods";
                             return Err(syn::Error::new(r.span(), msg));
                         }

                         syn::FnArg::Receiver(r) if r.reference.is_none() => {
                             let msg = "cannot derive `Ref` for a trait declaring `self` methods";
                             return Err(syn::Error::new(r.span(), msg));
                         }

                         syn::FnArg::Typed(pat) => {
                             let msg = "cannot derive `Ref` for a trait declaring methods with arbitrary receiver types";
                             return Err(syn::Error::new(pat.span(), msg));
                         }
                         _ => ()
                    }
                }


                let mut call = signature_to_method_call(&m.sig)?;
                call.receiver = Box::new(deref_expr(deref_expr(*call.receiver)));

                let signature = &m.sig;
                let item = parse_quote!(#signature { #call });
                methods.push(item)
            }
        }

        Ok(parse_quote!(
            #[automatically_derived]
            impl<T: #name + ?Sized> #name for &T {
                #(#methods)*
            }
        ))
    }
}
