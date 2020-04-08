use ast::*;
use ident::Ident;
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

    assert_eq!(
        typecheck::check(Expr::Const(Literal::Func {
            param_name: Ident::new("x"),
            param_type: Type::Int,
            ret_type: Type::Int,
            body: box Expr::Const(Literal::Number(42)),
            pos: Position { start: 0, end: 0 }
        })),
        Ok((
            Expr::Const(Literal::Func {
                param_name: Ident::new("x"),
                param_type: Type::Int,
                ret_type: Type::Int,
                body: box Expr::Const(Literal::Number(42)),
                pos: Position { start: 0, end: 0 }
            }),
            Type::Func(box Type::Int, box Type::Int)
        ))
    );
}

#[test]
fn apply() {
    assert_eq!(
        typecheck::check(Expr::Apply(
            box Expr::Const(Literal::Func {
                param_name: Ident::new("x"),
                param_type: Type::Int,
                ret_type: Type::Int,
                body: box Expr::Var(Ident::new("x"), Position { start: 0, end: 0 }),
                pos: Position { start: 0, end: 0 }
            }),
            box Expr::Const(Literal::Number(42)),
            Position { start: 0, end: 0 }
        )),
        Ok((
            Expr::Apply(
                box Expr::Const(Literal::Func {
                    param_name: Ident::new("x"),
                    param_type: Type::Int,
                    ret_type: Type::Int,
                    body: box Expr::Var(Ident::new("x"), Position { start: 0, end: 0 }),
                    pos: Position { start: 0, end: 0 }
                }),
                box Expr::Const(Literal::Number(42)),
                Position { start: 0, end: 0 }
            ),
            Type::Int
        ))
    );
    assert_eq!(
        typecheck::check(Expr::Apply(
            box Expr::Const(Literal::Func {
                param_name: Ident::new("x"),
                param_type: Type::Var(Ident::new("a")),
                ret_type: Type::Var(Ident::new("b")),
                body: box Expr::Var(Ident::new("x"), Position { start: 0, end: 0 }),
                pos: Position { start: 0, end: 0 }
            }),
            box Expr::Const(Literal::Number(42)),
            Position { start: 0, end: 0 }
        )),
        Ok((
            Expr::Apply(
                box Expr::Const(Literal::Func {
                    param_name: Ident::new("x"),
                    param_type: Type::Int,
                    ret_type: Type::Int,
                    body: box Expr::Var(Ident::new("x"), Position { start: 0, end: 0 }),
                    pos: Position { start: 0, end: 0 }
                }),
                box Expr::Const(Literal::Number(42)),
                Position { start: 0, end: 0 }
            ),
            Type::Int
        ))
    );
}

#[test]
fn binop_expr() {
    let x_var = Expr::Var(Ident::new("x"), Position { start: 0, end: 0 });
    assert_eq!(
        typecheck::check(Expr::Const(Literal::Func {
            param_name: Ident::new("x"),
            param_type: Type::Var(Ident::new("a")),
            ret_type: Type::Var(Ident::new("b")),
            body: box Expr::BinOp(
                BinOp::Add,
                box x_var.clone(),
                box x_var.clone(),
                Position { start: 0, end: 0 }
            ),
            pos: Position { start: 0, end: 0 }
        })),
        Ok((
            Expr::Const(Literal::Func {
                param_name: Ident::new("x"),
                param_type: Type::Int,
                ret_type: Type::Int,
                body: box Expr::BinOp(
                    BinOp::Add,
                    box x_var.clone(),
                    box x_var.clone(),
                    Position { start: 0, end: 0 }
                ),
                pos: Position { start: 0, end: 0 }
            }),
            Type::Func(box Type::Int, box Type::Int)
        ))
    );
}

#[test]
fn if_expr() {
    assert_eq!(
        typecheck::check(Expr::Const(Literal::Func {
            param_name: Ident::new("x"),
            param_type: Type::Var(Ident::new("a1")),
            ret_type: Type::Var(Ident::new("b1")),
            body: box Expr::Const(Literal::Func {
                param_name: Ident::new("y"),
                param_type: Type::Var(Ident::new("a2")),
                ret_type: Type::Var(Ident::new("b1")),
                body: box Expr::If(
                    box Expr::Var(Ident::new("x"), Position { start: 0, end: 0 }),
                    box Expr::Var(Ident::new("y"), Position { start: 0, end: 0 }),
                    box Expr::Const(Literal::Number(42)),
                    Position { start: 0, end: 0 }
                ),
                pos: Position { start: 0, end: 0 }
            }),
            pos: Position { start: 0, end: 0 }
        })),
        Ok((
            Expr::Const(Literal::Func {
                param_name: Ident::new("x"),
                param_type: Type::Bool,
                ret_type: Type::Int,
                body: box Expr::Const(Literal::Func {
                    param_name: Ident::new("y"),
                    param_type: Type::Int,
                    ret_type: Type::Int,
                    body: box Expr::If(
                        box Expr::Var(Ident::new("x"), Position { start: 0, end: 0 }),
                        box Expr::Var(Ident::new("y"), Position { start: 0, end: 0 }),
                        box Expr::Const(Literal::Number(42)),
                        Position { start: 0, end: 0 }
                    ),
                    pos: Position { start: 0, end: 0 }
                }),
                pos: Position { start: 0, end: 0 }
            }),
            Type::Func(box Type::Bool, box Type::Int)
        ))
    );
}
