use ast::{parsed, typed, BinOp};
use env::Env;
use typecheck::*;

#[test]
fn primitive_literal() {
    let env = Env::new();
    assert_eq!(
        check_lit(
            &parsed::Literal::Number(42, parsed::Position { start: 0, end: 0 }),
            &env
        ),
        Ok(typed::Literal::Number(42))
    );
    /*
    assert_eq!(type_::check(&Const(Number(42)), &context), Ok(Type::Int));
    assert_eq!(type_::check(&Const(Bool(true)), &context), Ok(Type::Bool));
    assert_eq!(type_::check(&Const(Char('c')), &context), Ok(Type::Char));
    assert_eq!(type_::check(&Const(Unit), &context), Ok(Type::Unit));

    let context = context.add(&"a".to_string(), &Type::Int);
    assert_eq!(type_::check(&Var("a".to_string()), &context), Ok(Type::Int)); */
}

/*
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
} */
