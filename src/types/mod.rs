mod arc;
mod r#box;
mod r#mut;
mod rc;
mod r#ref;

// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Box,
    Ref,
    Mut,
    Rc,
    Arc,
}

impl Type {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Box" => Some(Type::Box),
            "Ref" => Some(Type::Ref),
            "Mut" => Some(Type::Mut),
            "Rc" => Some(Type::Rc),
            "Arc" => Some(Type::Arc),
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
            Type::Box => self::r#box::derive(trait_),
            Type::Ref => self::r#ref::derive(trait_),
            Type::Mut => self::r#mut::derive(trait_),
            Type::Rc => self::rc::derive(trait_),
            Type::Arc => self::arc::derive(trait_),
        }
    }
}
