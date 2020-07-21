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
        impl<B: #name + ?Sized> #name for &B {
            #(#methods)*
        }
    ))
}
