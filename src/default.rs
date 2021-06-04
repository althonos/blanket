use syn::spanned::Spanned;

use super::utils::{prepend_function_path, signature_to_function_call};

/// Update the method declarations of `trait_` to use default implementation from `default` module.
pub fn defer_trait_methods(
    mut trait_: syn::ItemTrait,
    default: syn::Path,
) -> syn::Result<syn::ItemTrait> {
    for item in trait_.items.iter_mut() {
        if let syn::TraitItem::Method(ref mut m) = item {
            // check no default implementation was provided for the current
            // trait method
            if m.default.is_some() {
                let msg = "method should not have default implementation if using #[blanket(default = \"...\")]";
                return Err(syn::Error::new(m.span(), msg));
            }
            // update the declaration to include a default implementation
            // deferring the method call to a function call in the `default_mod`
            // module
            let mut call = signature_to_function_call(&m.sig)?;
            prepend_function_path(&mut call, default.clone())?;
            m.default = Some(syn::Block {
                brace_token: syn::token::Brace { span: call.span() },
                stmts: vec![syn::Stmt::Expr(syn::Expr::Call(call))],
            });
        }
    }
    Ok(trait_)
}
