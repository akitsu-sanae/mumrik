use super::*;

impl Expr {
    pub fn subst_type(&self, name: &Ident, typ: &Type) -> Expr {
        subst_type_expr(&self, name, typ)
    }
}

fn subst_type_expr(e: &Expr, name: &Ident, typ: &Type) -> Expr {
    match e {
        Expr::Const(ref lit) => Expr::Const(subst_type_literal(lit, name, typ)),
        Expr::Lambda(ref param_name, ref param_type, box ref e, ref pos) => Expr::Lambda(
            param_name.clone(),
            subst_type_type(param_type, name, typ),
            box subst_type_expr(e, name, typ),
            pos.clone(),
        ),
        Expr::Apply(box ref e1, box ref e2) => Expr::Apply(
            box subst_type_expr(e1, name, typ),
            box subst_type_expr(e2, name, typ),
        ),
        Expr::Let(ref name_, box ref e1, box ref e2, ref pos) if name_ != name => Expr::Let(
            name_.clone(),
            box subst_type_expr(e1, name, typ),
            box subst_type_expr(e2, name, typ),
            pos.clone(),
        ),
        Expr::LetType(ref name_, ref typ_, box ref e, ref pos) if name_ != name => Expr::LetType(
            name_.clone(),
            subst_type_type(typ_, name, typ),
            box subst_type_expr(e, name, typ),
            pos.clone(),
        ),
        Expr::If(box ref cond, box ref e1, box ref e2, ref pos) => Expr::If(
            box subst_type_expr(cond, name, typ),
            box subst_type_expr(e1, name, typ),
            box subst_type_expr(e2, name, typ),
            pos.clone(),
        ),
        Expr::BinOp(ref op, box ref e1, box ref e2) => Expr::BinOp(
            *op,
            box subst_type_expr(e1, name, typ),
            box subst_type_expr(e2, name, typ),
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
                .map(|&(ref arm, ref pos)| {
                    (
                        PatternMatchArm {
                            label: arm.label.clone(),
                            name: arm.name.clone(),
                            body: subst_type_expr(&arm.body, name, typ),
                        },
                        pos.clone(),
                    )
                })
                .collect::<Vec<_>>(),
            pos.clone(),
        ),
        Expr::Println(box ref e, ref pos) => {
            Expr::Println(box subst_type_expr(e, name, typ), pos.clone())
        }
        Expr::Var(_, _) | Expr::Let(_, _, _, _) | Expr::LetType(_, _, _, _) => e.clone(),
    }
}

fn subst_type_literal(lit: &Literal, name: &Ident, typ: &Type) -> Literal {
    match lit {
        Literal::Number(_, _) | Literal::Bool(_, _) | Literal::Char(_, _) | Literal::Unit(_) => {
            lit.clone()
        }
        Literal::Variant(label, box e, typ_, pos) => Literal::Variant(
            label.clone(),
            box subst_type_expr(e, name, typ),
            subst_type_type(typ_, name, typ),
            pos.clone(),
        ),
        Literal::Record(fields, pos) => Literal::Record(
            fields
                .iter()
                .map(|&(ref label, ref e)| (label.clone(), subst_type_expr(e, name, typ)))
                .collect(),
            pos.clone(),
        ),
        Literal::Tuple(es, pos) => Literal::Tuple(
            es.iter()
                .map(|ref e| subst_type_expr(e, name, typ))
                .collect(),
            pos.clone(),
        ),
    }
}

fn subst_type_type(typ_: &Type, name: &Ident, typ: &Type) -> Type {
    match typ_ {
        Type::Var(ref name_, _) if name_ == name => typ.clone(),
        Type::Int(_) | Type::Bool(_) | Type::Char(_) | Type::Unit(_) | Type::Var(_, _) => {
            typ_.clone()
        }
        Type::Func(box ref typ1, box ref typ2, ref pos) => Type::Func(
            box subst_type_type(typ1, name, typ),
            box subst_type_type(typ2, name, typ),
            pos.clone(),
        ),
        Type::Record(ref fields, ref pos) => Type::Record(
            fields
                .iter()
                .map(|(label, typ_)| (label.clone(), subst_type_type(typ_, name, typ)))
                .collect(),
            pos.clone(),
        ),
        Type::Variant(ref ctors, ref pos) => Type::Variant(
            ctors
                .iter()
                .map(|(label, typ_)| (label.clone(), subst_type_type(typ_, name, typ)))
                .collect(),
            pos.clone(),
        ),
    }
}
