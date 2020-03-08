use crate::{
    expr::{BinOp, Expr, Literal},
    parser::*,
};
use context::Context;
use type_::{self, Type};

#[test]
fn list() {
    assert_eq!(
        type_::check(&e, &Context::new()),
        Ok(Type::List(box Type::Int))
    );
    assert_eq!(
        e.eval(&Context::new()),
        Ok(Expr::Const(Literal::List(vec![
            Expr::Const(Literal::Number(1)),
            Expr::Const(Literal::Number(2)),
            Expr::Const(Literal::Number(3))
        ])))
    );
}

#[test]
fn string() {
    let e = expr("\"nyan\"").unwrap();
    assert_eq!(
        e,
        Expr::Const(Literal::List(vec![
            Expr::Const(Literal::Char('n')),
            Expr::Const(Literal::Char('y')),
            Expr::Const(Literal::Char('a')),
            Expr::Const(Literal::Char('n'))
        ]))
    );
    assert_eq!(
        type_::check(&e, &Context::new()),
        Ok(Type::List(box Type::Char))
    );
}

#[test]
fn let_type_func() {
    let e = expr("type a = Int; 42").unwrap();
    assert_eq!(
        e,
        Expr::LetType(
            "a".to_string(),
            box Type::Int,
            box Expr::Const(Literal::Number(42))
        )
    );
    assert_eq!(
        e.eval(&Context::new()),
        Ok(Expr::Const(Literal::Number(42)))
    );
}
