use expr::Expr;
use type_::Type;
use context::Context;
use parser::expr;

#[test]
fn literal() {
    assert_eq!(expr(b"123").unwrap().1, Expr::Number(123));
    assert_eq!(expr(b"true").unwrap().1, Expr::Bool(true));
    assert_eq!(expr(b"false").unwrap().1, Expr::Bool(false));
    assert_eq!(expr(b"unit").unwrap().1, Expr::Unit);
    assert_eq!(expr(b"a").unwrap().1, Expr::Var("a".to_string()));
}

#[test]
fn apply() {
    let e = expr(b"(func x: Int => x)@1").unwrap().1;
    assert_eq!(e, Expr::Apply(
            box Expr::Lambda("x".to_string(), box Type::Primitive("Int".to_string()), box Expr::Var("x".to_string())),
            box Expr::Number(1)));
    assert_eq!(e.eval(&Context::new()), Expr::Number(1));
}

#[test]
fn sequence() {
    let e = expr(b"1; 2; 3").unwrap().1;
    assert_eq!(e, Expr::Sequence(
            box Expr::Number(1),
            box Expr::Sequence(
                box Expr::Number(2),
                box Expr::Number(3))));
    assert_eq!(e.eval(&Context::new()), Expr::Number(3));
}

#[test]
fn if_() {
    let e = expr(b"if true 1 2").unwrap().1;
    assert_eq!(e, Expr::If(
            box Expr::Bool(true),
            box Expr::Number(1),
            box Expr::Number(2)));
    assert_eq!(e.eval(&Context::new()), Expr::Number(1));
}

#[test]
fn arithmetic() {
    let e = expr(b"1+2*5+6").unwrap().1;
    assert_eq!(e, Expr::Add(
            box Expr::Add(
                box Expr::Number(1),
                box Expr::Mult(
                    box Expr::Number(2),
                    box Expr::Number(5))),
            box Expr::Number(6)));
    assert_eq!(e.eval(&Context::new()), Expr::Number(17))
}

#[test]
fn record() {
    let e = expr(b"[* id=42, value=123]").unwrap().1;
    assert_eq!(e, Expr::Record(vec![
        ("id".to_string(), box Expr::Number(42)),
        ("value".to_string(), box Expr::Number(123))]));
    assert_eq!(e.eval(&Context::new()), Expr::Record(vec![
        ("id".to_string(), box Expr::Number(42)),
        ("value".to_string(), box Expr::Number(123))]));
}

#[test]
fn dot() {
    let e = expr(b"[* id=42, value=123].id").unwrap().1;
    assert_eq!(e, Expr::Dot(
            box Expr::Record(vec![
                ("id".to_string(), box Expr::Number(42)),
                ("value".to_string(), box Expr::Number(123))]),
            "id".to_string()));
    assert_eq!(e.eval(&Context::new()), Expr::Number(42));
}

#[test]
fn variant() {
    let e = expr(b"[+ hoge=1] as [+ hoge:Int, fuga: Bool]").unwrap().1;
    assert_eq!(e, Expr::Variant(
            "hoge".to_string(),
            box Expr::Number(1),
            box Type::Variant(vec![
                ("hoge".to_string(), box Type::Primitive("Int".to_string())),
                ("fuga".to_string(), box Type::Primitive("Bool".to_string()))
                ])));
    assert_eq!(e.eval(&Context::new()), Expr::Variant(
            "hoge".to_string(),
            box Expr::Number(1),
            box Type::Variant(vec![
                              ("hoge".to_string(), box Type::Primitive("Int".to_string())),
                              ("fuga".to_string(), box Type::Primitive("Bool".to_string()))
                              ])));

}

#[test]
fn match_() {
    let e = expr(b"match [+ hoge=1] as [+ hoge:Int, fuga: Bool] { hoge x => x+1, fuga x => if x 100 200 }").unwrap().1;
    assert_eq!(e, Expr::Match(
            box Expr::Variant(
                "hoge".to_string(),
                box Expr::Number(1),
                box Type::Variant(vec![
                                  ("hoge".to_string(), box Type::Primitive("Int".to_string())),
                                  ("fuga".to_string(), box Type::Primitive("Bool".to_string()))
                                  ])),
            vec![
            ("hoge".to_string(), "x".to_string(),
            box Expr::Add(
                box Expr::Var("x".to_string()),
                box Expr::Number(1))),
            ("fuga".to_string(), "x".to_string(),
            box Expr::If(
                box Expr::Var("x".to_string()),
                box Expr::Number(100),
                box Expr::Number(200)))]));
    assert_eq!(e.eval(&Context::new()), Expr::Number(2));
}

