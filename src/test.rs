use context::Context;
use expr::Expr;
use parser::grammar::*;
use type_::Type;

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
    let e = expr("(func x: Int => x) 1").unwrap();
    assert_eq!(
        e,
        Expr::Apply(
            box Expr::Lambda(
                "x".to_string(),
                box Type::Int,
                box Expr::Var("x".to_string())
            ),
            box Expr::Number(1)
        )
    );
    assert_eq!(e.eval(&Context::new()), Ok(Expr::Number(1)));
}

#[test]
fn sequence() {
    let e = expr("1; 2; 3").unwrap();
    assert_eq!(
        e,
        Expr::Sequence(
            box Expr::Number(1),
            box Expr::Sequence(box Expr::Number(2), box Expr::Number(3))
        )
    );
    assert_eq!(e.eval(&Context::new()), Ok(Expr::Number(3)));
}

#[test]
fn if_() {
    let e = expr("if true { 1 } else { 2 }").unwrap();
    assert_eq!(
        e,
        Expr::If(
            box Expr::Bool(true),
            box Expr::Number(1),
            box Expr::Number(2)
        )
    );
    assert_eq!(e.eval(&Context::new()), Ok(Expr::Number(1)));
}

#[test]
fn arithmetic() {
    let e = expr("1+2*5+6").unwrap();
    assert_eq!(
        e,
        Expr::Add(
            box Expr::Add(
                box Expr::Number(1),
                box Expr::Mult(box Expr::Number(2), box Expr::Number(5))
            ),
            box Expr::Number(6)
        )
    );
    assert_eq!(e.eval(&Context::new()), Ok(Expr::Number(17)));
}

#[test]
fn compare() {
    let e = expr("1 < 2").unwrap();
    assert_eq!(e, Expr::LessThan(box Expr::Number(1), box Expr::Number(2)));
    assert_eq!(e.eval(&Context::new()), Ok(Expr::Bool(true)));

    let e = expr("1 > 2").unwrap();
    assert_eq!(
        e,
        Expr::GreaterThan(box Expr::Number(1), box Expr::Number(2))
    );
    assert_eq!(e.eval(&Context::new()), Ok(Expr::Bool(false)));
}

#[test]
fn record() {
    let e = expr("{id:Int, value:Int}{ id=42, value=123 }").unwrap();
    assert_eq!(
        e,
        Expr::Record(vec![
            ("id".to_string(), box Expr::Number(42)),
            ("value".to_string(), box Expr::Number(123))
        ])
    );
    assert_eq!(
        e.eval(&Context::new()),
        Ok(Expr::Record(vec![
            ("id".to_string(), box Expr::Number(42)),
            ("value".to_string(), box Expr::Number(123))
        ]))
    );
}

#[test]
fn tuple() {
    let e = expr("(1, 2, 3)").unwrap();
    assert_eq!(
        e,
        Expr::Record(vec![
            ("0".to_string(), box Expr::Number(1)),
            ("1".to_string(), box Expr::Number(2)),
            ("2".to_string(), box Expr::Number(3))
        ])
    );
    assert_eq!(
        e.eval(&Context::new()),
        Ok(Expr::Record(vec![
            ("0".to_string(), box Expr::Number(1)),
            ("1".to_string(), box Expr::Number(2)),
            ("2".to_string(), box Expr::Number(3))
        ]))
    );
}

#[test]
fn dot() {
    let e = expr("{id:Int, value:Int}{ id=42, value=123 }.id").unwrap();
    assert_eq!(
        e,
        Expr::Dot(
            box Expr::Record(vec![
                ("id".to_string(), box Expr::Number(42)),
                ("value".to_string(), box Expr::Number(123))
            ]),
            "id".to_string()
        )
    );
    assert_eq!(e.eval(&Context::new()), Ok(Expr::Number(42)));
}

#[test]
fn variant() {
    let mut program = program("type Nyan = enum { Hoge : Int Fuga: Bool} Nyan::Hoge(42)").unwrap();
    program.expr.subst_typealias(&program.typ_aliases);
    let nyan_ty = Type::Variant(vec![
        ("Hoge".to_string(), box Type::Int),
        ("Fuga".to_string(), box Type::Bool),
    ]);
    assert_eq!(
        program.expr,
        Expr::Variant(
            "Hoge".to_string(),
            box Expr::Number(42),
            box nyan_ty.clone()
        )
    );
    assert_eq!(
        program.expr.eval(&Context::new()),
        Ok(Expr::Variant(
            "Hoge".to_string(),
            box Expr::Number(42),
            box nyan_ty
        ))
    );
}

