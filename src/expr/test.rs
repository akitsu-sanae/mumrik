use context::Context;
use expr::{parser::*, BinOp, Expr, Literal};
use type_::Type;

#[test]
fn literal() {
    assert_eq!(expr("123"), Ok(Expr::Const(Literal::Number(123))));
    assert_eq!(expr("true"), Ok(Expr::Const(Literal::Bool(true))));
    assert_eq!(expr("false"), Ok(Expr::Const(Literal::Bool(false))));
    assert_eq!(expr("unit"), Ok(Expr::Const(Literal::Unit)));
    assert_eq!(expr("a"), Ok(Expr::Var("a".to_string())));
}

#[test]
fn apply() {
    let e = expr("(func x: Int => x) 1");
    assert_eq!(
        e,
        Ok(Expr::Apply(
            box Expr::Lambda(
                "x".to_string(),
                box Type::Int,
                box Expr::Var("x".to_string())
            ),
            box Expr::Const(Literal::Number(1))
        ))
    );
    assert_eq!(
        e.unwrap().eval(&Context::new()),
        Ok(Expr::Const(Literal::Number(1)))
    );
}

#[test]
fn sequence() {
    let e = expr("1; 2; 3");
    assert_eq!(
        e,
        Ok(Expr::Let(
            "<dummy>".to_string(),
            box Expr::Const(Literal::Number(1)),
            box Expr::Let(
                "<dummy>".to_string(),
                box Expr::Const(Literal::Number(2)),
                box Expr::Const(Literal::Number(3))
            )
        ))
    );
    assert_eq!(
        e.unwrap().eval(&Context::new()),
        Ok(Expr::Const(Literal::Number(3)))
    );
}

#[test]
fn if_() {
    let e = expr("if true { 1 } else { 2 }");
    assert_eq!(
        e,
        Ok(Expr::If(
            box Expr::Const(Literal::Bool(true)),
            box Expr::Const(Literal::Number(1)),
            box Expr::Const(Literal::Number(2))
        ))
    );
    assert_eq!(
        e.unwrap().eval(&Context::new()),
        Ok(Expr::Const(Literal::Number(1)))
    );
}

#[test]
fn arithmetic() {
    let e = expr("1+2*5+6");
    assert_eq!(
        e,
        Ok(Expr::BinOp(
            BinOp::Add,
            box Expr::BinOp(
                BinOp::Add,
                box Expr::Const(Literal::Number(1)),
                box Expr::BinOp(
                    BinOp::Mult,
                    box Expr::Const(Literal::Number(2)),
                    box Expr::Const(Literal::Number(5))
                )
            ),
            box Expr::Const(Literal::Number(6))
        ))
    );
    assert_eq!(
        e.unwrap().eval(&Context::new()),
        Ok(Expr::Const(Literal::Number(17)))
    );
}

#[test]
fn compare() {
    let e = expr("1 < 2");
    assert_eq!(
        e,
        Ok(Expr::BinOp(
            BinOp::LessThan,
            box Expr::Const(Literal::Number(1)),
            box Expr::Const(Literal::Number(2))
        ))
    );
    assert_eq!(
        e.unwrap().eval(&Context::new()),
        Ok(Expr::Const(Literal::Bool(true)))
    );

    let e = expr("1 > 2");
    assert_eq!(
        e,
        Ok(Expr::BinOp(
            BinOp::GreaterThan,
            box Expr::Const(Literal::Number(1)),
            box Expr::Const(Literal::Number(2))
        ))
    );
    assert_eq!(
        e.unwrap().eval(&Context::new()),
        Ok(Expr::Const(Literal::Bool(false)))
    );
}

