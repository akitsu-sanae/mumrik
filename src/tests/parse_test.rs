use ast::{parsed::*, BinOp};
use ident::Ident;
use parser::*;

#[test]
fn primitive_literal() {
    assert_eq!(
        program("123"),
        Ok((
            vec![],
            Expr::Const(Literal::Number(123, Position { start: 0, end: 3 }))
        ))
    );
    assert_eq!(
        program("true"),
        Ok((
            vec![],
            Expr::Const(Literal::Bool(true, Position { start: 0, end: 4 }))
        ))
    );
    assert_eq!(
        program("false"),
        Ok((
            vec![],
            Expr::Const(Literal::Bool(false, Position { start: 0, end: 5 }))
        ))
    );
    assert_eq!(
        program("unit"),
        Ok((
            vec![],
            Expr::Const(Literal::Unit(Position { start: 0, end: 4 }))
        ))
    );
    assert_eq!(
        program("a"),
        Ok((
            vec![],
            Expr::Var(Ident::new("a"), Position { start: 0, end: 1 })
        ))
    );
}

#[test]
fn apply() {
    assert_eq!(
        program("(func x:Int => x) 1"),
        Ok((
            vec![],
            Expr::Apply(
                box Expr::Lambda(
                    Ident::new("x"),
                    Type::Int(Position { start: 8, end: 12 }),
                    box Expr::Var(Ident::new("x"), Position { start: 15, end: 16 }),
                    Position { start: 1, end: 16 }
                ),
                box Expr::Const(Literal::Number(1, Position { start: 18, end: 19 }))
            )
        ))
    );
}

#[test]
fn sequence() {
    assert_eq!(
        program("1; 2; 3"),
        Ok((
            vec![],
            Expr::Sequence(vec![
                Expr::Const(Literal::Number(1, Position { start: 0, end: 1 })),
                Expr::Const(Literal::Number(2, Position { start: 3, end: 4 })),
                Expr::Const(Literal::Number(3, Position { start: 6, end: 7 }))
            ])
        ))
    );
}

#[test]
fn if_() {
    assert_eq!(
        program("if true { 1 } else { 2 }"),
        Ok((
            vec![],
            Expr::If(
                box Expr::Const(Literal::Bool(true, Position { start: 3, end: 8 })),
                box Expr::Const(Literal::Number(1, Position { start: 10, end: 12 })),
                box Expr::Const(Literal::Number(2, Position { start: 21, end: 23 })),
                Position { start: 0, end: 24 }
            )
        ))
    );
}

#[test]
fn arithmetic() {
    assert_eq!(
        program("1+2*5+6"),
        Ok((
            vec![],
            Expr::BinOp(
                BinOp::Add,
                box Expr::BinOp(
                    BinOp::Add,
                    box Expr::Const(Literal::Number(1, Position { start: 0, end: 1 })),
                    box Expr::BinOp(
                        BinOp::Mult,
                        box Expr::Const(Literal::Number(2, Position { start: 2, end: 3 })),
                        box Expr::Const(Literal::Number(5, Position { start: 4, end: 5 }))
                    )
                ),
                box Expr::Const(Literal::Number(6, Position { start: 6, end: 7 }))
            )
        ))
    );
}

#[test]
fn compare() {
    assert_eq!(
        program("1 < 2"),
        Ok((
            vec![],
            Expr::BinOp(
                BinOp::Lt,
                box Expr::Const(Literal::Number(1, Position { start: 0, end: 2 })),
                box Expr::Const(Literal::Number(2, Position { start: 4, end: 5 }))
            )
        ))
    );
    assert_eq!(
        program("1 > 2"),
        Ok((
            vec![],
            Expr::BinOp(
                BinOp::Gt,
                box Expr::Const(Literal::Number(1, Position { start: 0, end: 2 })),
                box Expr::Const(Literal::Number(2, Position { start: 4, end: 5 }))
            )
        ))
    );
}

#[test]
fn record() {
    assert_eq!(
        program("{ id=42, value=123 }"),
        Ok((
            vec![],
            Expr::Const(Literal::Record(
                vec![
                    (
                        Ident::new("id"),
                        Expr::Const(Literal::Number(42, Position { start: 5, end: 7 }))
                    ),
                    (
                        Ident::new("value"),
                        Expr::Const(Literal::Number(123, Position { start: 15, end: 19 }))
                    ),
                ],
                Position { start: 0, end: 20 }
            ))
        ))
    );
}

