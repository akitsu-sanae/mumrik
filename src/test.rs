mod syntax;
mod parse;

#[test]
fn expr_test() {
    assert_eq!(syntax::parse::expr("12"), Ok(
            Expression::Number(12)
            ));
    assert_eq!(syntax::parse::expr("120"), Ok(
            Expression::Number(120)
            ));
    assert_eq!(syntax::parse::expr("012"), Ok(
            Expression::Number(12)
            ));
    assert_eq!(syntax::parse::expr("akitsu"), Ok(
            Expression::Identifier("akitsu".to_string())
            ));
    assert_eq!(eval (syntax::parse::expr("1+2").unwrap()), 3);
    assert_eq!(eval (syntax::parse::expr("1+2+3").unwrap()), 6);
    assert_eq!(eval (syntax::parse::expr("1+2*4").unwrap()), 9);
    assert_eq!(eval (syntax::parse::expr("1*4+2").unwrap()), 6);
    assert_eq!(eval (syntax::parse::expr("2*(4+2)").unwrap()), 12);
    assert_eq!(eval (syntax::parse::expr("(1*4)+2").unwrap()), 6);
}

#[test]
fn type_test() {
    assert_eq!(syntax::parse::type_("int"), Ok(Type::Atomic("int".to_string())));
    assert_eq!(syntax::parse::type_("int -> int"), Ok(Type::Function(
                Box::new(Type::Atomic("int".to_string())),
                Box::new(Type::Atomic("int".to_string()))
                )));
    assert_eq!(syntax::parse::type_("[int]"), Ok(Type::List(
                Box::new(Type::Atomic("int".to_string())))));
}

#[test]
fn function_test() {
    assert_eq!(syntax::parse::function("func akitsu : int = 1"), Ok(Function {
        name : "akitsu".to_string(),
        expr : Expression::Number(1),
        type_ : Type::Atomic("int".to_string())
    }
            ));
}

#[test]
fn program_test() {
    assert_eq!(syntax::parse::class(r#"
    class Hoge {
        func f : int -> int = 1+2+3
        let a : int = 12
        let b : int = 12 + 23
    }"#),
    Ok(
      Class{
            name : "Hoge".to_string(),
            functions : vec! [
                Function {
                    name: "f".to_string(),
                    type_ : Type::Function(
                        Box::new(Type::Atomic("int".to_string())),
                        Box::new(Type::Atomic("int".to_string()))
                    ),
                    expr : Expression::Add(
                        Box::new(Expression::Number(1)),
                        Box::new(Expression::Add(
                            Box::new(Expression::Number(2)),
                            Box::new(Expression::Number(3))
                            ))
                        )
                }
            ],
            variables : vec! [
                Variable {
                    name : "a".to_string(),
                    type_ : Type::Atomic("int".to_string()),
                    expr : Expression::Number(12)
                },
                Variable {
                    name : "b".to_string(),
                    type_ : Type::Atomic("int".to_string()),
                    expr : Expression::Add(
                        Box::new(Expression::Number(12)),
                        Box::new(Expression::Number(23))
                        )
                }
            ]
        }
    ));
}