#[test]
fn record() {
    let e = expr("{ id=42, value=123 }");
    assert_eq!(
        e,
        Ok(Expr::Const(Literal::Record(vec![
            ("id".to_string(), box Expr::Const(Literal::Number(42))),
            ("value".to_string(), box Expr::Const(Literal::Number(123)))
        ])))
    );
    assert_eq!(
        e.unwrap().eval(&Context::new()),
        Ok(Expr::Const(Literal::Record(vec![
            ("id".to_string(), box Expr::Const(Literal::Number(42))),
            ("value".to_string(), box Expr::Const(Literal::Number(123)))
        ])))
    );
}

#[test]
fn tuple() {
    let e = expr("(1, 2, 3)");
    assert_eq!(
        e,
        Ok(Expr::Const(Literal::Record(vec![
            ("0".to_string(), box Expr::Const(Literal::Number(1))),
            ("1".to_string(), box Expr::Const(Literal::Number(2))),
            ("2".to_string(), box Expr::Const(Literal::Number(3)))
        ])))
    );
    assert_eq!(
        e.unwrap().eval(&Context::new()),
        Ok(Expr::Const(Literal::Record(vec![
            ("0".to_string(), box Expr::Const(Literal::Number(1))),
            ("1".to_string(), box Expr::Const(Literal::Number(2))),
            ("2".to_string(), box Expr::Const(Literal::Number(3)))
        ])))
    );
}

#[test]
fn dot() {
    let e = expr("{ id=42, value=123 }.id");
    assert_eq!(
        e,
        Ok(Expr::Dot(
            box Expr::Const(Literal::Record(vec![
                ("id".to_string(), box Expr::Const(Literal::Number(42))),
                ("value".to_string(), box Expr::Const(Literal::Number(123)))
            ])),
            "id".to_string()
        ))
    );
    assert_eq!(
        e.unwrap().eval(&Context::new()),
        Ok(Expr::Const(Literal::Number(42)))
    );
}

#[test]
fn variant() {
    let expr = expr("type Nyan = enum { Hoge: Int, Fuga: Bool}; Nyan::Hoge(42)").unwrap();
    let nyan_ty = Type::Variant(vec![
        ("Hoge".to_string(), box Type::Int),
        ("Fuga".to_string(), box Type::Bool),
    ]);
    assert_eq!(
        expr,
        Expr::LetType(
            "Nyan".to_string(),
            box nyan_ty,
            box Expr::Const(Literal::Variant(
                "Hoge".to_string(),
                box Expr::Const(Literal::Number(42)),
                box Type::Variable("Nyan".to_string()),
            )),
        )
    );
    assert_eq!(
        expr.eval(&Context::new()),
        Ok(Expr::Const(Literal::Variant(
            "Hoge".to_string(),
            box Expr::Const(Literal::Number(42)),
            box Type::Variable("Nyan".to_string()),
        )))
    );
}

#[test]
fn list() {
    let e = expr("[1, 2, 3]").unwrap();
    assert_eq!(
        e,
        Expr::Const(Literal::List(vec![
            Expr::Const(Literal::Number(1)),
            Expr::Const(Literal::Number(2)),
            Expr::Const(Literal::Number(3))
        ]))
    );
    assert_eq!(
        Type::from_expr(&e, &Context::new()),
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
        Type::from_expr(&e, &Context::new()),
        Ok(Type::List(box Type::Char))
    );
    assert_eq!(
        e.eval(&Context::new()),
        Ok(Expr::Const(Literal::List(vec![
            Expr::Const(Literal::Char('n')),
            Expr::Const(Literal::Char('y')),
            Expr::Const(Literal::Char('a')),
            Expr::Const(Literal::Char('n'))
        ])))
    );
}

