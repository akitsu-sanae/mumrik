use super::*;

impl Expr {
    pub fn subst_type(self, name: &Ident, typ: &Type) -> Expr {
        subst_type_expr(self, name, typ)
    }
}

fn subst_type_expr(e: Expr, name: &Ident, typ: &Type) -> Expr {
    match e {
        Expr::Const(lit) => subst_type_literal(lit, name, typ),
        Expr::Var(name, pos) => Expr::Var(name, pos),
    }
}
