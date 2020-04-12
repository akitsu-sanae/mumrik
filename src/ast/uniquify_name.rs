use ast::*;
use ident::Ident;
use std::collections::HashSet;

impl Expr {
    pub fn uniquify_variable(self: Expr) -> Expr {
        uniquify_expr(self)
    }
}

fn uniquify_expr(e: Expr) -> Expr {
    use ast::Expr::*;
    match e {
        Const(lit) => Const(uniquify_literal(lit, variables)),
        Var(_) => e,
    }
}

fn uniquify_literal(lit: Literal) -> Literal {
    use ast::Literal::*;
    match lit {
        Number(_) | Bool(_) | Char(_) | Unit => lit,
        Func {
            param_name,
            param_type,
            ret_type,
            box body,
            pos,
        } => {
            let fresh_ident = Ident::fresh("var");
            let body = body.subst_expr(&param_name);
        }
        Record(fields) => Record(
            fields
                .into_iter()
                .map(|(label, box e)| (label, box uniquify_expr(e)))
                .collect(),
        ),
    }
}
