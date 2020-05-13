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
        typecheck::check(Expr::Func {
            name: Ident::new("f"),
            param_name: Ident::new("x"),
            param_type: Type::Int,
            ret_type: Type::Int,
            body: box Expr::Const(Literal::Number(42)),
            left: box Expr::Var(
                Ident::new("f"),
                Type::Func(box Type::Int, box Type::Int),
                Position { start: 0, end: 0 }
            ),
            pos: Position { start: 0, end: 0 }
        }),
        Ok((
            Expr::Func {
                name: Ident::new("f"),
                param_name: Ident::new("x"),
                param_type: Type::Int,
                ret_type: Type::Int,
                body: box Expr::Const(Literal::Number(42)),
                left: box Expr::Var(
                    Ident::new("f"),
                    Type::Func(box Type::Int, box Type::Int),
                    Position { start: 0, end: 0 }
                ),
                pos: Position { start: 0, end: 0 }
            },
            Type::Func(box Type::Int, box Type::Int)
        ))
    );
}

#[test]
fn apply() {
    let func_name = Ident::fresh();
    assert_eq!(
        typecheck::check(Expr::Apply(
            box Expr::Func {
                name: func_name.clone(),
                param_name: Ident::new("x"),
                param_type: Type::Int,
                ret_type: Type::Int,
                body: box Expr::Var(
                    Ident::new("x"),
                    Type::Var(Ident::fresh()),
                    Position { start: 0, end: 0 }
                ),
                left: box Expr::Var(
                    func_name.clone(),
                    Type::Func(box Type::Int, box Type::Int),
                    Position { start: 0, end: 0 }
                ),
                pos: Position { start: 0, end: 0 }
            },
            box Expr::Const(Literal::Number(42)),
            Position { start: 0, end: 0 }
        )),
        Ok((
            Expr::Apply(
                box Expr::Func {
                    name: func_name.clone(),
                    param_name: Ident::new("x"),
                    param_type: Type::Int,
                    ret_type: Type::Int,
                    body: box Expr::Var(Ident::new("x"), Type::Int, Position { start: 0, end: 0 }),
                    left: box Expr::Var(
                        func_name.clone(),
                        Type::Func(box Type::Int, box Type::Int),
                        Position { start: 0, end: 0 }
                    ),
                    pos: Position { start: 0, end: 0 }
                },
                box Expr::Const(Literal::Number(42)),
                Position { start: 0, end: 0 }
            ),
            Type::Int,
        ))
    );

    assert_eq!(
        typecheck::check(Expr::Apply(
            box Expr::Func {
                name: func_name.clone(),
                param_name: Ident::new("x"),
                param_type: Type::Var(Ident::new("a")),
                ret_type: Type::Var(Ident::new("b")),
                body: box Expr::Var(
                    Ident::new("x"),
                    Type::Var(Ident::fresh()),
                    Position { start: 0, end: 0 }
                ),
                left: box Expr::Var(
                    func_name.clone(),
                    Type::Var(Ident::new("d")),
                    Position { start: 0, end: 0 }
                ),
                pos: Position { start: 0, end: 0 }
            },
            box Expr::Const(Literal::Number(42)),
            Position { start: 0, end: 0 }
        )),
        Ok((
            Expr::Apply(
                box Expr::Func {
                    name: func_name.clone(),
                    param_name: Ident::new("x"),
                    param_type: Type::Int,
                    ret_type: Type::Int,
                    body: box Expr::Var(Ident::new("x"), Type::Int, Position { start: 0, end: 0 }),
                    left: box Expr::Var(
                        func_name.clone(),
                        Type::Var(Ident::new("d")),
                        Position { start: 0, end: 0 }
                    ),
                    pos: Position { start: 0, end: 0 }
                },
                box Expr::Const(Literal::Number(42)),
                Position { start: 0, end: 0 }
            ),
            Type::Int
        ))
    );
}

