use std::str;
use std::i64;
use nom::*;
use ast::Expression;
use tpe::Type;

named!(expr<Expression>, chain!(
        multispace? ~
        e: alt!(
            func_expr |
            let_expr |
            println |
            if_expr |
            equal) ~
        multispace?,
        || e
        ));

named!(println<Expression>,
       chain!(
           tag!("println") ~
           multispace ~
           e: expr,
           || { Expression::Println(box e) }
           )
       );

named!(func_expr<Expression>,
       alt!(
           chain!(
               tag!("func") ~
               multispace ~
               name: string ~
               multispace ~
               arg_name: string ~
               multispace? ~
               tag!(":") ~
               multispace? ~
               ty: function_type ~
               multispace? ~
               e: expr ~
               multispace? ~
               after: expr,
               || {
                   Expression::Let(name, box Expression::Closure(arg_name, box ty, box e), box after)
               }) |
           chain!(
               tag!("rec") ~
               multispace ~
               tag!("func") ~
               multispace ~
               name: string ~
               multispace ~
               arg_name: string ~
               multispace? ~
               tag!(":") ~
               multispace? ~
               ty: function_type ~
               multispace? ~
               tag!(":") ~
               multispace? ~
               ret_type: function_type ~
               multispace? ~
               tag!("=") ~
               e: expr ~
               after: expr,
               || {
                   Expression::RecFunc(name, arg_name, box ty, box ret_type, box e, box after)
               })
));


named!(let_expr<Expression>,
       chain!(
           tag!("let") ~
           space ~
           name: string ~
           space ~
           char!('=') ~
           space ~
           init: equal ~
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
        multispace? ~
        mut acc: greater ~
        multispace? ~
        many0!(
            alt!(
               tap!(a: preceded!(tag!("="), greater) => acc = Expression::Equal(box acc, box a.clone())) |
               tap!(a: preceded!(tag!("/="), greater) => acc = Expression::NotEqual(box acc, box a.clone()))
               )
        ) ~
        multispace?,
       || { return acc }
       )
   );

named!(greater<Expression>,
    chain!(
        multispace? ~
        mut acc: additive ~
        multispace? ~
        many0!(
            alt!(
               tap!(a: preceded!(tag!(">"), additive) => acc = Expression::GreaterThan(box acc, box a.clone())) |
               tap!(a: preceded!(tag!("<"), additive) => acc = Expression::LessThan(box acc, box a.clone()))
               )
        ) ~
        multispace?,
       || { return acc }
       )
   );



named!(additive<Expression>,
    chain!(
        multispace? ~
        mut acc: multive  ~
        multispace? ~
        many0!(
            alt!(
                tap!(add: preceded!(tag!("+"), multive) => acc = Expression::Add(box acc, box add.clone())) |
                tap!(sub: preceded!(tag!("-"), multive) => acc = Expression::Sub(box acc, box sub.clone()))
                )
            ) ~
        multispace?,
        || { return acc }
        )
    );

named!(multive<Expression>,
   chain!(
       multispace? ~
       mut acc: apply  ~
       multispace? ~
       many0!(
           alt!(
               tap!(mul: preceded!(tag!("*"), apply) => acc = Expression::Mult(box acc, box mul.clone())) |
               tap!(div: preceded!(tag!("/"), apply) => acc = Expression::Div(box acc, box div.clone())) |
               tap!(m: preceded!(tag!("%"), apply) => acc = Expression::Mod(box acc, box m.clone()))
               )
           ) ~
       multispace?,
       || { return acc }
       )
   );

named!(apply<Expression>,
    chain!(
        multispace? ~
        mut acc: factor ~
        multispace? ~
        many0!(
            tap!(f: preceded!(tag!("@"), factor) => acc = Expression::Apply(box acc, box f.clone()))
            ) ~
        multispace?,
        || { return acc }
        )
    );

named!(factor<Expression>,
   chain!(
       multispace? ~
       e: alt!(
           closure |
           number |
           boolean |
           variable |
           parens) ~
       multispace?,
       || e
       ));

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
           multispace? ~
           name: string ~
           multispace? ~
           tag!(":") ~
           multispace? ~
           t: function_type ~
           multispace? ~
           tag!("=>") ~
           e: expr,
           || Expression::Closure(name, box t, box e)
           ));

named!(function_type<Type>,
    alt!(
       chain!(
           from: variant_type ~
           multispace? ~
           tag!("->") ~
           multispace? ~
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
       chain!(
           multispace? ~
           e: map_res!(
               string,
               |s: String| Ok(Type::Primitive(s)) as Result<Type, ()>
               ) ~
           multispace?,
           || e
          ));


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



