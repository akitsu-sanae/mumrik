use expr::Expr;
use type_::Type;
use context::Context;
use parse::*;

#[test]
fn literal() {
    assert_eq!(expr("123").unwrap(), Expr::Number(123));
    assert_eq!(expr("true").unwrap(), Expr::Bool(true));
    assert_eq!(expr("false").unwrap(), Expr::Bool(false));
    assert_eq!(expr("unit").unwrap(), Expr::Unit);
    assert_eq!(expr("a").unwrap(), Expr::Var("a".to_string()));
}

#[test]
fn apply() {
    let e = expr("(func x: Int => x)@1").unwrap();
    assert_eq!(e, Expr::Apply(
            box Expr::Lambda("x".to_string(), box Type::Primitive("Int".to_string()), box Expr::Var("x".to_string())),
            box Expr::Number(1)));
    assert_eq!(e.eval(&Context::new()), Expr::Number(1));
}

#[test]
fn sequence() {
    let e = expr("1; 2; 3").unwrap();
    assert_eq!(e, Expr::Sequence(
            box Expr::Number(1),
            box Expr::Sequence(
                box Expr::Number(2),
                box Expr::Number(3))));
    assert_eq!(e.eval(&Context::new()), Expr::Number(3));
}

#[test]
fn if_() {
    let e = expr("if true 1 2").unwrap();
    assert_eq!(e, Expr::If(
            box Expr::Bool(true),
            box Expr::Number(1),
            box Expr::Number(2)));
    assert_eq!(e.eval(&Context::new()), Expr::Number(1));
}

#[test]
fn arithmetic() {
    let e = expr("1+2*5+6").unwrap();
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
    let e = expr("[* id=42, value=123 *]").unwrap();
    assert_eq!(e, Expr::Record(vec![
        ("id".to_string(), box Expr::Number(42)),
        ("value".to_string(), box Expr::Number(123))]));
    assert_eq!(e.eval(&Context::new()), Expr::Record(vec![
        ("id".to_string(), box Expr::Number(42)),
        ("value".to_string(), box Expr::Number(123))]));
}

#[test]
fn dot() {
    let e = expr("[* id=42, value=123 *].id").unwrap();
    assert_eq!(e, Expr::Dot(
            box Expr::Record(vec![
                ("id".to_string(), box Expr::Number(42)),
                ("value".to_string(), box Expr::Number(123))]),
            "id".to_string()));
    assert_eq!(e.eval(&Context::new()), Expr::Number(42));
}

#[test]
fn variant() {
    let e = expr("[+ hoge=1 +] as [+ hoge:Int, fuga: Bool +]").unwrap();
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
    let e = expr("match [+ hoge=1 +] as [+ hoge:Int, fuga: Bool +] { hoge x => x+1, fuga x => if x 100 200 }").unwrap();
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

#[test]
fn println() {
    let e = expr("println 1; println true; println unit").unwrap();
    assert_eq!(e, Expr::Sequence(
            box Expr::Println(box Expr::Number(1)),
            box Expr::Sequence(
                box Expr::Println(box Expr::Bool(true)),
                box Expr::Println(box Expr::Unit))));
    assert_eq!(e.eval(&Context::new()), Expr::Unit);

}

