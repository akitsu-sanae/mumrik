use super::*;

impl Expr {
    pub fn subst_type(self, name: &Ident, typ: &Type) -> Expr {
        subst_type_expr(self, name, typ)
    }
}

fn subst_type_expr(e: Expr, name: &Ident, typ: &Type) -> Expr {
    match e {
        Expr::Const(lit) => Expr::Const(subst_type_literal(lit, name, typ)),
        Expr::Var(name_, typ_) => Expr::Var(name_, subst_type_type(typ_, name, typ)),
        Expr::Lambda(param_name, param_type, box body) => Expr::Lambda(
            param_name,
            subst_type_type(param_type, name, typ),
            box subst_type_expr(body, name, typ),
        ),
        Expr::Apply(box e1, box e2) => Expr::Apply(
            box subst_type_expr(e1, name, typ),
            box subst_type_expr(e2, name, typ),
        ),
        Expr::LetRec(name_, typ_, box e1, box e2) => Expr::LetRec(
            name_,
            subst_type_type(typ_, name, typ),
            box subst_type_expr(e1, name, typ),
            box subst_type_expr(e2, name, typ),
        ),
        Expr::Let(name_, box e1, box e2) => Expr::Let(
            name_,
            box subst_type_expr(e1, name, typ),
            box subst_type_expr(e2, name, typ),
        ),
        Expr::If(box cond, box e1, box e2) => Expr::If(
            box subst_type_expr(cond, name, typ),
            box subst_type_expr(e1, name, typ),
            box subst_type_expr(e2, name, typ),
        ),
        Expr::BinOp(op, box e1, box e2) => Expr::BinOp(
            op,
            box subst_type_expr(e1, name, typ),
            box subst_type_expr(e2, name, typ),
        ),
        Expr::FieldAccess(box e, label) => {
            Expr::FieldAccess(box subst_type_expr(e, name, typ), label)
        }
        Expr::PatternMatch(box e, arms) => Expr::PatternMatch(
            box subst_type_expr(e, name, typ),
            arms.into_iter()
                .map(|mut arm| {
                    arm.body = subst_type_expr(arm.body, name, typ);
                    arm
                })
                .collect(),
        ),
        Expr::Println(box e) => Expr::Println(box subst_type_expr(e, name, typ)),
    }
}

fn subst_type_literal(lit: Literal, name: &Ident, typ: &Type) -> Literal {
    match lit {
        Literal::Number(_) | Literal::Bool(_) | Literal::Char(_) | Literal::Unit => lit,
        Literal::Variant(label, box e, typ_) => Literal::Variant(
            label,
            box subst_type_expr(e, name, typ),
            subst_type_type(typ_, name, typ),
        ),
        Literal::Record(fields) => Literal::Record(
            fields
                .into_iter()
                .map(|(label, e)| (label, subst_type_expr(e, name, typ)))
                .collect(),
        ),
    }
}

fn subst_type_type(typ_: Type, name: &Ident, typ: &Type) -> Type {
    match typ_ {
        Type::Int | Type::Bool | Type::Char | Type::Unit => typ_,
        Type::Func(box typ1, box typ2) => Type::Func(
            box subst_type_type(typ1, name, typ),
            box subst_type_type(typ2, name, typ),
        ),
        Type::Record(fields) => Type::Record(
            fields
                .into_iter()
                .map(|(label, typ_)| (label, subst_type_type(typ_, name, typ)))
                .collect(),
        ),
        Type::Variant(ctors) => Type::Variant(
            ctors
                .into_iter()
                .map(|(label, typ_)| (label, subst_type_type(typ_, name, typ)))
                .collect(),
        ),
    }
}
