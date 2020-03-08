use crate::{
    expr::{BinOp, Expr::*, Literal::*},
    parser::*,
};
use type_::Type;

#[test]
fn primitive_literal() {
    assert_eq!(expr("123"), Ok(Const(Number(123))));
    assert_eq!(expr("true"), Ok(Const(Bool(true))));
    assert_eq!(expr("false"), Ok(Const(Bool(false))));
    assert_eq!(expr("unit"), Ok(Const(Unit)));
    assert_eq!(expr("a"), Ok(Var("a".to_string())));
}

#[test]
fn apply() {
    let e = expr("(func x: Int => x) 1");
    assert_eq!(
        e,
        Ok(Apply(
            box Lambda("x".to_string(), box Type::Int, box Var("x".to_string())),
            box Const(Number(1))
        ))
    );
}

#[test]
fn sequence() {
    let e = expr("1; 2; 3");
    assert_eq!(
        e,
        Ok(Let(
            "<dummy>".to_string(),
            box Const(Number(1)),
            box Let(
                "<dummy>".to_string(),
                box Const(Number(2)),
                box Const(Number(3))
            )
        ))
    );
}

#[test]
fn if_() {
    let e = expr("if true { 1 } else { 2 }");
    assert_eq!(
        e,
        Ok(If(
            box Const(Bool(true)),
            box Const(Number(1)),
            box Const(Number(2))
        ))
    );
}

#[test]
fn arithmetic() {
    let e = expr("1+2*5+6");
    assert_eq!(
        e,
        Ok(BinOp(
            BinOp::Add,
            box BinOp(
                BinOp::Add,
                box Const(Number(1)),
                box BinOp(BinOp::Mult, box Const(Number(2)), box Const(Number(5)))
            ),
            box Const(Number(6))
        ))
    );
}

#[test]
fn compare() {
    let e = expr("1 < 2");
    assert_eq!(
        e,
        Ok(BinOp(
            BinOp::LessThan,
            box Const(Number(1)),
            box Const(Number(2))
        ))
    );

    let e = expr("1 > 2");
    assert_eq!(
        e,
        Ok(BinOp(
            BinOp::GreaterThan,
            box Const(Number(1)),
            box Const(Number(2))
        ))
    );
}

#[test]
fn record() {
    let e = expr("{ id=42, value=123 }");
    assert_eq!(
        e,
        Ok(Const(Record(vec![
            ("id".to_string(), box Const(Number(42))),
            ("value".to_string(), box Const(Number(123)))
        ])))
    );
}

#[test]
fn tuple() {
    let e = expr("(1, 2, 3)");
    assert_eq!(
        e,
        Ok(Const(Record(vec![
            ("0".to_string(), box Const(Number(1))),
            ("1".to_string(), box Const(Number(2))),
            ("2".to_string(), box Const(Number(3)))
        ])))
    );
}

#[test]
fn dot() {
    let e = expr("{ id=42, value=123 }.id");
    assert_eq!(
        e,
        Ok(Dot(
            box Const(Record(vec![
                ("id".to_string(), box Const(Number(42))),
                ("value".to_string(), box Const(Number(123)))
            ])),
            "id".to_string()
        ))
    );
}

#[test]
fn variant() {
    let expr = expr("type Nyan = enum { Hoge: Int, Fuga: Bool}; Nyan::Hoge(42)").unwrap();
    let nyan_ty = Type::Variant(vec![
        ("Hoge".to_string(), Type::Int),
        ("Fuga".to_string(), Type::Bool),
    ]);
    assert_eq!(
        expr,
        LetType(
            "Nyan".to_string(),
            box nyan_ty,
            box Const(Variant(
                "Hoge".to_string(),
                box Const(Number(42)),
                box Type::Variable("Nyan".to_string()),
            )),
        )
    );
}

