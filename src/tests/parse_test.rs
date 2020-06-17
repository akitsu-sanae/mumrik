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
fn field_access() {
    assert_eq!(
        program("{id=42}.id"),
        Ok(Program {
            imports: vec![],
            expr: FieldAccess(
                box Const(Record(hashmap! {Ident::new("id") => Const(Number(42))})),
                Type::Var(Ident::new("<fresh-expected>")),
                Ident::new("id"),
                Position { start: 0, end: 10 }
            )
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
            expr: Let(
                Ident::new("f"),
                Type::Func(box Type::Int, box Type::Int),
                box Func {
                    name: Ident::new("<fresh-expected>"),
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
                    left: box Var(
                        Ident::new("<fresh-expected>"),
                        Type::Func(box Type::Int, box Type::Int),
                        Position { start: 1, end: 34 }
                    ),
                    pos: Position { start: 1, end: 34 },
                },
                box Apply(
                    box Var(
                        Ident::new("f"),
                        Type::Var(Ident::new("<fresh-expected>")),
                        Position { start: 34, end: 36 }
                    ),
                    box Const(Number(13)),
                    Position { start: 34, end: 39 }
                ),
                Position { start: 1, end: 34 }
            )
        })
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
                            Position { start: 34, end: 36 }
                        ),
                        box Const(Number(2)),
                        Position { start: 36, end: 38 }
                    ),
                    box Const(Number(1)),
                    box BinOp(
                        ast::BinOp::Add,
                        box Apply(
                            box Var(
                                Ident::new("fib"),
                                Type::Var(Ident::new("<fresh-expected>")),
                                Position { start: 73, end: 77 }
                            ),
                            box BinOp(
                                ast::BinOp::Sub,
                                box Var(
                                    Ident::new("x"),
                                    Type::Var(Ident::new("<fresh-expected>")),
                                    Position { start: 78, end: 79 }
                                ),
                                box Const(Number(1)),
                                Position { start: 79, end: 80 }
                            ),
                            Position { start: 73, end: 83 }
                        ),
                        box Apply(
                            box Var(
                                Ident::new("fib"),
                                Type::Var(Ident::new("<fresh-expected>")),
                                Position { start: 85, end: 89 }
                            ),
                            box BinOp(
                                ast::BinOp::Sub,
                                box Var(
                                    Ident::new("x"),
                                    Type::Var(Ident::new("<fresh-expected>")),
                                    Position { start: 90, end: 91 }
                                ),
                                box Const(Number(2)),
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
                left: box Apply(
                    box Var(
                        Ident::new("fib"),
                        Type::Var(Ident::new("<fresh-expected>")),
                        Position {
                            start: 103,
                            end: 107
                        }
                    ),
                    box Const(Number(3)),
                    Position {
                        start: 103,
                        end: 109
                    }
                ),
                pos: Position { start: 1, end: 103 }
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