#[test]
fn tuple() {
    assert_eq!(
        program("(1, 2, 3)"),
        Ok((
            vec![],
            Expr::Const(Literal::Tuple(
                vec![
                    Expr::Const(Literal::Number(1, Position { start: 1, end: 2 })),
                    Expr::Const(Literal::Number(2, Position { start: 4, end: 5 })),
                    Expr::Const(Literal::Number(3, Position { start: 7, end: 8 })),
                ],
                Position { start: 0, end: 9 }
            ))
        ))
    );
}

#[test]
fn field_access() {
    assert_eq!(
        program("{id=42}.id"),
        Ok((
            vec![],
            Expr::FieldAccess(
                box Expr::Const(Literal::Record(
                    vec![(
                        Ident::new("id"),
                        Expr::Const(Literal::Number(42, Position { start: 4, end: 6 }))
                    )],
                    Position { start: 0, end: 7 }
                )),
                Ident::new("id"),
                Position { start: 0, end: 10 }
            )
        ))
    );
}

#[test]
fn variant() {
    assert_eq!(
        program(
            r#"
type Nyan = enum {
    Hoge: Int,
    Fuga: Bool,
};
Nyan::Hoge(42)"#
        ),
        Ok((
            vec![ToplevelExpr::LetType(
                LetType {
                    name: Ident::new("Nyan"),
                    typ: Type::Variant(
                        vec![
                            (
                                Ident::new("Hoge"),
                                Type::Int(Position { start: 30, end: 33 })
                            ),
                            (
                                Ident::new("Fuga"),
                                Type::Bool(Position { start: 45, end: 49 })
                            )
                        ],
                        Position { start: 13, end: 52 }
                    ),
                },
                Position { start: 1, end: 54 }
            )],
            Expr::Const(Literal::Variant(
                Ident::new("Hoge"),
                box Expr::Const(Literal::Number(42, Position { start: 65, end: 67 })),
                Type::Var(Ident::new("Nyan"), Position { start: 54, end: 58 }),
                Position { start: 54, end: 68 }
            ))
        ))
    );
}

#[test]
fn match_() {
    assert_eq!(
        program(
            r#"
type Nyan = enum {
    Hoge: Int
    Fuga: Bool
};
match Nyan::Hoge(42) {
    Hoge x => x + 1,
    Fuga x => if x { 100 } else { 200 }
}
"#
        ),
        Ok((
            vec![ToplevelExpr::LetType(
                LetType {
                    name: Ident::new("Nyan"),
                    typ: Type::Variant(
                        vec![
                            (
                                Ident::new("Hoge"),
                                Type::Int(Position { start: 30, end: 38 })
                            ),
                            (
                                Ident::new("Fuga"),
                                Type::Bool(Position { start: 44, end: 49 })
                            )
                        ],
                        Position { start: 13, end: 50 }
                    ),
                },
                Position { start: 1, end: 52 }
            )],
            Expr::PatternMatch(
                box Expr::Const(Literal::Variant(
                    Ident::new("Hoge"),
                    box Expr::Const(Literal::Number(42, Position { start: 69, end: 71 })),
                    Type::Var(Ident::new("Nyan"), Position { start: 58, end: 62 }),
                    Position { start: 58, end: 73 }
                )),
                vec![
                    (
                        PatternMatchArm {
                            label: Ident::new("Hoge"),
                            name: Ident::new("x"),
                            body: Expr::BinOp(
                                BinOp::Add,
                                box Expr::Var(Ident::new("x"), Position { start: 89, end: 91 }),
                                box Expr::Const(Literal::Number(
                                    1,
                                    Position { start: 93, end: 94 }
                                ))
                            ),
                        },
                        Position {
                            start: 79,
                            end: 100
                        }
                    ),
                    (
                        PatternMatchArm {
                            label: Ident::new("Fuga"),
                            name: Ident::new("x"),
                            body: Expr::If(
                                box Expr::Var(
                                    Ident::new("x"),
                                    Position {
                                        start: 113,
                                        end: 115
                                    }
                                ),
                                box Expr::Const(Literal::Number(
                                    100,
                                    Position {
                                        start: 117,
                                        end: 121
                                    }
                                )),
                                box Expr::Const(Literal::Number(
                                    200,
                                    Position {
                                        start: 130,
                                        end: 134
                                    }
                                )),
                                Position {
                                    start: 110,
                                    end: 136
                                }
                            ),
                        },
                        Position {
                            start: 100,
                            end: 136
                        }
                    )
                ],
                Position {
                    start: 52,
                    end: 138
                }
            )
        ))
    );
}