#[test]
fn list() {
    let e = expr("[1, 2, 3]").unwrap();
    assert_eq!(
        e,
        Expr::List(vec![Expr::Number(1), Expr::Number(2), Expr::Number(3)])
    );
    assert_eq!(
        Type::from_expr(&e, &Context::new()),
        Ok(Type::List(box Type::Int))
    );
    assert_eq!(
        e.eval(&Context::new()),
        Ok(Expr::List(vec![
            Expr::Number(1),
            Expr::Number(2),
            Expr::Number(3)
        ]))
    );
}

#[test]
fn string() {
    let e = expr("\"nyan\"").unwrap();
    assert_eq!(
        e,
        Expr::List(vec![
            Expr::Char('n'),
            Expr::Char('y'),
            Expr::Char('a'),
            Expr::Char('n')
        ])
    );
    assert_eq!(
        Type::from_expr(&e, &Context::new()),
        Ok(Type::List(box Type::Char))
    );
    assert_eq!(
        e.eval(&Context::new()),
        Ok(Expr::List(vec![
            Expr::Char('n'),
            Expr::Char('y'),
            Expr::Char('a'),
            Expr::Char('n')
        ]))
    );
}

#[test]
fn match_() {
    let mut program = program("type Nyan = enum { Hoge: Int Fuga: Bool} match Nyan::Hoge(42) { Hoge x => x+1, Fuga x => if x { 100 } else { 200 } }").unwrap();
    program.expr.subst_typealias(&program.typ_aliases);
    let nyan_ty = Type::Variant(vec![
        ("Hoge".to_string(), box Type::Int),
        ("Fuga".to_string(), box Type::Bool),
    ]);
    assert_eq!(
        program.expr,
        Expr::Match(
            box Expr::Variant("Hoge".to_string(), box Expr::Number(42), box nyan_ty),
            vec![
                (
                    "Hoge".to_string(),
                    "x".to_string(),
                    box Expr::Add(box Expr::Var("x".to_string()), box Expr::Number(1))
                ),
                (
                    "Fuga".to_string(),
                    "x".to_string(),
                    box Expr::If(
                        box Expr::Var("x".to_string()),
                        box Expr::Number(100),
                        box Expr::Number(200)
                    )
                )
            ]
        )
    );
    assert_eq!(program.expr.eval(&Context::new()), Ok(Expr::Number(43)));
}

#[test]
fn println() {
    let e = expr("println 1; println true; println unit").unwrap();
    assert_eq!(
        e,
        Expr::Sequence(
            box Expr::Println(box Expr::Number(1)),
            box Expr::Sequence(
                box Expr::Println(box Expr::Bool(true)),
                box Expr::Println(box Expr::Unit)
            )
        )
    );
    assert_eq!(e.eval(&Context::new()), Ok(Expr::Unit));
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
                box Expr::Add(box Expr::Var("a".to_string()), box Expr::Number(12))
            ),
            box Expr::Apply(box Expr::Var("f".to_string()), box Expr::Number(13))
        )
    );
    assert_eq!(e.eval(&Context::new()), Ok(Expr::Number(25)));
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
                    box Expr::LessThan(box Expr::Var("x".to_string()), box Expr::Number(2)),
                    box Expr::Number(1),
                    box Expr::Add(
                        box Expr::Apply(
                            box Expr::Var("fib".to_string()),
                            box Expr::Sub(box Expr::Var("x".to_string()), box Expr::Number(1))
                        ),
                        box Expr::Apply(
                            box Expr::Var("fib".to_string()),
                            box Expr::Sub(box Expr::Var("x".to_string()), box Expr::Number(2))
                        )
                    )
                )
            ),
            box Expr::Apply(box Expr::Var("fib".to_string()), box Expr::Number(3))
        )
    );
    assert_eq!(e.eval(&Context::new()), Ok(Expr::Number(3)));
}
