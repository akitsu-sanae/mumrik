use super::*;
use ident::Ident;
use std::collections::HashMap;

impl Expr {
    pub fn free_term_vars(&self) -> HashMap<Ident, Type> {
        match self {
            Expr::Const(Literal::Record(ref fields)) => {
                let mut vars = HashMap::new();
                for (_, ref e) in fields.iter() {
                    vars.extend(e.free_term_vars());
                }
                vars
            }
            Expr::Const(Literal::Array(ref elems, _)) => {
                let mut vars = HashMap::new();
                for e in elems.iter() {
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
            Expr::Func {
                name,
                param_name,
                param_type,
                ret_type: _,
                box body,
                box left,
                pos: _,
            } => {
                let mut vars = body.free_term_vars();
                if param_name.is_omitted_param_name() {
                    if let Type::Record(ref fields) = param_type {
                        for (ref label, _) in fields.iter() {
                            vars.remove(label);
                        }
                    } else {
                        unreachable!()
                    }
                } else {
                    vars.remove(param_name);
                }
                vars.extend(left.free_term_vars());
                vars.remove(name);
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
            Expr::RecordGet(box ref e, _, _, _) => e.free_term_vars(),
            Expr::ArrayGet(box ref e1, box ref e2, _) => {
                let mut vars = HashMap::new();
                vars.extend(e1.free_term_vars());
                vars.extend(e2.free_term_vars());
                vars
            }
            Expr::Assign(box ref e1, box ref e2, _) => {
                let mut vars = HashMap::new();
                vars.extend(e1.free_term_vars());
                vars.extend(e2.free_term_vars());
                vars
            }
            Expr::Println(box ref e) => e.free_term_vars(),
            Expr::EmptyMark => HashMap::new(),
        }
    }
}
