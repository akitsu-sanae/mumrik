use ast::*;
use typecheck;

#[test]
fn primitive_literal() {
    assert_eq!(
        typecheck::check(Expr::Const(Literal::Number(42))),
        Ok((Expr::Const(Literal::Number(42)), Type::Int))
    );
    assert_eq!(
        typecheck::check(Expr::Const(Literal::Bool(true))),
        Ok((Expr::Const(Literal::Bool(true)), Type::Bool))
    );
}