#[test]
fn match_() {
    let expr = expr("type Nyan = enum { Hoge: Int Fuga: Bool}; match Nyan::Hoge(42) { Hoge x => x+1, Fuga x => if x { 100 } else { 200 } }").unwrap();
    let nyan_ty = Type::Variant(vec![
        ("Hoge".to_string(), box Type::Int),
        ("Fuga".to_string(), box Type::Bool),
    ]);
    let hoge_branch = (
        "Hoge".to_string(),
        "x".to_string(),
        box Expr::BinOp(
            BinOp::Add,
            box Expr::Var("x".to_string()),
            box Expr::Const(Literal::Number(1)),
        ),
    );
    let fuga_branch = (
        "Fuga".to_string(),
        "x".to_string(),
        box Expr::If(
            box Expr::Var("x".to_string()),
            box Expr::Const(Literal::Number(100)),
            box Expr::Const(Literal::Number(200)),
        ),
    );

    assert_eq!(
        expr,
        Expr::LetType(
            "Nyan".to_string(),
            box nyan_ty,
            box Expr::Match(
                box Expr::Const(Literal::Variant(
                    "Hoge".to_string(),
                    box Expr::Const(Literal::Number(42)),
                    box Type::Variable("Nyan".to_string())
                )),
                vec![hoge_branch, fuga_branch]
            )
        )
    );
    assert_eq!(
        expr.eval(&Context::new()),
        Ok(Expr::Const(Literal::Number(43)))
    );
}

#[test]
fn println() {
    let e = expr("println 1; println true; println unit").unwrap();
    assert_eq!(
        e,
        Expr::Let(
            "<dummy>".to_string(),
            box Expr::Println(box Expr::Const(Literal::Number(1))),
            box Expr::Let(
                "<dummy>".to_string(),
                box Expr::Println(box Expr::Const(Literal::Bool(true))),
                box Expr::Println(box Expr::Const(Literal::Unit))
            )
        )
    );
    assert_eq!(e.eval(&Context::new()), Ok(Expr::Const(Literal::Unit)));
}

#[test]
fn func() {
    let e = expr("func f a:Int { a+12 } f 13").unwrap();
    assert_eq!(
        e,
        Expr::Let(
            "f".to_string(),
            box Expr::Lambda(
                "a".to_string(),
                box Type::Int,
                box Expr::BinOp(
                    BinOp::Add,
                    box Expr::Var("a".to_string()),
                    box Expr::Const(Literal::Number(12))
                )
            ),
            box Expr::Apply(
                box Expr::Var("f".to_string()),
                box Expr::Const(Literal::Number(13))
            )
        )
    );
    assert_eq!(
        e.eval(&Context::new()),
        Ok(Expr::Const(Literal::Number(25)))
    );
}

#[test]
fn rec_func() {
    // let e = expr("rec let fib: Int -> Int = func x:Int => if x < 2 { 1 } else { (fib (x-1)) + (fib (x-2) }; fib 8").unwrap();
    let e =
        expr("rec func fib x:Int :Int { if x < 2 { 1 } else { (fib (x-1)) + (fib (x-2)) } } fib 3")
            .unwrap();
    assert_eq!(
        e,
        Expr::LetRec(
            "fib".to_string(),
            box Type::Function(box Type::Int, box Type::Int),
            box Expr::Lambda(
                "x".to_string(),
                box Type::Int,
                box Expr::If(
                    box Expr::BinOp(
                        BinOp::LessThan,
                        box Expr::Var("x".to_string()),
                        box Expr::Const(Literal::Number(2))
                    ),
                    box Expr::Const(Literal::Number(1)),
                    box Expr::BinOp(
                        BinOp::Add,
                        box Expr::Apply(
                            box Expr::Var("fib".to_string()),
                            box Expr::BinOp(
                                BinOp::Sub,
                                box Expr::Var("x".to_string()),
                                box Expr::Const(Literal::Number(1))
                            )
                        ),
                        box Expr::Apply(
                            box Expr::Var("fib".to_string()),
                            box Expr::BinOp(
                                BinOp::Sub,
                                box Expr::Var("x".to_string()),
                                box Expr::Const(Literal::Number(2))
                            )
                        )
                    )
                )
            ),
            box Expr::Apply(
                box Expr::Var("fib".to_string()),
                box Expr::Const(Literal::Number(3))
            )
        )
    );
    assert_eq!(e.eval(&Context::new()), Ok(Expr::Const(Literal::Number(3))));
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
