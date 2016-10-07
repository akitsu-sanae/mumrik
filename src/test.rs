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


