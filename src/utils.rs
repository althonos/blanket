use quote::quote_spanned;
use syn::{parse_quote, punctuated::Punctuated, spanned::Spanned, GenericParam, Token};

/// Convert a function signature to a function call with the same arguments.
pub fn signature_to_function_call(sig: &syn::Signature) -> syn::Result<syn::ExprCall> {
    // Simply use the function ident as the function expression.
    let funcexpr = syn::ExprPath {
        attrs: Vec::new(),
        qself: None,
        path: sig.ident.clone().into(),
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
                        path: id.ident.clone().into(),
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
        paren_token: syn::token::Paren {
            span: funcexpr.span(),
        },
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
                        path: id.ident.clone().into(),
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
        dot_token: syn::token::Dot {
            spans: [sig.span()],
        },
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
        expr: Box::new(syn::Expr::Unary(syn::ExprUnary {
            attrs: Vec::new(),
            op: syn::UnOp::Deref(parse_quote!(*)),
            expr: Box::new(expr),
        })),
    })
}

/// Build a generic identifier suitable for the given trait.
///
/// This function extracts the initials of the trait identifier. If this results
/// in a generic type identifier already present in the generics of that trait,
/// as many underscores are added to the end of the identifier.
pub fn trait_to_generic_ident(trait_: &syn::ItemTrait) -> syn::Ident {
    let mut raw = trait_
        .ident
        .to_string()
        .chars()
        .filter(|c| c.is_uppercase())
        .collect::<String>();
    loop {
        if !trait_.generics.params.iter().any(|g| match g {
            syn::GenericParam::Type(param) if param.ident == raw => true,
            syn::GenericParam::Const(param) if param.ident == raw => true,
            _ => false,
        }) {
            break;
        } else {
            raw.push('_');
        }
    }

    syn::Ident::new(&raw, trait_.ident.span())
}

/// Convert a generic type declaration to a generic with the same arguments.
///
/// Given a generic section `<T: 'static + Send>`, get simply `<T>`.
pub fn generics_declaration_to_generics(
    generics: &Punctuated<GenericParam, Token![,]>,
) -> syn::Result<Punctuated<GenericParam, Token![,]>> {
    generics
        .iter()
        .map(|gen| match gen {
            syn::GenericParam::Type(t) => Ok(syn::GenericParam::Type(syn::TypeParam {
                attrs: t.attrs.clone(),
                ident: t.ident.clone(),
                colon_token: None,
                bounds: Punctuated::new(),
                eq_token: None,
                default: None,
            })),
            syn::GenericParam::Lifetime(l) => Ok(syn::GenericParam::Lifetime(syn::LifetimeDef {
                attrs: l.attrs.clone(),
                lifetime: l.lifetime.clone(),
                colon_token: None,
                bounds: Punctuated::new(),
            })),
            syn::GenericParam::Const(c) => {
                Err(syn::Error::new(c.span(), "cannot handle const generics"))
            }
        })
        .collect()
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

    #[test]
    fn trait_to_generic_ident() {
        let trait_ = syn::parse_quote!(
            trait Trait {}
        );
        let expected: syn::Ident = syn::parse_quote!(T);
        assert_eq!(super::trait_to_generic_ident(&trait_), expected);

        let trait_ = syn::parse_quote!(
            trait SomeTrait {}
        );
        let expected: syn::Ident = syn::parse_quote!(ST);
        assert_eq!(super::trait_to_generic_ident(&trait_), expected);

        let trait_ = syn::parse_quote!(
            trait Trait<T> {}
        );
        let expected: syn::Ident = syn::parse_quote!(T_);
        assert_eq!(super::trait_to_generic_ident(&trait_), expected);
    }
}
