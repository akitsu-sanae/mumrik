use ast::*;
use env::Env;
use ident::Ident;
use std::collections::VecDeque;

mod subst;
mod unify;
use self::unify::Constraint;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    RecOccur {
        pos: Position,
        var: Ident,
        typ: Type,
    },
    Unify {
        pos: Position,
        typ1: Type,
        typ2: Type,
    },
    UnboundVar {
        pos: Position,
        name: Ident,
    },
    Other {
        pos: Position,
        message: String,
    },
}

pub fn check(e: Expr) -> Result<(Expr, Type), Error> {
    let (constraints, typ) = gather_constraint_from_expr(&e, &Env::new())?;
    let subst = unify::solve(constraints)?;
    Ok((subst.apply_expr(e), subst.apply_type(typ)))
}

fn gather_constraint_from_expr(
    e: &Expr,
    env: &Env<Type>,
) -> Result<(VecDeque<Constraint>, Type), Error> {
    match e {
        Expr::Const(ref lit) => gather_constraint_from_lit(lit, env),
        Expr::Var(ref name, ref typ, ref pos) => {
            if let Some(typ_) = env.lookup(name) {
                Ok((
                    VecDeque::from(vec![Constraint::Equation(
                        typ.clone(),
                        typ_.clone(),
                        pos.clone(),
                    )]),
                    typ_,
                ))
            } else {
                Err(Error::UnboundVar {
                    pos: pos.clone(),
                    name: name.clone(),
                })
            }
        }
        Expr::Func {
            ref name,
            ref param_name,
            ref param_type,
            ref ret_type,
            box ref body,
            box ref left,
            ref pos,
        } => {
            let env = env.add(
                name.clone(),
                Type::Func(box param_type.clone(), box ret_type.clone()),
            );
            let (mut left_constraints, left_typ) = gather_constraint_from_expr(left, &env)?;
            let env = if param_name.is_omitted_param_name() {
                if let Type::Record(fields) = param_type {
                    fields
                        .iter()
                        .fold(env, |acc, (name, typ)| acc.add(name.clone(), typ.clone()))
                } else {
                    unreachable!()
                }
            } else {
                env.add(param_name.clone(), param_type.clone())
            };
            let mut constraints = VecDeque::new();
            let (mut body_constraints, body_typ) = gather_constraint_from_expr(body, &env)?;
            constraints.append(&mut body_constraints);
            constraints.push_back(Constraint::Equation(
                ret_type.clone(),
                body_typ,
                pos.clone(),
            ));
            constraints.append(&mut left_constraints);
            Ok((constraints, left_typ))
        }
        Expr::Apply(box ref e1, box ref e2, ref pos) => {
            let mut constraints = VecDeque::new();
            let (mut constraints1, typ1) = gather_constraint_from_expr(e1, env)?;
            constraints.append(&mut constraints1);
            let (mut constraints2, typ2) = gather_constraint_from_expr(e2, env)?;
            constraints.append(&mut constraints2);
            let ret_type = Type::Var(Ident::fresh());
            constraints.push_back(Constraint::Equation(
                typ1,
                Type::Func(box typ2, box ret_type.clone()),
                pos.clone(),
            ));
            Ok((constraints, ret_type))
        }
        Expr::Let(ref name, ref typ, box ref e1, box ref e2, ref pos) => {
            let mut constraints = VecDeque::new();

            let (mut constraints1, typ1) = gather_constraint_from_expr(e1, env)?;
            constraints.append(&mut constraints1);
            constraints.push_back(Constraint::Equation(typ.clone(), typ1.clone(), pos.clone()));

            let env = env.add(name.clone(), typ1);

            let (mut constraints2, typ2) = gather_constraint_from_expr(e2, &env)?;
            constraints.append(&mut constraints2);

            Ok((constraints, typ2))
        }
        Expr::LetType(ref name, ref typ, box ref e) => {
            let env = env.add(name.clone(), typ.clone());
            gather_constraint_from_expr(e, &env)
        }
        Expr::If(box ref cond, box ref e1, box ref e2, ref pos) => {
            let mut constraints = VecDeque::new();

            let (mut cond_constraints, cond_typ) = gather_constraint_from_expr(cond, env)?;
            let (mut constraints1, typ1) = gather_constraint_from_expr(e1, env)?;
            let (mut constraints2, typ2) = gather_constraint_from_expr(e2, env)?;

            constraints.append(&mut cond_constraints);
            constraints.append(&mut constraints1);
            constraints.append(&mut constraints2);
            constraints.push_back(Constraint::Equation(cond_typ, Type::Bool, pos.clone()));
            constraints.push_back(Constraint::Equation(typ1.clone(), typ2, pos.clone()));
            Ok((constraints, typ1))
        }
        Expr::BinOp(ref op, box ref e1, box ref e2, ref pos) => {
            let mut constraints = VecDeque::new();
            let (mut constraints1, typ1) = gather_constraint_from_expr(e1, env)?;
            let (mut constraints2, typ2) = gather_constraint_from_expr(e2, env)?;
            constraints.append(&mut constraints1);
            constraints.append(&mut constraints2);
            constraints.push_back(Constraint::Equation(typ1.clone(), typ2, pos.clone()));

            match op {
                BinOp::Add | BinOp::Sub | BinOp::Mult | BinOp::Div => {
                    constraints.push_back(Constraint::Equation(typ1, Type::Int, pos.clone()));
                    Ok((constraints, Type::Int))
                }
                BinOp::Eq | BinOp::Neq | BinOp::Lt | BinOp::Gt => Ok((constraints, Type::Bool)),
            }
        }
        Expr::RecordGet(box ref e, ref typ, ref label, ref pos) => {
            let (mut constraints, typ_) = gather_constraint_from_expr(e, env)?;
            constraints.push_back(Constraint::Equation(typ.clone(), typ_.clone(), pos.clone()));
            let elem_type = Type::Var(Ident::fresh());
            constraints.push_back(Constraint::RecordAt(
                typ_,
                label.clone(),
                elem_type.clone(),
                pos.clone(),
            ));

            Ok((constraints, elem_type))
        }
        Expr::ArrayGet(box ref e1, box ref e2, ref pos) => {
            let mut constraints = VecDeque::new();
            let (mut constraints1, typ1) = gather_constraint_from_expr(e1, env)?;
            constraints.append(&mut constraints1);
            let elem_typ = Type::Var(Ident::fresh());
            constraints.push_back(Constraint::Array(
                typ1.clone(),
                elem_typ.clone(),
                pos.clone(),
            ));
            let (mut constraints2, typ2) = gather_constraint_from_expr(e2, env)?;
            constraints.append(&mut constraints2);
            constraints.push_back(Constraint::Equation(typ2.clone(), Type::Int, pos.clone()));
            Ok((constraints, elem_typ))
        }
        Expr::Assign(box ref e1, box ref e2, ref pos) => {
            let mut constraints = VecDeque::new();
            let (mut constraints1, typ1) = gather_constraint_from_expr(e1, env)?;
            constraints.append(&mut constraints1);
            let (mut constraints2, typ2) = gather_constraint_from_expr(e2, env)?;
            constraints.append(&mut constraints2);
            constraints.push_back(Constraint::Equation(typ1.clone(), typ2, pos.clone()));
            Ok((constraints, typ1))
        }
        Expr::Println(box ref e) => {
            let (constraints, _) = gather_constraint_from_expr(e, env)?;
            Ok((constraints, Type::Unit))
        }
        Expr::EmptyMark => Ok((VecDeque::new(), Type::EmptyMark)),
    }
}

