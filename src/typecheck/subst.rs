use super::*;
use env::Env;
use ident::Ident;
use std::collections::HashMap;

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

    pub fn apply_env(&self, _env: Env<Type>) -> Env<Type> {
        todo!()
        /*
        self.0.iter().fold(env, |acc, &(ref name, ref typ)| {
            let env = env.bindings.into_iter().map().collect();
        }) */
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
