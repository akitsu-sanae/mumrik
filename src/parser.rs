use std::str;
use std::i64;
use nom::*;
use ast::Expression;
use tpe::Type;

named!(expr<Expression>, alt!(
        let_expr |
        if_expr |
        equal
        ));

named!(let_expr<Expression>,
       chain!(
           tag!("let") ~
           space ~
           name: string ~
           space ~
           char!('=') ~
           space ~
           init: additive ~
           char!(';') ~
           space ~
           e: expr,
           || Expression::Let(name, box init, box e)
           )
      );

named!(if_expr<Expression>,
       chain!(
           tag!("if") ~
           multispace ~
           cond: expr ~
           multispace? ~
           true_expr: delimited!(char!('{'), expr, char!('}')) ~
           multispace? ~
           tag!("else") ~
           multispace? ~
           false_expr: delimited!(char!('{'), expr, char!('}')),
           || Expression::If(box cond, box true_expr, box false_expr)
           ));

named!(equal<Expression>,
    chain!(
        mut acc: greater ~
        many0!(
            alt!(
               tap!(a: preceded!(tag!("="), greater) => acc = Expression::Equal(box acc, box a.clone())) |
               tap!(a: preceded!(tag!("/="), greater) => acc = Expression::NotEqual(box acc, box a.clone()))
               )
        ),
       || { return acc }
       )
   );

named!(greater<Expression>,
    chain!(
        mut acc: additive ~
        many0!(
            alt!(
               tap!(a: preceded!(tag!(">"), additive) => acc = Expression::GreaterThan(box acc, box a.clone())) |
               tap!(a: preceded!(tag!("<"), additive) => acc = Expression::LessThan(box acc, box a.clone()))
               )
        ),
       || { return acc }
       )
   );



named!(additive<Expression>,
  chain!(
    mut acc: multive  ~
             many0!(
               alt!(
                 tap!(add: preceded!(tag!("+"), multive) => acc = Expression::Add(box acc, box add.clone())) |
                 tap!(sub: preceded!(tag!("-"), multive) => acc = Expression::Sub(box acc, box sub.clone()))
               )
             ),
    || { return acc }
  )
);

named!(multive<Expression>,
  chain!(
    mut acc: apply  ~
             many0!(
               alt!(
                 tap!(mul: preceded!(tag!("*"), apply) => acc = Expression::Mult(box acc, box mul.clone())) |
                 tap!(div: preceded!(tag!("/"), apply) => acc = Expression::Div(box acc, box div.clone()))
               )
             ),
    || { return acc }
  )
);

named!(apply<Expression>,
    chain!(
        mut acc: factor ~
        many0!(
               tap!(f: preceded!(tag!("@"), factor) => acc = Expression::Apply(box acc, box f.clone()))
        ),
       || { return acc }
       )
   );

named!(factor<Expression>,
       alt!(
           closure |
           number |
           boolean |
           variable |
           parens
           )
      );

named!(number<Expression>,
  map_res!(
    map_res!(
      digit,
      str::from_utf8
    ),
    |s: &str| Ok(Expression::Number(i64::from_str_radix(s, 10).unwrap())) as Result<Expression, ()>
  )
);

named!(boolean<Expression>, alt!(
        map_res!(
            tag!("true"),
            |_: &[u8]| Ok(Expression::Bool(true)) as Result<Expression, ()>) |
        map_res!(
            tag!("false"),
            |_: &[u8]| Ok(Expression::Bool(false)) as Result<Expression, ()>)
        ));


named!(variable<Expression>,
       map_res!(
           string,
           |s: String| Ok(Expression::Var(s)) as Result<Expression, ()>
           )
    );

named!(closure<Expression>,
       chain!(
           tag!("\\") ~
           name: string ~
           tag!(":") ~
           space ~
           t: function_type ~
           space ~
           tag!("=>") ~
           space ~
           e: expr,
           || Expression::Closure(name, box t, box e)
           ));

named!(function_type<Type>,
    alt!(
       chain!(
           from: variant_type ~
           space ~
           tag!("->") ~
           space ~
           to: function_type,
           || Type::Function(box from, box to)
           ) |
       variant_type
       ));

named!(variant_type<Type>,
       chain!(
           mut acc: tuple_type ~
        many0!(
               tap!(t: preceded!(tag!("+"), tuple_type) => acc = Type::Variant(box acc, box t.clone()))
        ),
       || { return acc }
       )
   );

named!(tuple_type<Type>,
    chain!(
        mut acc: primitive_type ~
        many0!(
               tap!(p: preceded!(tag!("*"), primitive_type) => acc = Type::Tuple(box acc, box p.clone()))
        ),
       || { return acc }
       )
   );


named!(primitive_type<Type>,
       map_res!(
           string,
           |s: String| Ok(Type::Primitive(s)) as Result<Type, ()>
           )
    );


named!(string<String>,
       map_res!(
           map_res!(
               is_a!("abcdefghijklnmopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_?"),
               str::from_utf8
               ),
               |s: &str| Ok(s.to_string()) as Result<String, ()>
               )
      );

named!(parens<Expression>, delimited!(
    char!('('),
    expr,
    char!(')')
  )
);

pub fn expression(input: &[u8]) -> Expression {
    expr(input.as_bytes()).unwrap().1
}



