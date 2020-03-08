use context::Context;
use eval::expr;
use expr::{BinOp, Expr::*, Literal::*};
use type_::Type;

#[test]
fn apply() {
    let e = Apply(
        box Lambda("x".to_string(), box Type::Int, box Var("x".to_string())),
        box Const(Number(1)),
    );
    assert_eq!(expr(&e, &Context::new()), Ok(Const(Number(1))));
}

#[test]
fn sequence() {
    let e = Let(
        "<dummy>".to_string(),
        box Const(Number(1)),
        box Let(
            "<dummy>".to_string(),
            box Const(Number(2)),
            box Const(Number(3)),
        ),
    );
    assert_eq!(expr(&e, &Context::new()), Ok(Const(Number(3))));
}

#[test]
fn if_() {
    let e = If(
        box Const(Bool(true)),
        box Const(Number(1)),
        box Const(Number(2)),
    );
    assert_eq!(expr(&e, &Context::new()), Ok(Const(Number(1))));
}

#[test]
fn arithmetic() {
    let e = BinOp(
        BinOp::Add,
        box BinOp(
            BinOp::Add,
            box Const(Number(1)),
            box BinOp(BinOp::Mult, box Const(Number(2)), box Const(Number(5))),
        ),
        box Const(Number(6)),
    );

    assert_eq!(expr(&e, &Context::new()), Ok(Const(Number(17))));
}

#[test]
fn compare() {
    let e = BinOp(BinOp::LessThan, box Const(Number(1)), box Const(Number(2)));

    assert_eq!(expr(&e, &Context::new()), Ok(Const(Bool(true))));

    let e = BinOp(
        BinOp::GreaterThan,
        box Const(Number(1)),
        box Const(Number(2)),
    );
    assert_eq!(expr(&e, &Context::new()), Ok(Const(Bool(false))));
}

#[test]
fn record() {
    let e = Const(Record(vec![
        ("id".to_string(), box Const(Number(42))),
        ("value".to_string(), box Const(Number(123))),
    ]));
    assert_eq!(
        expr(&e, &Context::new()),
        Ok(Const(Record(vec![
            ("id".to_string(), box Const(Number(42))),
            ("value".to_string(), box Const(Number(123)))
        ])))
    );
}

#[test]
fn dot() {
    let e = Dot(
        box Const(Record(vec![
            ("id".to_string(), box Const(Number(42))),
            ("value".to_string(), box Const(Number(123))),
        ])),
        "id".to_string(),
    );
    assert_eq!(expr(&e, &Context::new()), Ok(Const(Number(42))));
}

#[test]
fn variant() {
    let nyan_ty = Type::Variant(vec![
        ("Hoge".to_string(), Type::Int),
        ("Fuga".to_string(), Type::Bool),
    ]);
    let e = LetType(
        "Nyan".to_string(),
        box nyan_ty,
        box Const(Variant(
            "Hoge".to_string(),
            box Const(Number(42)),
            box Type::Variable("Nyan".to_string()),
        )),
    );
    assert_eq!(
        expr(&e, &Context::new()),
        Ok(Const(Variant(
            "Hoge".to_string(),
            box Const(Number(42)),
            box Type::Variable("Nyan".to_string()),
        )))
    );
}

#[test]
fn match_() {
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

    let e = LetType(
        "Nyan".to_string(),
        box nyan_ty,
        box Match(
            box Const(Variant(
                "Hoge".to_string(),
                box Const(Number(42)),
                box Type::Variable("Nyan".to_string()),
            )),
            vec![hoge_branch, fuga_branch],
        ),
    );

    assert_eq!(expr(&e, &Context::new()), Ok(Const(Number(43))));
}

#[test]
fn println() {
    let e = Let(
        "<dummy>".to_string(),
        box Println(box Const(Number(1))),
        box Let(
            "<dummy>".to_string(),
            box Println(box Const(Bool(true))),
            box Println(box Const(Unit)),
        ),
    );

    assert_eq!(expr(&e, &Context::new()), Ok(Const(Unit)));
}

#[test]
fn func() {
    let e = Let(
        "f".to_string(),
        box Lambda(
            "a".to_string(),
            box Type::Int,
            box BinOp(BinOp::Add, box Var("a".to_string()), box Const(Number(12))),
        ),
        box Apply(box Var("f".to_string()), box Const(Number(13))),
    );

    assert_eq!(expr(&e, &Context::new()), Ok(Const(Number(25))));
}

#[test]
fn rec_func() {
    let e = LetRec(
        "fib".to_string(),
        box Type::Function(box Type::Int, box Type::Int),
        box Lambda(
            "x".to_string(),
            box Type::Int,
            box If(
                box BinOp(
                    BinOp::LessThan,
                    box Var("x".to_string()),
                    box Const(Number(2)),
                ),
                box Const(Number(1)),
                box BinOp(
                    BinOp::Add,
                    box Apply(
                        box Var("fib".to_string()),
                        box BinOp(BinOp::Sub, box Var("x".to_string()), box Const(Number(1))),
                    ),
                    box Apply(
                        box Var("fib".to_string()),
                        box BinOp(BinOp::Sub, box Var("x".to_string()), box Const(Number(2))),
                    ),
                ),
            ),
        ),
        box Apply(box Var("fib".to_string()), box Const(Number(3))),
    );

    assert_eq!(expr(&e, &Context::new()), Ok(Const(Number(3))));
}

#[test]
fn let_type_func() {
    let e = LetType("a".to_string(), box Type::Int, box Const(Number(42)));
    assert_eq!(expr(&e, &Context::new()), Ok(Const(Number(42))));
}
