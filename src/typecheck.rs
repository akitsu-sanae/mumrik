use ast::{
    parsed::{self, Position},
    typed,
};
use env::Env;
use ident::Ident;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    pub pos: Position,
    pub expected: parsed::Type,
    pub actual: parsed::Type,
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
        _ => todo!(),
    }
}

pub fn check_lit(lit: &parsed::Literal, env: &Env<typed::Type>) -> Result<typed::Literal, Error> {
    match lit {
        parsed::Literal::Number(n, _) => Ok(typed::Literal::Number(*n)),
        parsed::Literal::Bool(b, _) => Ok(typed::Literal::Bool(*b)),
        parsed::Literal::Char(c, _) => Ok(typed::Literal::Char(*c)),
        _ => todo!(),
    }
}