#[test]
fn list() {
    let e = expr("[1, 2, 3]");
    assert_eq!(
        e,
        Ok(Const(List(vec![
            Const(Number(1)),
            Const(Number(2)),
            Const(Number(3))
        ])))
    );
}

#[test]
fn string() {
    let e = expr("\"nyan\"").unwrap();
    assert_eq!(
        e,
        Const(List(vec![
            Const(Char('n')),
            Const(Char('y')),
            Const(Char('a')),
            Const(Char('n'))
        ]))
    );
}

#[test]
fn match_() {
    let expr = expr("type Nyan = enum { Hoge: Int Fuga: Bool}; match Nyan::Hoge(42) { Hoge x => x+1, Fuga x => if x { 100 } else { 200 } }");
    let nyan_ty = Type::Variant(vec![
        ("Hoge".to_string(), Type::Int),
        ("Fuga".to_string(), Type::Bool),
    ]);
    let hoge_branch = (
        "Hoge".to_string(),
        "x".to_string(),
        box BinOp(BinOp::Add, box Var("x".to_string()), box Const(Number(1))),
    );
    let fuga_branch = (
        "Fuga".to_string(),
        "x".to_string(),
        box If(
            box Var("x".to_string()),
            box Const(Number(100)),
            box Const(Number(200)),
        ),
    );

    assert_eq!(
        expr,
        Ok(LetType(
            "Nyan".to_string(),
            box nyan_ty,
            box Match(
                box Const(Variant(
                    "Hoge".to_string(),
                    box Const(Number(42)),
                    box Type::Variable("Nyan".to_string())
                )),
                vec![hoge_branch, fuga_branch]
            )
        ))
    );
}

#[test]
fn println() {
    let e = expr("println 1; println true; println unit");
    assert_eq!(
        e,
        Ok(Let(
            "<dummy>".to_string(),
            box Println(box Const(Number(1))),
            box Let(
                "<dummy>".to_string(),
                box Println(box Const(Bool(true))),
                box Println(box Const(Unit))
            )
        ))
    );
}

#[test]
fn func() {
    let e = expr("func f a:Int { a+12 } f 13");
    assert_eq!(
        e,
        Ok(Let(
            "f".to_string(),
            box Lambda(
                "a".to_string(),
                box Type::Int,
                box BinOp(BinOp::Add, box Var("a".to_string()), box Const(Number(12)))
            ),
            box Apply(box Var("f".to_string()), box Const(Number(13)))
        ))
    );
}

#[test]
fn rec_func() {
    // let e = expr("rec let fib: Int -> Int = func x:Int => if x < 2 { 1 } else { (fib (x-1)) + (fib (x-2) }; fib 8").unwrap();
    let e =
        expr("rec func fib x:Int :Int { if x < 2 { 1 } else { (fib (x-1)) + (fib (x-2)) } } fib 3");
    assert_eq!(
        e,
        Ok(LetRec(
            "fib".to_string(),
            box Type::Function(box Type::Int, box Type::Int),
            box Lambda(
                "x".to_string(),
                box Type::Int,
                box If(
                    box BinOp(
                        BinOp::LessThan,
                        box Var("x".to_string()),
                        box Const(Number(2))
                    ),
                    box Const(Number(1)),
                    box BinOp(
                        BinOp::Add,
                        box Apply(
                            box Var("fib".to_string()),
                            box BinOp(BinOp::Sub, box Var("x".to_string()), box Const(Number(1)))
                        ),
                        box Apply(
                            box Var("fib".to_string()),
                            box BinOp(BinOp::Sub, box Var("x".to_string()), box Const(Number(2)))
                        )
                    )
                )
            ),
            box Apply(box Var("fib".to_string()), box Const(Number(3)))
        ))
    );
}

#[test]
fn let_type_func() {
    let e = expr("type a = Int; 42");
    assert_eq!(
        e,
        Ok(LetType(
            "a".to_string(),
            box Type::Int,
            box Const(Number(42))
        ))
    );
}
