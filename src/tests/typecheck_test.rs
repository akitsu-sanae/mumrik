use ast::parsed::Position;
use ast::{parsed, typed, BinOp};
use env::Env;
use ident::Ident;
use typecheck::*;

#[test]
fn primitive_literal() {
    let env = Env::new();
    assert_eq!(
        check_lit(&parsed::Literal::Number(42), &env),
        Ok(typed::Literal::Number(42))
    );
    assert_eq!(
        check_lit(&parsed::Literal::Bool(true), &env),
        Ok(typed::Literal::Bool(true))
    );
    assert_eq!(
        check_lit(&parsed::Literal::Char('c'), &env),
        Ok(typed::Literal::Char('c'))
    );
    assert_eq!(
        check_lit(&parsed::Literal::Unit, &env),
        Ok(typed::Literal::Unit)
    );
}

#[test]
fn apply() {
    use ast::parsed::{Expr::*, Literal::*};
    let env = Env::new().add(
        Ident::new("a"),
        typed::Type::Func(box typed::Type::Int, box typed::Type::Unit),
    );
    assert_eq!(
        check_expr(
            &Apply(
                box Var(Ident::new("a"), Position { start: 0, end: 0 }),
                box Const(Number(1)),
                Position { start: 0, end: 0 }
            ),
            &env
        ),
        Ok(typed::Expr::Apply(
            box typed::Expr::Var(
                Ident::new("a"),
                typed::Type::Func(box typed::Type::Int, box typed::Type::Unit)
            ),
            box typed::Expr::Const(typed::Literal::Number(1))
        ))
    );
}

#[test]
fn if_() {
    use ast::parsed::{Expr::*, Literal::*};
    let env = Env::new();
    assert_eq!(
        check_expr(
            &If(
                box Const(Bool(true)),
                box Const(Number(1)),
                box Const(Number(2)),
                Position { start: 0, end: 0 }
            ),
            &env
        ),
        Ok(typed::Expr::If(
            box typed::Expr::Const(typed::Literal::Bool(true)),
            box typed::Expr::Const(typed::Literal::Number(1)),
            box typed::Expr::Const(typed::Literal::Number(2))
        ))
    );
}

#[test]
fn arithmetic() {
    let env = Env::new();
    assert_eq!(
        check_expr(
            &parsed::Expr::BinOp(
                BinOp::Add,
                box parsed::Expr::BinOp(
                    BinOp::Add,
                    box parsed::Expr::Const(parsed::Literal::Number(1)),
                    box parsed::Expr::BinOp(
                        BinOp::Mult,
                        box parsed::Expr::Const(parsed::Literal::Number(2)),
                        box parsed::Expr::Const(parsed::Literal::Number(5)),
                        Position { start: 0, end: 0 }
                    ),
                    Position { start: 0, end: 0 }
                ),
                box parsed::Expr::Const(parsed::Literal::Number(6)),
                Position { start: 0, end: 0 }
            ),
            &env
        ),
        Ok(typed::Expr::BinOp(
            BinOp::Add,
            box typed::Expr::BinOp(
                BinOp::Add,
                box typed::Expr::Const(typed::Literal::Number(1)),
                box typed::Expr::BinOp(
                    BinOp::Mult,
                    box typed::Expr::Const(typed::Literal::Number(2)),
                    box typed::Expr::Const(typed::Literal::Number(5))
                )
            ),
            box typed::Expr::Const(typed::Literal::Number(6))
        ))
    );
}

#[test]
fn let_type() {
    let env = Env::new();
    assert_eq!(
        check_expr(
            &parsed::Expr::LetType(
                Ident::new("i"),
                parsed::Type::Int,
                box parsed::Expr::Lambda(
                    Ident::new("a"),
                    parsed::Type::Var(Ident::new("i"), Position { start: 0, end: 0 }),
                    box parsed::Expr::Var(Ident::new("a"), Position { start: 0, end: 0 }),
                )
            ),
            &env
        ),
        Ok(typed::Expr::Lambda(
            Ident::new("a"),
            typed::Type::Int,
            box typed::Expr::Var(Ident::new("a"), typed::Type::Int)
        ))
    );
}
