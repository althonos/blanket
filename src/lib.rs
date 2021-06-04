#![cfg_attr(feature = "_doc", feature(doc_cfg, external_doc))]
#![cfg_attr(feature = "_doc", doc(include = "../README.md"))]

extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;

use std::collections::HashSet;

use quote::{quote, ToTokens};
use syn::{parse_macro_input, spanned::Spanned};

// ---------------------------------------------------------------------------

mod default;
mod derive;
mod utils;

// ---------------------------------------------------------------------------

struct Args {
    default: Option<syn::Path>,
    derives: HashSet<derive::Derive>,
}

impl Args {
    fn from_args(args: &syn::AttributeArgs) -> syn::Result<Self> {
        let mut default = None;
        let mut derives = HashSet::new();

        let meta = args
            .iter()
            .map(|arg| match arg {
                syn::NestedMeta::Lit(lit) => Err(syn::Error::new(lit.span(), "unexpected literal")),
                syn::NestedMeta::Meta(meta) => Ok(meta),
            })
            .collect::<syn::Result<Vec<&syn::Meta>>>()?;

        for arg in meta {
            // argument paths are compared against their token stream serialization
            // to avoid to compile `syn` with the `extra-traits` feature
            match arg {
                syn::Meta::List(ref l) if l.path.to_token_stream().to_string() == "derive" => {
                    for elem in l.nested.iter() {
                        if let syn::NestedMeta::Meta(syn::Meta::Path(path)) = elem {
                            if let Some(d) = derive::Derive::from_path(&path) {
                                derives.insert(d);
                            } else {
                                return Err(syn::Error::new(
                                    path.span(),
                                    "unknown blanket derive option",
                                ));
                            }
                        } else {
                            return Err(syn::Error::new(elem.span(), "expected identifier"));
                        }
                    }
                }
                syn::Meta::NameValue(ref n)
                    if n.path.to_token_stream().to_string() == "default" =>
                {
                    if let syn::Lit::Str(ref s) = n.lit {
                        match syn::parse_str(&s.value()) {
                            Ok(path) if default.is_none() => {
                                default = Some(path);
                            }
                            Ok(_) => {
                                return Err(syn::Error::new(
                                    s.span(),
                                    "duplicate default module given",
                                ))
                            }
                            Err(_) => {
                                return Err(syn::Error::new(s.span(), "expected module identifier"))
                            }
                        }
                    } else {
                        return Err(syn::Error::new(n.lit.span(), "expected string literal"));
                    }
                }
                _ => return Err(syn::Error::new(arg.span(), "unexpected argument")),
            }
        }

        Ok(Self { default, derives })
    }
}

// ---------------------------------------------------------------------------

#[proc_macro_attribute]
pub fn blanket(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // parse input
    let trait_ = parse_macro_input!(input as syn::ItemTrait);
    let attribute_args = parse_macro_input!(args as syn::AttributeArgs);
    // parse macro arguments and immediately exit if they are invalid
    let args = match Args::from_args(&attribute_args) {
        Ok(args) => args,
        Err(e) => {
            let err = e.to_compile_error();
            return proc_macro::TokenStream::from(quote!(#err #trait_));
        }
    };
    // generate output
    let mut out = proc_macro2::TokenStream::new();
    // update trait methods declaration if given a `default = "..."` argument,
    // otherwise simply keep the output
    match args.default {
        None => out.extend(quote!(#trait_)),
        Some(d) => match default::defer_trait_methods(trait_.clone(), d) {
            Ok(trait_) => out.extend(quote!(#trait_)),
            Err(err) => out.extend(err.to_compile_error()),
        },
    };
    // add derived implementations
    for d in args.derives {
        match d.defer_trait_methods(&trait_) {
            Ok(item) => out.extend(quote!(#item)),
            Err(e) => out.extend(e.to_compile_error()),
        }
    }
    // return the new `proc-macro2` token stream as a `proc-macro` stream
    proc_macro::TokenStream::from(out)
}