#[test]
fn println() {
    assert_eq!(
        program("println 1; println true; println unit"),
        Ok((
            vec![],
            Expr::Sequence(vec![
                Expr::Println(
                    box Expr::Const(Literal::Number(1, Position { start: 8, end: 9 })),
                    Position { start: 0, end: 9 }
                ),
                Expr::Println(
                    box Expr::Const(Literal::Bool(true, Position { start: 19, end: 23 })),
                    Position { start: 11, end: 23 }
                ),
                Expr::Println(
                    box Expr::Const(Literal::Unit(Position { start: 33, end: 37 })),
                    Position { start: 25, end: 37 }
                ),
            ])
        ))
    );
}

#[test]
fn func() {
    assert_eq!(
        program(
            r#"
func f a:Int {
    a + 12
}
f 13
"#
        ),
        Ok((
            vec![ToplevelExpr::Func(
                Func {
                    name: Ident::new("f"),
                    param_name: Ident::new("a"),
                    param_type: Type::Int(Position { start: 10, end: 14 }),
                    body: Expr::BinOp(
                        BinOp::Add,
                        box Expr::Var(Ident::new("a"), Position { start: 20, end: 22 }),
                        box Expr::Const(Literal::Number(12, Position { start: 24, end: 27 }))
                    ),
                },
                Position { start: 1, end: 29 }
            )],
            Expr::Apply(
                box Expr::Var(Ident::new("f"), Position { start: 29, end: 31 }),
                box Expr::Const(Literal::Number(13, Position { start: 31, end: 34 }))
            )
        ))
    );
}

#[test]
fn rec_func() {
    assert_eq!(
        program(
            r#"
rec func fib x:Int :Int {
    if x < 2 {
        1
    } else {
        fib (x-1) + fib (x-2)
    }
}
fib 3
"#
        ),
        Ok((
            vec![ToplevelExpr::RecFunc(
                RecFunc {
                    name: Ident::new("fib"),
                    param_name: Ident::new("x"),
                    param_type: Type::Int(Position { start: 16, end: 20 }),
                    ret_type: Type::Int(Position { start: 21, end: 25 }),
                    body: Expr::If(
                        box Expr::BinOp(
                            BinOp::Lt,
                            box Expr::Var(Ident::new("x"), Position { start: 34, end: 36 }),
                            box Expr::Const(Literal::Number(2, Position { start: 38, end: 40 }))
                        ),
                        box Expr::Const(Literal::Number(1, Position { start: 50, end: 56 })),
                        box Expr::BinOp(
                            BinOp::Add,
                            box Expr::Apply(
                                box Expr::Var(Ident::new("fib"), Position { start: 73, end: 77 }),
                                box Expr::BinOp(
                                    BinOp::Sub,
                                    box Expr::Var(Ident::new("x"), Position { start: 78, end: 79 }),
                                    box Expr::Const(Literal::Number(
                                        1,
                                        Position { start: 80, end: 81 }
                                    ))
                                )
                            ),
                            box Expr::Apply(
                                box Expr::Var(Ident::new("fib"), Position { start: 85, end: 89 }),
                                box Expr::BinOp(
                                    BinOp::Sub,
                                    box Expr::Var(Ident::new("x"), Position { start: 90, end: 91 }),
                                    box Expr::Const(Literal::Number(
                                        2,
                                        Position { start: 92, end: 93 }
                                    ))
                                )
                            )
                        ),
                        Position {
                            start: 31,
                            end: 101
                        }
                    ),
                },
                Position { start: 1, end: 103 }
            )],
            Expr::Apply(
                box Expr::Var(
                    Ident::new("fib"),
                    Position {
                        start: 103,
                        end: 107
                    }
                ),
                box Expr::Const(Literal::Number(
                    3,
                    Position {
                        start: 107,
                        end: 109
                    }
                ))
            )
        ))
    );
}

#[test]
fn let_type_func() {
    assert_eq!(
        program("type a = Int; 42"),
        Ok((
            vec![ToplevelExpr::LetType(
                LetType {
                    name: Ident::new("a"),
                    typ: Type::Int(Position { start: 9, end: 12 }),
                },
                Position { start: 0, end: 14 }
            )],
            Expr::Const(Literal::Number(42, Position { start: 14, end: 16 }))
        ))
    );
}
