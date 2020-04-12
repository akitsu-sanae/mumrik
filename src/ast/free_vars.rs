use super::*;
use ident::Ident;
use std::collections::HashMap;

impl Expr {
    pub fn free_term_vars(&self) -> HashMap<Ident, Type> {
        match self {
            Expr::Const(Literal::Func {
                ref param_name,
                param_type: _,
                ret_type: _,
                box ref body,
                pos: _,
            }) => {
                let mut vars = body.free_term_vars();
                vars.remove(param_name);
                vars
            }
            Expr::Const(Literal::Record(ref fields)) => {
                let mut vars = HashMap::new();
                for (_, ref e) in fields.iter() {
                    vars.extend(e.free_term_vars());
                }
                vars
            }
            Expr::Const(_) => HashMap::new(),
            Expr::Var(ref name, ref typ, _) => {
                let mut vars = HashMap::new();
                vars.insert(name.clone(), typ.clone());
                vars
            }
            Expr::Apply(box ref e1, box ref e2, _) => {
                let mut vars = HashMap::new();
                vars.extend(e1.free_term_vars());
                vars.extend(e2.free_term_vars());
                vars
            }
            Expr::Let(ref name, _, box ref e1, box ref e2, _) => {
                let mut vars = HashMap::new();
                vars.extend(e2.free_term_vars());
                vars.remove(name);
                vars.extend(e1.free_term_vars());
                vars
            }
            Expr::LetRec(ref name, _, box ref e1, box ref e2, _) => {
                let mut vars = HashMap::new();
                vars.extend(e1.free_term_vars());
                vars.extend(e2.free_term_vars());
                vars.remove(name);
                vars
            }
            Expr::LetType(_, _, box ref e) => e.free_term_vars(),
            Expr::If(box ref cond, box ref e1, box ref e2, _) => {
                let mut vars = HashMap::new();
                vars.extend(cond.free_term_vars());
                vars.extend(e1.free_term_vars());
                vars.extend(e2.free_term_vars());
                vars
            }
            Expr::BinOp(_, box ref e1, box ref e2, _) => {
                let mut vars = HashMap::new();
                vars.extend(e1.free_term_vars());
                vars.extend(e2.free_term_vars());
                vars
            }
            Expr::FieldAccess(box ref e, _, _) => e.free_term_vars(),
            Expr::Println(box ref e) => e.free_term_vars(),
        }
    }
}
