mod r#box;
mod r#ref;

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
            _ => None,
        }
    }

    pub fn from_path(p: &syn::Path) -> Option<Self> {
        p.segments
            .first()
            .and_then(|s| Self::from_str(&s.ident.to_string()))
    }

    pub fn defer_trait_methods(&self, trait_: &syn::ItemTrait) -> syn::Result<syn::ItemImpl> {
        match self {
            Derive::Box => self::r#box::derive(trait_),
            Derive::Ref => self::r#ref::derive(trait_),
        }
    }
}
