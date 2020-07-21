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
            _ => unimplemented!(),
        }
    }
}


mod r#box {

    use syn::parse_quote;

    pub fn defer_trait_methods(trait_: &syn::ItemTrait) -> syn::Result<syn::ItemImpl> {

        let name = &trait_.ident;
        let mut methods: Vec<syn::ImplItemMethod> = Vec::new();

        for item in trait_.items.iter() {
            if let syn::TraitItem::Method(ref m) = item {
                let signature = &m.sig;
                let call = crate::utils::signature_to_method_call(&m.sig)?;
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
