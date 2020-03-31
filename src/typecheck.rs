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
            parsed::ToplevelExpr::Func(func, _) => {
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
        parsed::Expr::Const(lit) => Ok(typed::Expr::Const(check_lit(lit, env)?)),
        parsed::Expr::Var(name, pos) => Ok(typed::Expr::Var(
            name.clone(),
            env.lookup(name).map_err(|_| {
                Error::UnboundVariable(UnboundVariableError {
                    pos: pos.clone(),
                    name: name.clone(),
                })
            })?,
        )),
        parsed::Expr::Lambda(param_name, param_type, box body, _) => {
            let param_type = typed::Type::from_parsed_type(param_type);
            let env = env.add(param_name.clone(), param_type.clone());
            let body = check_expr(body, &env)?;
            Ok(typed::Expr::Lambda(
                param_name.clone(),
                param_type,
                box body,
            ))
        }
        parsed::Expr::Apply(box ref e1, box ref e2) => {
            let e1 = check_expr(e1, env)?;
            let e2 = check_expr(e2, env)?;
            Ok(typed::Expr::Apply(box e1, box e2))
        }
        parsed::Expr::Let(name, box ref e1, box ref e2, _) => {
            let e1 = check_expr(e1, env)?;
            let env = env.add(name.clone(), typed::type_of(&e1));
            let e2 = check_expr(e2, &env)?;
            Ok(typed::Expr::Let(name.clone(), box e1, box e2))
        }
        parsed::Expr::LetType(ref name, ref typ, box ref e, _) => {
            let e = e.subst_type(name, typ);
            check_expr(&e, env)
        }
        parsed::Expr::If(box cond, box e1, box e2, _) => Ok(typed::Expr::If(
            box check_expr(cond, env)?,
            box check_expr(e1, env)?,
            box check_expr(e2, env)?,
        )),
        parsed::Expr::BinOp(op, box e1, box e2) => Ok(typed::Expr::BinOp(
            *op,
            box check_expr(e1, env)?,
            box check_expr(e2, env)?,
        )),
        _ => todo!(),
    }
}

pub fn check_lit(lit: &parsed::Literal, env: &Env<typed::Type>) -> Result<typed::Literal, Error> {
    match lit {
        parsed::Literal::Number(n, _) => Ok(typed::Literal::Number(*n)),
        parsed::Literal::Bool(b, _) => Ok(typed::Literal::Bool(*b)),
        parsed::Literal::Char(c, _) => Ok(typed::Literal::Char(*c)),
        parsed::Literal::Unit(_) => Ok(typed::Literal::Unit),
        _ => todo!(),
    }
}