fn gather_constraint_from_lit(
    lit: &Literal,
    env: &Env<Type>,
) -> Result<(VecDeque<Constraint>, Type), Error> {
    match lit {
        Literal::Number(_) => Ok((VecDeque::new(), Type::Int)),
        Literal::Bool(_) => Ok((VecDeque::new(), Type::Bool)),
        Literal::Char(_) => Ok((VecDeque::new(), Type::Char)),
        Literal::Unit => Ok((VecDeque::new(), Type::Unit)),
        Literal::Record(ref fields) => {
            let mut constraints = VecDeque::new();
            let fields: Result<_, _> = fields
                .iter()
                .map(|(label, e)| {
                    let (mut constraints_, typ) = gather_constraint_from_expr(e, env)?;
                    constraints.append(&mut constraints_);
                    Ok((label.clone(), typ))
                })
                .collect();
            Ok((constraints, Type::Record(fields?)))
        }
        Literal::Array(ref elems, ref elem_typ) => {
            let mut constraints = VecDeque::new();
            for e in elems {
                let (mut constraints_, typ) = gather_constraint_from_expr(e, env)?;
                constraints.append(&mut constraints_);
                constraints.push_back(Constraint::Equation(
                    elem_typ.clone(),
                    typ,
                    Position { start: 0, end: 0 },
                )); // TODO
            }
            Ok((constraints, Type::Array(box elem_typ.clone(), elems.len())))
        }
    }
}
