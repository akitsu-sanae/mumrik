use ast::typed::*;
use ast::BinOp;
use ident::Ident;

pub fn expr(e: &Expr) -> Expr {
    match e {
        Expr::Const(lit) => Expr::Const(literal(lit)),
        _ => todo!(),
    }
}

fn literal(lit: &Literal) -> Literal {
    match lit {
        Literal::Number(n) => Literal::Number(*n),
        Literal::Bool(b) => Literal::Bool(*b),
        Literal::Char(c) => Literal::Char(*c),
        Literal::Unit => Literal::Unit,

        _ => todo!(),
    }
}



