use super::*;
use ident::Ident;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Subst(pub HashMap<Ident, Type>);

impl Subst {
    pub fn new() -> Subst {
        Subst(HashMap::new())
    }

    pub fn apply_type(&self, ty: Type) -> Type {
        self.0
            .iter()
            .fold(ty, |acc, (ref name, ref typ)| acc.subst_type(name, typ))
    }

    pub fn apply_expr(&self, e: Expr) -> Expr {
        self.0
            .iter()
            .fold(e, |acc, (ref name, ref typ)| acc.subst_type(name, typ))
    }
}
