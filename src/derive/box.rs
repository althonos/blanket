use syn::parse_quote;

use crate::utils::deref_expr;
use crate::utils::signature_to_method_call;

pub fn derive(trait_: &syn::ItemTrait) -> syn::Result<syn::ItemImpl> {

    let name = &trait_.ident;
    let mut methods: Vec<syn::ImplItemMethod> = Vec::new();

    for item in trait_.items.iter() {
        if let syn::TraitItem::Method(ref m) = item {
            let signature = &m.sig;
            let mut call = signature_to_method_call(&m.sig)?;
            call.receiver = Box::new(deref_expr(deref_expr(*call.receiver)));
            let item = parse_quote!(#[inline] #signature { #call });
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