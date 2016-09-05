use std::str;
use std::i64;
use nom::*;
use ast::Expression;


/*
 * expr := let x = additive; expr
 * */
named!(expr<Expression>, alt!(
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
            ) |
        additive
        ));

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
    mut acc: factor  ~
             many0!(
               alt!(
                 tap!(mul: preceded!(tag!("*"), factor) => acc = Expression::Mult(box acc, box mul.clone())) |
                 tap!(div: preceded!(tag!("/"), factor) => acc = Expression::Div(box acc, box div.clone()))
               )
             ),
    || { return acc }
  )
);

named!(factor<Expression>,
  alt!(
      number |
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

named!(variable<Expression>,
       map_res!(
           string,
           |s: String| Ok(Expression::Var(s)) as Result<Expression, ()>
           )
    );

named!(string<String>,
       map_res!(
           map_res!(
               is_a!("abcdefghijklmopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_?"),
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



