
use parser::expression;
use eval::eval;
use ast::Expression;

#[test]
fn parsing_test() {
    assert_eq!(expression(b"1"), Expression::Number(1));
    assert_eq!(expression(b"1+2"),
        Expression::Add(
            box Expression::Number(1),
            box Expression::Number(2)));
    assert_eq!(expression(b"1+2+3"),
        Expression::Add(
            box Expression::Add(
                box Expression::Number(1),
                box Expression::Number(2)),
            box Expression::Number(3)));
    assert_eq!(expression(b"2*3"),
        Expression::Mult(
            box Expression::Number(2),
            box Expression::Number(3)));
    assert_eq!(expression(b"4+2*3"),
        Expression::Add(
            box Expression::Number(4),
            box Expression::Mult(
                box Expression::Number(2),
                box Expression::Number(3))));
    assert_eq!(expression(b"5*4+1-3"),
        Expression::Sub(
            box Expression::Add(
                box Expression::Mult(
                    box Expression::Number(5),
                    box Expression::Number(4)),
                box Expression::Number(1)),
            box Expression::Number(3)));
    assert_eq!(expression(b"hoge"), Expression::Var("hoge".to_string()));
    assert_eq!(expression(b"hoge+1"),
        Expression::Add(
            box Expression::Var("hoge".to_string()),
            box Expression::Number(1)));

    assert_eq!(expression(b"let a = 1; 2"),
        Expression::Let("a".to_string(),
            box Expression::Number(1),
            box Expression::Number(2)));
    assert_eq!(expression(b"let a = 1+2; let b = 2+5; a*b"),
        Expression::Let("a".to_string(),
            box Expression::Add(
                box Expression::Number(1),
                box Expression::Number(2)),
            box Expression::Let("b".to_string(),
                box Expression::Add(
                    box Expression::Number(2),
                    box Expression::Number(5)),
                box Expression::Mult(
                    box Expression::Var("a".to_string()),
                    box Expression::Var("b".to_string())))));

    assert_eq!(expression(b"if true {1} else {2}"),
        Expression::If(
            box Expression::Bool(true),
            box Expression::Number(1),
            box Expression::Number(2)));

    assert_eq!(expression(b"if 1=1 {1} else {2}"),
        Expression::If(
            box Expression::Equal(
                box Expression::Number(1),
                box Expression::Number(1)),
            box Expression::Number(1),
            box Expression::Number(2)));

    assert_eq!(expression(b"if 1/=1 {1} else {2}"),
        Expression::If(
            box Expression::NotEqual(
                box Expression::Number(1),
                box Expression::Number(1)),
            box Expression::Number(1),
            box Expression::Number(2)));

    assert_eq!(expression(b"if 1<2 {1} else {2}"),
        Expression::If(
            box Expression::LessThan(
                box Expression::Number(1),
                box Expression::Number(2)),
            box Expression::Number(1),
            box Expression::Number(2)));

    assert_eq!(expression(b"if 1>2 {1} else {2}"),
        Expression::If(
            box Expression::GreaterThan(
                box Expression::Number(1),
                box Expression::Number(2)),
            box Expression::Number(1),
            box Expression::Number(2)));

}

#[test]
fn eval_test() {
    assert_eq!(eval(expression(b"1"), &vec![]), Expression::Number(1));
    assert_eq!(eval(expression(b"1+2"), &vec![]), Expression::Number(3));
    assert_eq!(eval(expression(b"1+2+3"), &vec![]), Expression::Number(6));
    assert_eq!(eval(expression(b"2*3"), &vec![]), Expression::Number(6));
    assert_eq!(eval(expression(b"4+2*3"), &vec![]), Expression::Number(10));
    assert_eq!(eval(expression(b"5*4+1-3"), &vec![]), Expression::Number(18));
    assert_eq!(eval(expression(b"hoge"), &vec![]), Expression::Error("no such variable: hoge".to_string()));

    assert_eq!(eval(expression(b"let a = 1; 2"), &vec![]), Expression::Number(2));
    assert_eq!(eval(expression(b"let a = 1+2; let b = 2+5; a*b"), &vec![]), Expression::Number(21));

    assert_eq!(eval(expression(b"if true {1} else {2}"), &vec![]), Expression::Number(1));
    assert_eq!(eval(expression(b"if 1=1 {1} else {2}"), &vec![]), Expression::Number(1));
    assert_eq!(eval(expression(b"if 1+1=2 {1} else {2}"), &vec![]), Expression::Number(1));

    assert_eq!(eval(expression(b"if 1/=1 {1} else {2}"), &vec![]), Expression::Number(2));
    assert_eq!(eval(expression(b"1>1"), &vec![]), Expression::Bool(false));
    assert_eq!(eval(expression(b"2>1"), &vec![]), Expression::Bool(true));
    assert_eq!(eval(expression(b"if 1<2 {1} else {2}"), &vec![]), Expression::Number(1));
    assert_eq!(eval(expression(b"if 1>2 {1} else {2}"), &vec![]),  Expression::Number(2));
}

