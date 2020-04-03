use ast::{parsed::*, BinOp};
use ident::Ident;
use parser::*;

#[test]
fn primitive_literal() {
    assert_eq!(
        program("123"),
        Ok((vec![], Expr::Const(Literal::Number(123))))
    );
    assert_eq!(
        program("true"),
        Ok((vec![], Expr::Const(Literal::Bool(true))))
    );
    assert_eq!(
        program("false"),
        Ok((vec![], Expr::Const(Literal::Bool(false))))
    );
    assert_eq!(program("unit"), Ok((vec![], Expr::Const(Literal::Unit))));
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
                    Type::Int,
                    box Expr::Var(Ident::new("x"), Position { start: 15, end: 16 }),
                ),
                box Expr::Const(Literal::Number(1)),
                Position { start: 0, end: 19 }
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
                Expr::Const(Literal::Number(1)),
                Expr::Const(Literal::Number(2)),
                Expr::Const(Literal::Number(3))
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
                box Expr::Const(Literal::Bool(true)),
                box Expr::Const(Literal::Number(1)),
                box Expr::Const(Literal::Number(2)),
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
                    box Expr::Const(Literal::Number(1)),
                    box Expr::BinOp(
                        BinOp::Mult,
                        box Expr::Const(Literal::Number(2)),
                        box Expr::Const(Literal::Number(5)),
                        Position { start: 3, end: 4 }
                    ),
                    Position { start: 1, end: 2 }
                ),
                box Expr::Const(Literal::Number(6)),
                Position { start: 5, end: 6 }
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
                box Expr::Const(Literal::Number(1)),
                box Expr::Const(Literal::Number(2)),
                Position { start: 2, end: 4 }
            )
        ))
    );
    assert_eq!(
        program("1 > 2"),
        Ok((
            vec![],
            Expr::BinOp(
                BinOp::Gt,
                box Expr::Const(Literal::Number(1)),
                box Expr::Const(Literal::Number(2)),
                Position { start: 2, end: 4 }
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
            Expr::Const(Literal::Record(vec![
                (Ident::new("id"), Expr::Const(Literal::Number(42))),
                (Ident::new("value"), Expr::Const(Literal::Number(123))),
            ],))
        ))
    );
}

#[test]
fn tuple() {
    assert_eq!(
        program("(1, 2, 3)"),
        Ok((
            vec![],
            Expr::Const(Literal::Tuple(vec![
                Expr::Const(Literal::Number(1)),
                Expr::Const(Literal::Number(2)),
                Expr::Const(Literal::Number(3)),
            ],))
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
                box Expr::Const(Literal::Record(vec![(
                    Ident::new("id"),
                    Expr::Const(Literal::Number(42))
                )],)),
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
            vec![ToplevelExpr::LetType(LetType {
                name: Ident::new("Nyan"),
                typ: Type::Variant(vec![
                    (Ident::new("Hoge"), Type::Int),
                    (Ident::new("Fuga"), Type::Bool)
                ]),
            },)],
            Expr::Const(Literal::Variant(
                Ident::new("Hoge"),
                box Expr::Const(Literal::Number(42)),
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
            vec![ToplevelExpr::LetType(LetType {
                name: Ident::new("Nyan"),
                typ: Type::Variant(vec![
                    (Ident::new("Hoge"), Type::Int),
                    (Ident::new("Fuga"), Type::Bool)
                ]),
            })],
            Expr::PatternMatch(
                box Expr::Const(Literal::Variant(
                    Ident::new("Hoge"),
                    box Expr::Const(Literal::Number(42)),
                    Type::Var(Ident::new("Nyan"), Position { start: 58, end: 62 }),
                    Position { start: 58, end: 73 }
                )),
                vec![
                    PatternMatchArm {
                        label: Ident::new("Hoge"),
                        name: Ident::new("x"),
                        body: Expr::BinOp(
                            BinOp::Add,
                            box Expr::Var(Ident::new("x"), Position { start: 89, end: 91 }),
                            box Expr::Const(Literal::Number(1)),
                            Position { start: 91, end: 93 }
                        ),
                    },
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
                            box Expr::Const(Literal::Number(100)),
                            box Expr::Const(Literal::Number(200)),
                            Position {
                                start: 110,
                                end: 136
                            }
                        ),
                    },
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
                Expr::Println(box Expr::Const(Literal::Number(1))),
                Expr::Println(box Expr::Const(Literal::Bool(true))),
                Expr::Println(box Expr::Const(Literal::Unit)),
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
            vec![ToplevelExpr::Func(Func {
                name: Ident::new("f"),
                param_name: Ident::new("a"),
                param_type: Type::Int,
                body: Expr::BinOp(
                    BinOp::Add,
                    box Expr::Var(Ident::new("a"), Position { start: 20, end: 22 }),
                    box Expr::Const(Literal::Number(12)),
                    Position { start: 22, end: 24 }
                ),
            })],
            Expr::Apply(
                box Expr::Var(Ident::new("f"), Position { start: 29, end: 31 }),
                box Expr::Const(Literal::Number(13)),
                Position { start: 29, end: 34 }
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
            vec![ToplevelExpr::RecFunc(RecFunc {
                name: Ident::new("fib"),
                param_name: Ident::new("x"),
                param_type: Type::Int,
                ret_type: Type::Int,
                body: Expr::If(
                    box Expr::BinOp(
                        BinOp::Lt,
                        box Expr::Var(Ident::new("x"), Position { start: 34, end: 36 }),
                        box Expr::Const(Literal::Number(2)),
                        Position { start: 36, end: 38 }
                    ),
                    box Expr::Const(Literal::Number(1)),
                    box Expr::BinOp(
                        BinOp::Add,
                        box Expr::Apply(
                            box Expr::Var(Ident::new("fib"), Position { start: 73, end: 77 }),
                            box Expr::BinOp(
                                BinOp::Sub,
                                box Expr::Var(Ident::new("x"), Position { start: 78, end: 79 }),
                                box Expr::Const(Literal::Number(1)),
                                Position { start: 79, end: 80 }
                            ),
                            Position { start: 73, end: 83 }
                        ),
                        box Expr::Apply(
                            box Expr::Var(Ident::new("fib"), Position { start: 85, end: 89 }),
                            box Expr::BinOp(
                                BinOp::Sub,
                                box Expr::Var(Ident::new("x"), Position { start: 90, end: 91 }),
                                box Expr::Const(Literal::Number(2)),
                                Position { start: 91, end: 92 }
                            ),
                            Position { start: 85, end: 99 }
                        ),
                        Position { start: 83, end: 85 }
                    ),
                    Position {
                        start: 31,
                        end: 101
                    }
                ),
            })],
            Expr::Apply(
                box Expr::Var(
                    Ident::new("fib"),
                    Position {
                        start: 103,
                        end: 107
                    }
                ),
                box Expr::Const(Literal::Number(3)),
                Position {
                    start: 103,
                    end: 109
                }
            )
        ))
    );
}

#[test]
fn let_type_func() {
    assert_eq!(
        program("type a = Int; 42"),
        Ok((
            vec![ToplevelExpr::LetType(LetType {
                name: Ident::new("a"),
                typ: Type::Int,
            })],
            Expr::Const(Literal::Number(42))
        ))
    );
}
