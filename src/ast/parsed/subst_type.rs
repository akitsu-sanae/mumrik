use super::*;

impl Expr {
    pub fn subst_type(&self, name: &Ident, typ: &Type) -> Expr {
        subst_type_expr(&self, name, typ)
    }
}

fn subst_type_expr(e: &Expr, name: &Ident, typ: &Type) -> Expr {
    match e {
        Expr::Const(ref lit) => Expr::Const(subst_type_literal(lit, name, typ)),
        Expr::Lambda(ref param_name, ref param_type, box ref e) => Expr::Lambda(
            param_name.clone(),
            subst_type_type(param_type, name, typ),
            box subst_type_expr(e, name, typ),
        ),
        Expr::Apply(box ref e1, box ref e2, ref pos) => Expr::Apply(
            box subst_type_expr(e1, name, typ),
            box subst_type_expr(e2, name, typ),
            pos.clone(),
        ),
        Expr::Let(ref name_, box ref e1, box ref e2) if name_ != name => Expr::Let(
            name_.clone(),
            box subst_type_expr(e1, name, typ),
            box subst_type_expr(e2, name, typ),
        ),
        Expr::LetType(ref name_, ref typ_, box ref e) if name_ != name => Expr::LetType(
            name_.clone(),
            subst_type_type(typ_, name, typ),
            box subst_type_expr(e, name, typ),
        ),
        Expr::If(box ref cond, box ref e1, box ref e2, ref pos) => Expr::If(
            box subst_type_expr(cond, name, typ),
            box subst_type_expr(e1, name, typ),
            box subst_type_expr(e2, name, typ),
            pos.clone(),
        ),
        Expr::BinOp(ref op, box ref e1, box ref e2, ref pos) => Expr::BinOp(
            *op,
            box subst_type_expr(e1, name, typ),
            box subst_type_expr(e2, name, typ),
            pos.clone(),
        ),
        Expr::Sequence(ref es) => {
            Expr::Sequence(es.iter().map(|e| subst_type_expr(e, name, typ)).collect())
        }
        Expr::FieldAccess(box ref e, ref label, ref pos) => Expr::FieldAccess(
            box subst_type_expr(e, name, typ),
            label.clone(),
            pos.clone(),
        ),
        Expr::PatternMatch(box ref e, ref arms, ref pos) => Expr::PatternMatch(
            box subst_type_expr(e, name, typ),
            arms.iter()
                .map(|ref arm| PatternMatchArm {
                    label: arm.label.clone(),
                    name: arm.name.clone(),
                    body: subst_type_expr(&arm.body, name, typ),
                })
                .collect(),
            pos.clone(),
        ),
        Expr::Println(box ref e) => Expr::Println(box subst_type_expr(e, name, typ)),
        Expr::Var(_, _) | Expr::Let(_, _, _) | Expr::LetType(_, _, _) => e.clone(),
    }
}

fn subst_type_literal(lit: &Literal, name: &Ident, typ: &Type) -> Literal {
    match lit {
        Literal::Number(_) | Literal::Bool(_) | Literal::Char(_) | Literal::Unit => lit.clone(),
        Literal::Variant(label, box e, typ_, pos) => Literal::Variant(
            label.clone(),
            box subst_type_expr(e, name, typ),
            subst_type_type(typ_, name, typ),
            pos.clone(),
        ),
        Literal::Record(fields) => Literal::Record(
            fields
                .iter()
                .map(|&(ref label, ref e)| (label.clone(), subst_type_expr(e, name, typ)))
                .collect(),
        ),
        Literal::Tuple(es) => Literal::Tuple(
            es.iter()
                .map(|ref e| subst_type_expr(e, name, typ))
                .collect(),
        ),
    }
}

fn subst_type_type(typ_: &Type, name: &Ident, typ: &Type) -> Type {
    match typ_ {
        Type::Var(ref name_, _) if name_ == name => typ.clone(),
        Type::Int | Type::Bool | Type::Char | Type::Unit | Type::Var(_, _) => typ_.clone(),
        Type::Func(box ref typ1, box ref typ2) => Type::Func(
            box subst_type_type(typ1, name, typ),
            box subst_type_type(typ2, name, typ),
        ),
        Type::Record(ref fields) => Type::Record(
            fields
                .iter()
                .map(|(label, typ_)| (label.clone(), subst_type_type(typ_, name, typ)))
                .collect(),
        ),
        Type::Variant(ref ctors) => Type::Variant(
            ctors
                .iter()
                .map(|(label, typ_)| (label.clone(), subst_type_type(typ_, name, typ)))
                .collect(),
        ),
    }
}
