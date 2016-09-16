use std::str;
use std::i64;
use nom::*;
use ast::Expression;
use ast::Function;
use ast::Arg;
use tpe::Type;

named!(pub program<Vec<Function> >, many0!(function));

named!(function<Function>,
       chain!(
           multispace? ~
           tag!("func") ~
           multispace? ~
           name: identifier ~
           args: args ~
           tag!("{") ~
           body: expr ~
           tag!("}") ~
           multispace?,
           || Function {
               name: name,
               args: args,
               body: body,
           }));

named!(args <Vec<Arg> >,
       many0!(
           chain!(
               multispace? ~
               name: identifier ~
               multispace? ~
               tag!(":") ~
               multispace? ~
               ty: type_ ~
               multispace?,
               || Arg {
                   name: name.clone(),
                   tpe: box ty
               })));

named!(pub expr <Expression>, alt!(
        chain!(
        multispace? ~
        tag!("let") ~
        multispace? ~
        name: identifier ~
        multispace? ~
        tag!("=") ~
        multispace? ~
        init: if_expr ~
        tag!(";") ~
        body: expr,
        || Expression::Let(name, box init, box body)
        ) |
        if_expr));

named!(if_expr <Expression>, alt!(
       chain!(
           multispace? ~
           tag!("if") ~
           cond: or_expr ~
           tru: or_expr ~
           tag!("else") ~
           flse: or_expr,
           || Expression::If(box cond, box tru, box flse)
           ) |
       or_expr));

named!(or_expr <Expression>,
       chain!(
           mut acc: and_expr ~
           multispace? ~
           many0!(
               tap!(a: preceded!(tag!("|"), and_expr) => acc = Expression::Or(box acc, box a.clone()))
               ),
           || acc));
named!(and_expr <Expression>,
       chain!(
           mut acc: equal_expr ~
           multispace? ~
           many0!(
               tap!(a: preceded!(tag!("&"), equal_expr) => acc = Expression::And(box acc, box a.clone()))
               ),
           || acc));
named!(equal_expr <Expression>,
       chain!(
           mut acc: greater_expr ~
           multispace? ~
           many0!(
               alt!(
                   tap!(a: preceded!(tag!("="), greater_expr) => acc = Expression::Equal(box acc, box a.clone())) |
                   tap!(a: preceded!(tag!("/="), greater_expr) => acc = Expression::NotEqual(box acc, box a.clone()))
                   )
               ),
           || acc));
named!(greater_expr <Expression>,
       chain!(
           mut acc: add_expr ~
           multispace? ~
           many0!(
               alt!(
                   tap!(a: preceded!(tag!("<"), add_expr) => acc = Expression::Less(box acc, box a.clone())) |
                   tap!(a: preceded!(tag!(">"), add_expr) => acc = Expression::Greater(box acc, box a.clone())) |
                   tap!(a: preceded!(tag!("<="), add_expr) => acc = Expression::LessOrEqual(box acc, box a.clone())) |
                   tap!(a: preceded!(tag!(">="), add_expr) => acc = Expression::GreaterOrEqual(box acc, box a.clone()))
                   )
               ),
           || acc));

named!(add_expr <Expression>,
       chain!(
           mut acc: mult_expr ~
           multispace? ~
           many0!(
               alt!(
                   tap!(a: preceded!(tag!("+"), mult_expr) => acc = Expression::Add(box acc, box a.clone())) |
                   tap!(a: preceded!(tag!("-"), mult_expr) => acc = Expression::Sub(box acc, box a.clone()))
                   )
               ),
           || acc));

named!(mult_expr <Expression>,
       chain!(
           mut acc: not_expr ~
           multispace? ~
           many0!(
               alt!(
                   tap!(a: preceded!(tag!("*"), not_expr) => acc = Expression::Mult(box acc, box a.clone())) |
                   tap!(a: preceded!(tag!("/"), not_expr) => acc = Expression::Div(box acc, box a.clone())) |
                   tap!(a: preceded!(tag!("%"), not_expr) => acc = Expression::Mod(box acc, box a.clone()))
                   )
               ),
           || acc));

named!(not_expr <Expression>,
       alt!(
           chain!(
               multispace? ~
               tag!("!") ~
               e: not_expr,
               || Expression::Not(box e)) |
           apply_expr));

named!(apply_expr <Expression>,
       chain!(
           mut acc: dot_expr ~
           multispace? ~
           many0!(
               tap!(a: preceded!(tag!("@"), dot_expr) => acc = Expression::Apply(box acc, box a.clone()))
               ),
           || acc));

named!(dot_expr <Expression>,
       chain!(
           mut acc: primitive_expr ~
           multispace? ~
           many0!(
               tap!(a: preceded!(tag!("."), primitive_expr) => acc = Expression::Dot(box acc, box a.clone()))
               ),
           || acc));

named!(primitive_expr <Expression>,
       chain!(
           multispace? ~
           e: alt!(
               closure_expr |
               number_expr |
               boolean_expr |
               variable_expr |
               parens_expr) ~
           multispace?,
           || e));

named!(closure_expr <Expression>,
       chain!(
           tag!("\\") ~
           multispace? ~
           name: identifier ~
           multispace? ~
           tag!(":") ~
           multispace? ~
           t: type_ ~
           multispace? ~
           tag!("=>") ~
           e: expr,
           || Expression::Closure(name, box t, box e)
           ));
named!(number_expr<Expression>,
  map_res!(
    map_res!(
      digit,
      str::from_utf8
    ),
    |s: &str| Ok(Expression::Number(i64::from_str_radix(s, 10).unwrap())) as Result<Expression, ()>
  )
);
named!(boolean_expr <Expression>, alt!(
        map_res!(
            tag!("true"),
            |_: &[u8]| Ok(Expression::Bool(true)) as Result<Expression, ()>) |
        map_res!(
            tag!("false"),
            |_: &[u8]| Ok(Expression::Bool(false)) as Result<Expression, ()>)
        ));


named!(variable_expr <Expression>,
       map_res!(
           identifier,
           |s: String| Ok(Expression::Var(s)) as Result<Expression, ()>
           )
    );

named!(parens_expr <Expression>, delimited!(
    char!('('),
    expr,
    char!(')')
  )
);

named!(type_ <Type>, alt!(primitive_type));

named!(primitive_type<Type>,
       chain!(
           multispace? ~
           e: map_res!(
               identifier,
               |s: String| Ok(Type::Primitive(s)) as Result<Type, ()>
               ) ~
           multispace?,
           || e
          ));

named!(identifier<String>,
       map_res!(
           map_res!(
               is_a!("abcdefghijklnmopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_?"),
               str::from_utf8
               ),
               |s: &str| Ok(s.to_string()) as Result<String, ()>
               )
      );

