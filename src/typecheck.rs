use ast::{
    parsed::{self, Position},
    typed,
};
use env::Env;
use ident::Ident;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    UnmatchType(UnmatchTypeError),
    UnboundVariable(UnboundVariableError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnmatchTypeError {
    pub pos: Position,
    pub expected: parsed::Type,
    pub actual: parsed::Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnboundVariableError {
    pub pos: Position,
    pub name: Ident,
}

pub fn check_program(program: &parsed::Program) -> Result<typed::Expr, Error> {
    let mut env = Env::new();
    let mut toplevel_funcs = vec![];
    for toplevel_expr in program.0.iter() {
        match toplevel_expr {
            parsed::ToplevelExpr::Func(func) => {
                let (name, expr, typ) = check_func(func)?;
                env = env.add(name.clone(), typ);
                toplevel_funcs.push((name, expr));
            }
            _ => todo!(),
        }
    }
    check_expr(&program.1, &env)
}

pub fn check_func(func: &parsed::Func) -> Result<(Ident, typed::Expr, typed::Type), Error> {
    todo!()
}

pub fn check_expr(e: &parsed::Expr, env: &Env<typed::Type>) -> Result<typed::Expr, Error> {
    match e {
        parsed::Expr::Const(ref lit) => Ok(typed::Expr::Const(check_lit(lit, env)?)),
        parsed::Expr::Var(ref name, ref pos) => Ok(typed::Expr::Var(
            name.clone(),
            env.lookup(name).map_err(|_| {
                Error::UnboundVariable(UnboundVariableError {
                    pos: pos.clone(),
                    name: name.clone(),
                })
            })?,
        )),
        parsed::Expr::Lambda(ref param_name, ref param_type, box ref body) => {
            let param_type = typed::Type::from_parsed_type(param_type);
            let env = env.add(param_name.clone(), param_type.clone());
            let body = check_expr(body, &env)?;
            Ok(typed::Expr::Lambda(
                param_name.clone(),
                param_type,
                box body,
            ))
        }
        parsed::Expr::Apply(box ref e1, box ref e2, _) => {
            let e1 = check_expr(e1, env)?;
            let e2 = check_expr(e2, env)?;
            let t1 = typed::type_of(&e1);
            if let typed::Type::Func(box param_ty, _) = t1 {
                let t2 = typed::type_of(&e2);
                if t2 == param_ty {
                    Ok(typed::Expr::Apply(box e1, box e2))
                } else {
                    todo!()
                    /*
                    Err(Error::UnmatchType(UnmatchTypeError {
                        pos: pos,
                        expected: param_ty,
                        actual: t2,
                    })) */
                }
            } else {
                todo!()
                /* Err(Error::UnmatchType(UnmatchTypeError {
                    pos: Position { start: 0, end: 0 }, // TODO
                    expected: typed::Type::Func(box typed::Type::Int, box typed::Type::Int), // TODO,
                    actual: t1,
                })) */
            }
        }
        parsed::Expr::Let(ref name, box ref e1, box ref e2) => {
            let e1 = check_expr(e1, env)?;
            let env = env.add(name.clone(), typed::type_of(&e1));
            let e2 = check_expr(e2, &env)?;
            Ok(typed::Expr::Let(name.clone(), box e1, box e2))
        }
        parsed::Expr::LetType(ref name, ref typ, box ref e) => {
            let e = e.subst_type(name, typ);
            check_expr(&e, env)
        }
        parsed::Expr::If(box ref cond, box ref e1, box ref e2, _) => Ok(typed::Expr::If(
            box check_expr(cond, env)?,
            box check_expr(e1, env)?,
            box check_expr(e2, env)?,
        )),
        parsed::Expr::BinOp(ref op, box ref e1, box ref e2, _) => Ok(typed::Expr::BinOp(
            *op,
            box check_expr(e1, env)?,
            box check_expr(e2, env)?,
        )),
        parsed::Expr::Sequence(ref es) => {
            if let &[ref es @ .., ref e] = es.as_slice() {
                es.iter().fold(Ok(check_expr(&e, env)?), |acc, e| {
                    if let Ok(acc) = acc {
                        let e = check_expr(&e, env)?;
                        Ok(typed::Expr::Let(Ident::new("<dummy>"), box acc, box e))
                    } else {
                        acc
                    }
                })
            } else {
                unreachable!()
            }
        }
        parsed::Expr::FieldAccess(box ref e, ref label_, _) => Ok(typed::Expr::FieldAccess(
            box check_expr(e, env)?,
            label_.clone(),
        )),
        parsed::Expr::PatternMatch(box ref e, ref arms, _) => {
            let e = check_expr(e, env)?;
            let fields_ty = if let typed::Type::Record(fields_ty) = typed::type_of(&e) {
                fields_ty
            } else {
                unreachable!() // TODO
            };
            let arms: Result<_, _> = arms
                .iter()
                .map(|ref arm| {
                    let field = fields_ty
                        .iter()
                        .find(|(ref label, _)| &arm.label == label)
                        .unwrap();
                    let env = env.add(arm.name.clone(), field.1.clone());
                    let body = check_expr(&arm.body, &env)?;
                    Ok(typed::PatternMatchArm {
                        label: arm.label.clone(),
                        name: arm.name.clone(),
                        body: body,
                    })
                })
                .collect();
            Ok(typed::Expr::PatternMatch(box e, arms?))
        }
        parsed::Expr::Println(box ref e) => Ok(typed::Expr::Println(box check_expr(e, env)?)),
    }
}

pub fn check_lit(lit: &parsed::Literal, env: &Env<typed::Type>) -> Result<typed::Literal, Error> {
    match lit {
        parsed::Literal::Number(n) => Ok(typed::Literal::Number(*n)),
        parsed::Literal::Bool(b) => Ok(typed::Literal::Bool(*b)),
        parsed::Literal::Char(c) => Ok(typed::Literal::Char(*c)),
        parsed::Literal::Unit => Ok(typed::Literal::Unit),

        parsed::Literal::Variant(ref label, box ref e, ref typ, _) => Ok(typed::Literal::Variant(
            label.clone(),
            box check_expr(e, env)?,
            typed::Type::from_parsed_type(typ),
        )),
        parsed::Literal::Record(ref fields) => {
            let fields: Result<_, _> = fields
                .iter()
                .map(|(ref label, ref e)| Ok((label.clone(), check_expr(e, env)?)))
                .collect();
            let fields = fields?;
            Ok(typed::Literal::Record(fields))
        }
        parsed::Literal::Tuple(ref es) => {
            let fields: Result<_, _> = es
                .iter()
                .enumerate()
                .map(|(n, ref e)| Ok((Ident::new(&n.to_string()), check_expr(e, env)?)))
                .collect();
            Ok(typed::Literal::Record(fields?))
        }
    }
}