#[test]
fn binop_expr() {
    let x_var_before = Expr::Var(
        Ident::new("x"),
        Type::Var(Ident::fresh()),
        Position { start: 0, end: 0 },
    );
    let x_var_after = Expr::Var(Ident::new("x"), Type::Int, Position { start: 0, end: 0 });
    let func_name = Ident::fresh();
    assert_eq!(
        typecheck::check(Expr::Func {
            name: func_name.clone(),
            param_name: Ident::new("x"),
            param_type: Type::Var(Ident::new("a")),
            ret_type: Type::Var(Ident::new("b")),
            body: box Expr::BinOp(
                BinOp::Add,
                box x_var_before.clone(),
                box x_var_before.clone(),
                Position { start: 0, end: 0 }
            ),
            left: box Expr::Var(
                func_name.clone(),
                Type::Var(Ident::new("c")),
                Position { start: 0, end: 0 }
            ),
            pos: Position { start: 0, end: 0 }
        }),
        Ok((
            Expr::Func {
                name: func_name.clone(),
                param_name: Ident::new("x"),
                param_type: Type::Int,
                ret_type: Type::Int,
                body: box Expr::BinOp(
                    BinOp::Add,
                    box x_var_before.clone(),
                    box x_var_before.clone(),
                    Position { start: 0, end: 0 }
                ),
                left: box Expr::Var(
                    func_name.clone(),
                    Type::Func(box Type::Int, box Type::Int),
                    Position { start: 0, end: 0 }
                ),
                pos: Position { start: 0, end: 0 }
            },
            Type::Func(box Type::Int, box Type::Int)
        ))
    );
}

#[test]
fn if_expr() {
    let func_name1 = Ident::fresh();
    let func_name2 = Ident::fresh();
    assert_eq!(
        typecheck::check(Expr::Func {
            name: func_name1.clone(),
            param_name: Ident::new("x"),
            param_type: Type::Var(Ident::new("a1")),
            ret_type: Type::Var(Ident::new("b1")),
            body: box Expr::Func {
                name: func_name2.clone(),
                param_name: Ident::new("y"),
                param_type: Type::Var(Ident::new("a2")),
                ret_type: Type::Var(Ident::new("b2")),
                body: box Expr::If(
                    box Expr::Var(
                        Ident::new("x"),
                        Type::Var(Ident::fresh()),
                        Position { start: 0, end: 1 }
                    ),
                    box Expr::Var(
                        Ident::new("y"),
                        Type::Var(Ident::fresh()),
                        Position { start: 0, end: 2 }
                    ),
                    box Expr::Const(Literal::Number(42)),
                    Position { start: 0, end: 3 }
                ),
                left: box Expr::Var(
                    func_name2.clone(),
                    Type::Var(Ident::fresh()),
                    Position { start: 0, end: 0 }
                ),
                pos: Position { start: 0, end: 0 }
            },
            left: box Expr::Var(
                func_name1.clone(),
                Type::Var(Ident::fresh()),
                Position { start: 0, end: 0 }
            ),
            pos: Position { start: 0, end: 0 },
        }),
        Ok((
            Expr::Func {
                name: func_name1.clone(),
                param_name: Ident::new("x"),
                param_type: Type::Bool,
                ret_type: Type::Int,
                body: box Expr::Func {
                    name: func_name2.clone(),
                    param_name: Ident::new("y"),
                    param_type: Type::Int,
                    ret_type: Type::Int,
                    body: box Expr::If(
                        box Expr::Var(Ident::new("x"), Type::Bool, Position { start: 0, end: 1 }),
                        box Expr::Var(Ident::new("y"), Type::Int, Position { start: 0, end: 2 }),
                        box Expr::Const(Literal::Number(42)),
                        Position { start: 0, end: 3 }
                    ),
                    left: box Expr::Var(
                        func_name2.clone(),
                        Type::Func(box Type::Int, box Type::Int),
                        Position { start: 0, end: 0 }
                    ),
                    pos: Position { start: 0, end: 0 }
                },
                left: box Expr::Var(
                    func_name1.clone(),
                    Type::Func(box Type::Bool, box Type::Func(box Type::Int, box Type::Int)),
                    Position { start: 0, end: 0 }
                ),
                pos: Position { start: 0, end: 0 },
            },
            Type::Func(box Type::Bool, box Type::Int)
        ))
    );
}
