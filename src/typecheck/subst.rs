use super::*;
use env::Env;
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

    pub fn apply_env(&self, env: Env<Type>) -> Env<Type> {
        self.0.iter().fold(env, |acc, (ref name, ref typ)| {
            Env(acc
                .0
                .into_iter()
                .map(|(name_, typ_)| (name_, typ_.subst_type(name, typ)))
                .collect())
        })
    }

    pub fn compose(subst1: Subst, subst2: Subst) -> Subst {
        let subst1_ = Subst(
            subst1
                .0
                .iter()
                .map(|(name, typ)| (name.clone(), subst2.apply_type(typ.clone())))
                .collect(),
        );
        subst2.0.into_iter().fold(subst1_, |mut acc, (name, typ)| {
            if subst1.0.iter().any(|(name_, _)| name_ == &name) {
                acc
            } else {
                acc.0.insert(name, typ);
                acc
            }
        })
    }
}
