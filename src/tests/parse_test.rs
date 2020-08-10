use ast::{self, Expr::*, Literal::*, Position, Program, Type};
use ident::Ident;
use parser::*;

macro_rules! hashmap(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

#[test]
fn primitive_literal() {
    assert_eq!(
        program("123"),
        Ok(Program {
            imports: vec![],
            expr: Const(Number(123)),
        })
    );
    assert_eq!(
        program("true"),
        Ok(Program {
            imports: vec![],
            expr: Const(Bool(true)),
        })
    );
    assert_eq!(
        program("false"),
        Ok(Program {
            imports: vec![],
            expr: Const(Bool(false)),
        })
    );
    assert_eq!(
        program("unit"),
        Ok(Program {
            imports: vec![],
            expr: Const(Unit),
        })
    );
    assert_eq!(
        program("a"),
        Ok(Program {
            imports: vec![],
            expr: Var(
                Ident::new("a"),
                Type::Var(Ident::new("<fresh-expected>")),
                Position { start: 0, end: 1 }
            )
        })
    );
}

#[test]
fn apply() {
    assert_eq!(
        program("(func x:Int :Int => x) 1"),
        Ok(Program {
            imports: vec![],
            expr: Apply(
                box Func {
                    name: Ident::new("<fresh-expected>"),
                    param_name: Ident::new("x"),
                    param_type: Type::Int,
                    ret_type: Type::Int,
                    body: box Var(
                        Ident::new("x"),
                        Type::Var(Ident::new("<fresh-expected>")),
                        Position { start: 20, end: 21 }
                    ),
                    left: box Var(
                        Ident::new("<fresh-expected>"),
                        Type::Func(box Type::Int, box Type::Int),
                        Position { start: 1, end: 21 }
                    ),
                    pos: Position { start: 1, end: 21 }
                },
                box Const(Number(1)),
                Position { start: 0, end: 24 }
            )
        })
    );
}

#[test]
fn sequence() {
    assert_eq!(
        program("1; 2; 3"),
        Ok(Program {
            imports: vec![],
            expr: Let(
                Ident::new("<dummy-sequence>"),
                Type::Var(Ident::new("<fresh-expected>")),
                box Const(Number(1)),
                box Let(
                    Ident::new("<dummy-sequence>"),
                    Type::Var(Ident::new("<fresh-expected>")),
                    box Const(Number(2)),
                    box Const(Number(3)),
                    Position { start: 3, end: 4 }
                ),
                Position { start: 0, end: 1 }
            )
        })
    );
}

#[test]
fn if_() {
    assert_eq!(
        program("if true { 1 } else { 2 }"),
        Ok(Program {
            imports: vec![],
            expr: If(
                box Const(Bool(true)),
                box Const(Number(1)),
                box Const(Number(2)),
                Position { start: 0, end: 24 }
            )
        })
    );
}

#[test]
fn arithmetic() {
    assert_eq!(
        program("1+2*5+6"),
        Ok(Program {
            imports: vec![],
            expr: BinOp(
                ast::BinOp::Add,
                box BinOp(
                    ast::BinOp::Add,
                    box Const(Number(1)),
                    box BinOp(
                        ast::BinOp::Mult,
                        box Const(Number(2)),
                        box Const(Number(5)),
                        Position { start: 3, end: 4 }
                    ),
                    Position { start: 1, end: 2 }
                ),
                box Const(Number(6)),
                Position { start: 5, end: 6 }
            )
        })
    );
}

#[test]
fn compare() {
    assert_eq!(
        program("1 < 2"),
        Ok(Program {
            imports: vec![],
            expr: BinOp(
                ast::BinOp::Lt,
                box Const(Number(1)),
                box Const(Number(2)),
                Position { start: 2, end: 4 }
            )
        })
    );
    assert_eq!(
        program("1 > 2"),
        Ok(Program {
            imports: vec![],
            expr: BinOp(
                ast::BinOp::Gt,
                box Const(Number(1)),
                box Const(Number(2)),
                Position { start: 2, end: 4 }
            )
        })
    );
}

#[test]
fn record() {
    assert_eq!(
        program("{ id=42, value=123 }"),
        Ok(Program {
            imports: vec![],
            expr: Const(Record(hashmap! {
                Ident::new("id") => Const(Number(42)),
                Ident::new("value") => Const(Number(123))
            }))
        })
    );
}

#[test]
fn tuple() {
    assert_eq!(
        program("(1, 2, 3)"),
        Ok(Program {
            imports: vec![],
            expr: Const(Record(hashmap! {
                Ident::new("0") => Const(Number(1)),
                Ident::new("1") => Const(Number(2)),
                Ident::new("2") => Const(Number(3))
            }))
        })
    );
}

#[test]
fn record_get() {
    assert_eq!(
        program("{id=42}.id"),
        Ok(Program {
            imports: vec![],
            expr: RecordGet(
                box Const(Record(hashmap! {Ident::new("id") => Const(Number(42))})),
                Type::Var(Ident::new("<fresh-expected>")),
                Ident::new("id"),
                Position { start: 0, end: 10 }
            )
        })
    );
}

#[test]
fn record_assign() {
    assert_eq!(
        program("let x = {hoge=12, fuga=32}; x.hoge <- 42"),
        Ok(Program {
            imports: vec![],
            expr: Let(
                Ident::new("x"),
                Type::Var(Ident::new("<fresh-expected>")),
                box Const(Record(hashmap! {
                    Ident::new("hoge") => Const(Number(12)),
                    Ident::new("fuga") => Const(Number(32))
                })),
                box Assign(
                    box RecordGet(
                        box Var(
                            Ident::new("x"),
                            Type::Var(Ident::new("<fresh-expected>")),
                            Position { start: 28, end: 29 }
                        ),
                        Type::Var(Ident::new("<fresh-expected>")),
                        Ident::new("hoge"),
                        Position { start: 28, end: 35 }
                    ),
                    box Const(Number(42)),
                    Position { start: 35, end: 38 }
                ),
                Position { start: 0, end: 28 }
            ),
        })
    );
}

#[test]
fn func() {
    assert_eq!(
        program(
            r#"
func f a:Int :Int {
    a + 12
}
f 13
"#
        ),
        Ok(Program {
            imports: vec![],
            expr: Func {
                name: Ident::new("f"),
                param_name: Ident::new("a"),
                param_type: Type::Int,
                ret_type: Type::Int,
                body: box BinOp(
                    ast::BinOp::Add,
                    box Var(
                        Ident::new("a"),
                        Type::Var(Ident::new("<fresh-expected>")),
                        Position { start: 25, end: 27 }
                    ),
                    box Const(Number(12)),
                    Position { start: 27, end: 29 }
                ),
                left: box Apply(
                    box Var(
                        Ident::new("f"),
                        Type::Var(Ident::new("<fresh-expected>")),
                        Position { start: 34, end: 36 }
                    ),
                    box Const(Number(13)),
                    Position { start: 34, end: 39 }
                ),
                pos: Position { start: 1, end: 34 }
            },
        })
    );
}

#[test]
fn rec_func() {
    assert_eq!(
        program(
            r#"
func fib x:Int :Int {
    if x < 2 {
        1
    } else {
        fib (x-1) + fib (x-2)
    }
}
fib 3
"#
        ),
        Ok(Program {
            imports: vec![],
            expr: Func {
                name: Ident::new("fib"),
                param_name: Ident::new("x"),
                param_type: Type::Int,
                ret_type: Type::Int,
                body: box If(
                    box BinOp(
                        ast::BinOp::Lt,
                        box Var(
                            Ident::new("x"),
                            Type::Var(Ident::new("<fresh-expected>")),
                            Position { start: 30, end: 32 }
                        ),
                        box Const(Number(2)),
                        Position { start: 32, end: 34 }
                    ),
                    box Const(Number(1)),
                    box BinOp(
                        ast::BinOp::Add,
                        box Apply(
                            box Var(
                                Ident::new("fib"),
                                Type::Var(Ident::new("<fresh-expected>")),
                                Position { start: 69, end: 73 }
                            ),
                            box BinOp(
                                ast::BinOp::Sub,
                                box Var(
                                    Ident::new("x"),
                                    Type::Var(Ident::new("<fresh-expected>")),
                                    Position { start: 74, end: 75 }
                                ),
                                box Const(Number(1)),
                                Position { start: 75, end: 76 }
                            ),
                            Position { start: 69, end: 79 }
                        ),
                        box Apply(
                            box Var(
                                Ident::new("fib"),
                                Type::Var(Ident::new("<fresh-expected>")),
                                Position { start: 81, end: 85 }
                            ),
                            box BinOp(
                                ast::BinOp::Sub,
                                box Var(
                                    Ident::new("x"),
                                    Type::Var(Ident::new("<fresh-expected>")),
                                    Position { start: 86, end: 87 }
                                ),
                                box Const(Number(2)),
                                Position { start: 87, end: 88 }
                            ),
                            Position { start: 81, end: 95 }
                        ),
                        Position { start: 79, end: 81 }
                    ),
                    Position { start: 27, end: 97 }
                ),
                left: box Apply(
                    box Var(
                        Ident::new("fib"),
                        Type::Var(Ident::new("<fresh-expected>")),
                        Position {
                            start: 99,
                            end: 103
                        }
                    ),
                    box Const(Number(3)),
                    Position {
                        start: 99,
                        end: 105
                    }
                ),
                pos: Position { start: 1, end: 99 }
            }
        })
    );
}

#[test]
fn let_type_func() {
    assert_eq!(
        program("type a = Int; 42"),
        Ok(Program {
            imports: vec![],
            expr: LetType(Ident::new("a"), Type::Int, box Const(Number(42)))
        })
    );
}
