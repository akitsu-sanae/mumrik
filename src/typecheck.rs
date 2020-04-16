use ast::*;
use env::Env;
use ident::Ident;
use std::collections::VecDeque;

mod subst;
use self::subst::Subst;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    RecursiveOccurrence {
        pos: Position,
        var: Ident,
        typ: Type,
    },
    UnmatchType {
        pos: Position,
        expected: Type,
        actual: Type,
    },
    UnboundVariable {
        pos: Position,
        name: Ident,
    },
}

fn unify(mut queue: VecDeque<(Type, Type)>, pos: &Position) -> Result<Subst, Error> {
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
                        return Err(Error::UnmatchType {
                            pos: pos.clone(),
                            expected: Type::Record(fields1),
                            actual: Type::Record(fields2),
                        });
                    }
                    for (ref field1, ref field2) in fields1.iter().zip(fields2.iter()) {
                        if field1.0 != field2.0 {
                            return Err(Error::UnmatchType {
                                pos: pos.clone(),
                                expected: field1.1.clone(),
                                actual: field2.1.clone(),
                            });
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
                        return Err(Error::RecursiveOccurrence {
                            pos: pos.clone(),
                            var: name,
                            typ: typ,
                        });
                    } else if !subst.0.iter().any(|(name_, _)| name_ == &name) {
                        subst.0.insert(name, typ);
                    }
                }
                (typ1, typ2) => {
                    return Err(Error::UnmatchType {
                        pos: pos.clone(),
                        expected: typ1,
                        actual: typ2,
                    })
                }
            }
        } else {
            break;
        }
    }
    Ok(subst)
}

pub fn check(e: Expr) -> Result<(Expr, Type), Error> {
    let (typ, subst) = check_expr(&e, &Env::new())?;
    let e = subst.apply_expr(e);
    Ok((e, typ))
}

