use super::*;

impl Expr {
    pub fn subst_type(self, name: &Ident, typ: &Type) -> Expr {
        aux_expr(
            self,
            name,
            typ,
            |e, name, _| match &e {
                Expr::LetType(ref name_, _, _) if name_ == name => Some(e),
                _ => None,
            },
            |_, _, _| None,
            |typ_, name, typ| match typ_ {
                Type::Var(name_) if name == &name_ => Some(typ.clone()),
                _ => None,
            },
        )
    }
    pub fn subst_expr(self, name: &Ident, expr: &Expr) -> Expr {
        aux_expr(
            self,
            name,
            expr,
            |e, name, expr| match &e {
                Expr::Var(ref name_, _, _) if name == name_ => Some(expr.clone()),
                Expr::Let(ref name_, _, _, _, _) if name == name_ => Some(e),
                Expr::Func {
                    name: ref func_name,
                    ref param_name,
                    ref param_type,
                    ..
                } => {
                    let same_as_func_name = name == func_name;
                    let same_as_param_name = if param_name.is_omitted_param_name() {
                        if let Type::Record(ref fields) = param_type {
                            fields.iter().any(|(ref label, _)| &name == label)
                        } else {
                            unreachable!()
                        }
                    } else {
                        name == param_name
                    };
                    if same_as_func_name || same_as_param_name {
                        Some(e)
                    } else {
                        None
                    }
                }
                _ => None,
            },
            |_, _, _| None,
            |_, _, _| None,
        )
    }
}

impl Type {
    pub fn subst_type(self, name: &Ident, typ: &Type) -> Type {
        aux_type(
            self,
            name,
            typ,
            |_, _, _| None,
            |_, _, _| None,
            |typ_, name, typ| match typ_ {
                Type::Var(name_) if name == &name_ => Some(typ.clone()),
                _ => None,
            },
        )
    }
}

fn aux_expr<T>(
    e: Expr,
    name: &Ident,
    v: &T,
    ef: fn(Expr, &Ident, &T) -> Option<Expr>,
    lf: fn(Literal, &Ident, &T) -> Option<Literal>,
    tf: fn(Type, &Ident, &T) -> Option<Type>,
) -> Expr {
    if let Some(e) = ef(e.clone(), name, v) {
        return e;
    }
    match e {
        Expr::Const(lit) => Expr::Const(aux_literal(lit, name, v, ef, lf, tf)),
        Expr::Var(name_, typ, pos) => Expr::Var(name_, aux_type(typ, name, v, ef, lf, tf), pos),
        Expr::Func {
            name: func_name,
            param_name,
            param_type,
            ret_type,
            box body,
            box left,
            pos,
        } => Expr::Func {
            name: func_name,
            param_name: param_name,
            param_type: aux_type(param_type, name, v, ef, lf, tf),
            ret_type: aux_type(ret_type, name, v, ef, lf, tf),
            body: box aux_expr(body, name, v, ef, lf, tf),
            left: box aux_expr(left, name, v, ef, lf, tf),
            pos: pos,
        },
        Expr::Apply(box e1, box e2, pos) => Expr::Apply(
            box aux_expr(e1, name, v, ef, lf, tf),
            box aux_expr(e2, name, v, ef, lf, tf),
            pos,
        ),
        Expr::Let(name_, typ, box e1, box e2, pos) => Expr::Let(
            name_,
            aux_type(typ, name, v, ef, lf, tf),
            box aux_expr(e1, name, v, ef, lf, tf),
            box aux_expr(e2, name, v, ef, lf, tf),
            pos,
        ),
        Expr::LetType(name_, typ, box e) => Expr::LetType(
            name_,
            aux_type(typ, name, v, ef, lf, tf),
            box aux_expr(e, name, v, ef, lf, tf),
        ),
        Expr::If(box cond, box e1, box e2, pos) => Expr::If(
            box aux_expr(cond, name, v, ef, lf, tf),
            box aux_expr(e1, name, v, ef, lf, tf),
            box aux_expr(e2, name, v, ef, lf, tf),
            pos,
        ),
        Expr::BinOp(op, box e1, box e2, pos) => Expr::BinOp(
            op,
            box aux_expr(e1, name, v, ef, lf, tf),
            box aux_expr(e2, name, v, ef, lf, tf),
            pos,
        ),
        Expr::FieldAccess(box e, typ, label, pos) => Expr::FieldAccess(
            box aux_expr(e, name, v, ef, lf, tf),
            aux_type(typ, name, v, ef, lf, tf),
            label,
            pos,
        ),
        Expr::Println(box e) => Expr::Println(box aux_expr(e, name, v, ef, lf, tf)),
        Expr::EmptyMark => Expr::EmptyMark,
    }
}

fn aux_literal<T>(
    lit: Literal,
    name: &Ident,
    v: &T,
    ef: fn(Expr, &Ident, &T) -> Option<Expr>,
    lf: fn(Literal, &Ident, &T) -> Option<Literal>,
    tf: fn(Type, &Ident, &T) -> Option<Type>,
) -> Literal {
    if let Some(lit) = lf(lit.clone(), name, v) {
        return lit;
    }
    match lit {
        Literal::Record(fields) => Literal::Record(
            fields
                .into_iter()
                .map(|(label, e)| (label, aux_expr(e, name, v, ef, lf, tf)))
                .collect(),
        ),
        _ => lit,
    }
}

fn aux_type<T>(
    typ: Type,
    name: &Ident,
    v: &T,
    ef: fn(Expr, &Ident, &T) -> Option<Expr>,
    lf: fn(Literal, &Ident, &T) -> Option<Literal>,
    tf: fn(Type, &Ident, &T) -> Option<Type>,
) -> Type {
    if let Some(typ) = tf(typ.clone(), name, v) {
        return typ;
    }
    match typ {
        Type::Func(box t1, box t2) => Type::Func(
            box aux_type(t1, name, v, ef, lf, tf),
            box aux_type(t2, name, v, ef, lf, tf),
        ),
        Type::Record(fields) => Type::Record(
            fields
                .into_iter()
                .map(|(label, typ)| (label, aux_type(typ, name, v, ef, lf, tf)))
                .collect(),
        ),
        _ => typ,
    }
}
