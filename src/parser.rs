use std::str;
use std::i64;
use nom::*;
use ast::Expression;

named!(expr<Expression>,
  chain!(
    mut acc: term  ~
             many0!(
               alt!(
                 tap!(add: preceded!(tag!("+"), term) => acc = Expression::Add(box acc, box add.clone())) |
                 tap!(sub: preceded!(tag!("-"), term) => acc = Expression::Sub(box acc, box sub.clone()))
               )
             ),
    || { return acc }
  )
);

named!(term<Expression>,
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



named!(parens<Expression>, delimited!(
    char!('('),
    expr,
    char!(')')
  )
);

pub fn expression(input: &[u8]) -> Expression {
    expr(input.as_bytes()).unwrap().1
}