fn check_expr(e: &Expr, env: &Env<Type>) -> Result<(Type, Subst), Error> {
    match e {
        Expr::Const(ref lit) => Ok(check_literal(lit, env)?),
        Expr::Var(ref name, ref typ, ref pos) => {
            if let Some(typ_) = env.lookup(name) {
                let subst = unify(VecDeque::from(vec![(typ_.clone(), typ.clone())]), pos)?;
                let typ_ = subst.apply_type(typ_);
                Ok((typ_, subst))
            } else {
                Err(Error::UnboundVariable {
                    pos: pos.clone(),
                    name: name.clone(),
                })
            }
        }
        Expr::Apply(box ref e1, box ref e2, ref pos) => {
            let (ty1, subst1) = check_expr(e1, env)?;
            let (ty2, subst2) = check_expr(e2, env)?;
            let ret_type = Type::Var(Ident::fresh());
            let subst3 = unify(
                VecDeque::from(vec![(ty1, Type::Func(box ty2, box ret_type.clone()))]),
                pos,
            )?;
            let subst = Subst::compose(subst1, Subst::compose(subst2, subst3));
            let ret_type = subst.apply_type(ret_type);
            Ok((ret_type, subst))
        }
        Expr::Let(ref name, ref typ, box ref e1, box ref e2, ref pos) => {
            let (ty1, subst1) = check_expr(e1, env)?;
            let env = env.add(name.clone(), ty1.clone());
            let (ty2, subst2) = check_expr(e2, &env)?;
            let subst3 = unify(VecDeque::from(vec![(typ.clone(), ty1)]), pos)?;
            let subst = Subst::compose(subst1, Subst::compose(subst2, subst3));
            let ty2 = subst.apply_type(ty2);
            Ok((ty2, subst))
        }
        Expr::LetRec(ref name, ref ty, box ref e1, box ref e2, ref pos) => {
            let env = env.add(name.clone(), ty.clone());
            let (ty1, subst1) = check_expr(e1, &env)?;
            let subst2 = unify(VecDeque::from(vec![(ty.clone(), ty1)]), pos)?;
            let (ty2, subst3) = check_expr(e2, &env)?;
            let subst = Subst::compose(subst1, Subst::compose(subst2, subst3));
            let ty2 = subst.apply_type(ty2);
            Ok((ty2, subst))
        }
        Expr::LetType(ref name, ref typ, box ref e) => {
            let env = env.add(name.clone(), typ.clone());
            check_expr(e, &env)
        }
        Expr::If(box ref cond, box ref e1, box ref e2, ref pos) => {
            let (cond_ty, subst1) = check_expr(cond, env)?;
            let (ty1, subst2) = check_expr(e1, env)?;
            let (ty2, subst3) = check_expr(e2, env)?;
            let subst_if = unify(
                VecDeque::from(vec![(cond_ty, Type::Bool), (ty1.clone(), ty2)]),
                pos,
            )?;
            let subst = Subst::compose(
                subst1,
                Subst::compose(subst2, Subst::compose(subst3, subst_if)),
            );
            let ty = subst.apply_type(ty1);
            Ok((ty, subst))
        }
        Expr::BinOp(ref op, box ref e1, box ref e2, ref pos) => {
            let (ty1, subst1) = check_expr(e1, env)?;
            let (ty2, subst2) = check_expr(e2, env)?;
            let subst_eq = unify(VecDeque::from(vec![(ty1.clone(), ty2)]), pos)?;
            let subst = Subst::compose(subst_eq, Subst::compose(subst1, subst2));
            match op {
                BinOp::Add | BinOp::Sub | BinOp::Mult | BinOp::Div => {
                    let subst_ = unify(VecDeque::from(vec![(ty1, Type::Int)]), pos)?;
                    let subst = Subst::compose(subst, subst_);
                    Ok((Type::Int, subst))
                }
                BinOp::Eq | BinOp::Neq | BinOp::Lt | BinOp::Gt => Ok((Type::Bool, subst)),
            }
        }
        Expr::FieldAccess(box ref e, ref typ, ref label, ref pos) => {
            let (typ_, subst) = check_expr(e, env)?;
            let subst_eq = unify(VecDeque::from(vec![(typ_.clone(), typ.clone())]), pos)?;
            let subst = Subst::compose(subst, subst_eq);
            if let Type::Record(fields) = subst.apply_type(typ_) {
                Ok((
                    fields
                        .into_iter()
                        .find(|(ref label_, _)| label == label_)
                        .unwrap()
                        .1,
                    subst,
                ))
            } else {
                todo!()
            }
        }
        Expr::Println(box ref e) => {
            let (_, subst) = check_expr(e, env)?;
            Ok((Type::Unit, subst))
        }
    }
}

fn check_literal(lit: &Literal, env: &Env<Type>) -> Result<(Type, Subst), Error> {
    match lit {
        Literal::Func {
            ref param_name,
            ref param_type,
            ref ret_type,
            box ref body,
            ref pos,
        } => {
            let env = env.add(param_name.clone(), param_type.clone());
            let (body_ty, subst1) = check_expr(body, &env)?;
            let subst2 = unify(VecDeque::from(vec![(ret_type.clone(), body_ty)]), pos)?;
            let subst = Subst::compose(subst1, subst2);
            let param_type = subst.apply_type(param_type.clone());
            let ret_type = subst.apply_type(ret_type.clone());
            Ok((Type::Func(box param_type, box ret_type), subst))
        }
        Literal::Number(_) => Ok((Type::Int, Subst::new())),
        Literal::Bool(_) => Ok((Type::Bool, Subst::new())),
        Literal::Char(_) => Ok((Type::Char, Subst::new())),
        Literal::Unit => Ok((Type::Unit, Subst::new())),
        Literal::Record(ref fields) => {
            let mut substs = vec![];
            let fields: Result<_, _> = fields
                .iter()
                .map(|(label, e)| {
                    let (ty, subst) = check_expr(e, env)?;
                    substs.push(subst);
                    Ok((label.clone(), ty))
                })
                .collect();
            Ok((
                Type::Record(fields?),
                substs
                    .into_iter()
                    .fold(Subst::new(), |acc, subst| Subst::compose(acc, subst)),
            ))
        }
    }
}
