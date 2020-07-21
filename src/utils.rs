use quote::quote_spanned;
use syn::parse_quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

/// Convert a function signature to a function call with the same arguments.
pub fn signature_to_function_call(sig: &syn::Signature) -> syn::Result<syn::ExprCall> {
    // Simply use the function ident as the function expression.
    let funcexpr = syn::ExprPath {
        attrs: Vec::new(),
        qself: None,
        path: sig.ident.clone().into()
    };

    // Extract arguments from the method signature names
    let mut funcargs = Punctuated::new();
    for item in &sig.inputs {
        match item {
            syn::FnArg::Receiver(recv) => {
                let span = recv.self_token.span;
                funcargs.push(syn::parse2(quote_spanned!(span=> self))?);
            }
            syn::FnArg::Typed(argty) => {
                if let syn::Pat::Ident(ref id) = *argty.pat {
                    let argpath = syn::ExprPath {
                        attrs: Vec::new(),
                        qself: None,
                        path: id.ident.clone().into()
                    };
                    funcargs.push(syn::Expr::Path(argpath));
                } else {
                    return Err(syn::Error::new(argty.span(), "expected identifier"));
                }
            }
        }
    }

    // Return the function call as an expression
    Ok(syn::ExprCall {
        attrs: Vec::new(),
        paren_token: syn::token::Paren { span: funcexpr.span() },
        func: Box::new(funcexpr.into()),
        args: funcargs,
    })
}

/// Convert a function signature to a method call with the same arguments.
pub fn signature_to_method_call(sig: &syn::Signature) -> syn::Result<syn::ExprMethodCall> {
    // Extract receiver
    let receiver = sig.receiver().unwrap();
    let span = receiver.span();

    // Extract arguments
    let mut funcargs = Punctuated::new();
    for item in &sig.inputs {
        match item {
            syn::FnArg::Receiver(_) => {}
            syn::FnArg::Typed(argty) => {
                if let syn::Pat::Ident(ref id) = *argty.pat {
                    let argpath = syn::ExprPath {
                        attrs: Vec::new(),
                        qself: None,
                        path: id.ident.clone().into()
                    };
                    funcargs.push(syn::Expr::Path(argpath));
                } else {
                    return Err(syn::Error::new(argty.span(), "expected identifier"));
                }
            }
        }
    }

    // Write the method call
    Ok(syn::ExprMethodCall {
        attrs: Vec::new(),
        receiver: Box::new(syn::parse2(quote_spanned!(span=> self))?),
        dot_token: syn::token::Dot { spans: [sig.span()] },
        method: sig.ident.clone(),
        turbofish: None,
        paren_token: syn::token::Paren { span: sig.span() },
        args: funcargs,
    })
}

/// Prepend a module path to a function call name.
pub fn prepend_function_path(call: &mut syn::ExprCall, module: syn::Path) -> syn::Result<()> {
    if let syn::Expr::Path(ref mut path) = *call.func {
        for (i, segment) in module.segments.into_iter().enumerate() {
            path.path.segments.insert(i, segment);
        }
        Ok(())
    } else {
        Err(syn::Error::new(call.func.span(), "expected path"))
    }
}

/// Deref an expression and wrap it in brackets to preserve operation priority.
pub fn deref_expr(expr: syn::Expr) -> syn::Expr {
    syn::Expr::Paren(syn::ExprParen {
        attrs: Vec::new(),
        paren_token: syn::token::Paren { span: expr.span() },
        expr: Box::new(
            syn::Expr::Unary(syn::ExprUnary {
                attrs: Vec::new(),
                op: syn::UnOp::Deref(parse_quote!(*)),
                expr: Box::new(expr)
            })
        )
    })
}

#[cfg(test)]
mod tests {

    use syn::parse_quote;

    #[test]
    fn prepend_function_path() {
        let path = parse_quote!(crate::qualified::path);
        let mut call = parse_quote!(myfunction(arg1, arg2));
        super::prepend_function_path(&mut call, path).unwrap();
        assert_eq!(
            call,
            parse_quote!(crate::qualified::path::myfunction(arg1, arg2))
        );
    }

    #[test]
    fn deref_expr() {
        let expr = parse_quote!(self);
        let dereffed = super::deref_expr(expr);
        assert_eq!(dereffed, parse_quote!((*self)));
    }
}
