use context::Context;
use expr::{BinOp, Expr::*, Literal::*};
use type_::{self, Type};

#[test]
fn primitive_literal() {
    let context = Context::new();
    assert_eq!(type_::check(&Const(Number(42)), &context), Ok(Type::Int));
    assert_eq!(type_::check(&Const(Bool(true)), &context), Ok(Type::Bool));
    assert_eq!(type_::check(&Const(Char('c')), &context), Ok(Type::Char));
    assert_eq!(type_::check(&Const(Unit), &context), Ok(Type::Unit));

    let context = context.add(&"a".to_string(), &Type::Int);
    assert_eq!(type_::check(&Var("a".to_string()), &context), Ok(Type::Int));
}

#[test]
fn list() {
    let e = Const(List(vec![
        Const(Number(1)),
        Const(Number(2)),
        Const(Number(3)),
    ]));
    assert_eq!(
        type_::check(&e, &Context::new()),
        Ok(Type::List(box Type::Int))
    );
}

#[test]
fn apply() {
    let e = Apply(
        box Lambda("x".to_string(), box Type::Int, box Var("x".to_string())),
        box Const(Number(1)),
    );
    assert_eq!(type_::check(&e, &Context::new()), Ok(Type::Int));
}

#[test]
fn if_() {
    let e = If(
        box Const(Bool(true)),
        box Const(Number(1)),
        box Const(Number(2)),
    );
    assert_eq!(type_::check(&e, &Context::new()), Ok(Type::Int));
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
    assert_eq!(type_::check(&e, &Context::new()), Ok(Type::Int));
}

#[test]
fn let_type_func() {
    let e = LetType("a".to_string(), box Type::Int, box Const(Number(42)));
    assert_eq!(type_::check(&e, &Context::new()), Ok(Type::Int));
}
