use expr::{Expr::*, Literal::*};
use type_::Type;

#[test]
fn subst() {
    // [42/a](func b:Int => a)
    let e = Lambda("b".to_string(), box Type::Int, box Var("a".to_string()));
    assert_eq!(
        e.subst_expr("a", &Const(Number(42))),
        Lambda("b".to_string(), box Type::Int, box Const(Number(42)))
    );

    // [42/b](func b:Int => a)
    let e = Lambda("b".to_string(), box Type::Int, box Var("a".to_string()));
    assert_eq!(e.clone().subst_expr("b", &Const(Number(42))), e);
}
