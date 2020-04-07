use ast::*;
use env::Env;
use ident::Ident;
use std::collections::VecDeque;

mod subst;
use self::subst::Subst;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    UnmatchType(UnmatchTypeError),
    UnboundVariable(UnboundVariableError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnmatchTypeError {
    pub pos: Position,
    pub expected: Type,
    pub actual: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnboundVariableError {
    pub pos: Position,
    pub name: Ident,
}

fn unify(mut queue: VecDeque<(Type, Type)>, pos: Position) -> Result<Subst, Error> {
    let mut subst = Subst::new();
    loop {
        if let Some((typ1, typ2)) = queue.pop_front() {
            match (typ1, typ2) {
                (typ1, typ2) if typ1 == typ2 => (),
                (Type::Func(box typ11, box typ12), Type::Func(box typ21, box typ22)) => {
                    queue.push_back((typ11, typ21));
                    queue.push_back((typ12, typ22));
                }
                (Type::Record(mut fields1), Type::Record(mut fields2)) => {
                    fields1.sort_by(|a, b| a.0.cmp(&b.0));
                    fields2.sort_by(|a, b| a.0.cmp(&b.0));
                    if fields1.len() != fields2.len() {
                        return Err(Error::UnmatchType(UnmatchTypeError {
                            pos: pos,
                            expected: Type::Record(fields1),
                            actual: Type::Record(fields2),
                        }));
                    }
                    for (ref field1, ref field2) in fields1.iter().zip(fields2.iter()) {
                        if field1.0 != field2.0 {
                            return Err(Error::UnmatchType(UnmatchTypeError {
                                pos: pos,
                                expected: field1.1.clone(),
                                actual: field2.1.clone(),
                            }));
                        }
                    }
                    for (field1, field2) in fields1.into_iter().zip(fields2.into_iter()) {
                        queue.push_back((field1.1, field2.1));
                    }
                }
                (Type::Var(name), typ) | (typ, Type::Var(name)) => {
                    queue = queue
                        .into_iter()
                        .map(|(typ1, typ2)| {
                            (typ1.subst_type(&name, &typ), typ2.subst_type(&name, &typ))
                        })
                        .collect();
                    subst = Subst(
                        subst
                            .0
                            .into_iter()
                            .map(|(name_, typ_): (Ident, Type)| {
                                (name_, typ_.subst_type(&name, &typ))
                            })
                            .collect(),
                    );
                    if typ.is_occurs(&name) {
                        todo!() // return error
                    } else if !subst.0.iter().any(|(name_, _)| name_ == &name) {
                        subst.0.insert(name, typ);
                    }
                }
                (typ1, typ2) => {
                    return Err(Error::UnmatchType(UnmatchTypeError {
                        pos: pos,
                        expected: typ1,
                        actual: typ2,
                    }))
                }
            }
        } else {
            break;
        }
    }
    Ok(subst)
}

pub fn check(e: Expr) -> Result<(Expr, Type), Error> {
    let (_, typ, subst) = check_expr(&e, &Env::new())?;
    let e = subst.apply_expr(e);
    Ok((e, typ))
}

fn check_expr(e: &Expr, env: &Env<Type>) -> Result<(Env<Type>, Type, Subst), Error> {
    match e {
        Expr::Const(ref lit) => Ok(check_literal(lit, env)?),
        Expr::Var(ref name, ref _pos) => {
            if let Some(typ) = env.lookup(&name) {
                Ok((env.clone(), typ, Subst::new()))
            } else {
                let type_var = Type::Var(Ident::fresh());
                let env = env.add(name.clone(), type_var.clone());
                Ok((env, type_var, Subst::new()))
            }
        }
        Expr::Apply(box ref e1, box ref e2, ref pos) => {
            let (env, ty1, subst1) = check_expr(e1, env)?;
            let (env, ty2, subst2) = check_expr(e2, &env)?;
            let ret_type = Type::Var(Ident::fresh());
            let subst3 = unify(
                VecDeque::from(vec![(ty1, Type::Func(box ty2, box ret_type.clone()))]),
                pos.clone(),
            )?;
            let ret_type = subst3.apply_type(ret_type);
            let env = subst3.apply_env(env);
            let subst = Subst::compose(subst1, Subst::compose(subst2, subst3));
            Ok((env, ret_type, subst))
        }
        _ => todo!(),
    }
}

fn check_literal(lit: &Literal, env: &Env<Type>) -> Result<(Env<Type>, Type, Subst), Error> {
    match lit {
        Literal::Func(ref param_name, ref param_type, ref ret_type, box ref body, ref pos) => {
            let env = env.add(param_name.clone(), param_type.clone());
            let (env, body_ty, subst1) = check_expr(body, &env)?;
            let subst2 = unify(
                VecDeque::from(vec![(ret_type.clone(), body_ty)]),
                pos.clone(),
            )?;
            let param_type = subst2.apply_type(param_type.clone());
            let ret_type = subst2.apply_type(ret_type.clone());
            Ok((
                env,
                Type::Func(box param_type, box ret_type),
                Subst::compose(subst1, subst2),
            ))
        }
        Literal::Number(_) => Ok((Env::new(), Type::Int, Subst::new())),
        Literal::Bool(_) => Ok((Env::new(), Type::Bool, Subst::new())),
        Literal::Char(_) => Ok((Env::new(), Type::Char, Subst::new())),
        Literal::Unit => Ok((Env::new(), Type::Unit, Subst::new())),
        Literal::Record(_fields) => todo!(),
    }
}
