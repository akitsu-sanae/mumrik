use std::str;
use std::i64;
use nom::*;

use expr::Expr;
use type_::Type;

named!(pub expr<Expr>, chain!(
        multispace? ~
        mut acc: if_expr ~
        multispace? ~
        many0!(
              tap!(e: preceded!(tag!(";"), expr) => acc = Expr::Sequence(box acc, box e.clone()))) ~
        multispace?,
        || acc));

named!(if_expr <Expr>, alt!(
        chain!(
            multispace? ~
            tag!("if") ~
            cond: expr ~
            tr: expr ~
            fl: expr,
            || Expr::If(box cond, box tr, box fl)) |
        equal_expr));

named!(equal_expr <Expr>, chain!(
        multispace? ~
        mut acc: add_expr ~
        multispace? ~
        many0!(
              alt!(
                  tap!(e: preceded!(tag!("="), add_expr) => acc = Expr::Equal(box acc, box e.clone())) |
                  tap!(e: preceded!(tag!("/="), add_expr) => acc = Expr::NotEqual(box acc, box e.clone()))
                  )
              ) ~
        multispace?,
        || acc));

named!(add_expr <Expr>, chain!(
        multispace? ~
        mut acc: mult_expr ~
        multispace? ~
        many0!(
            alt!(
            tap!(e: preceded!(tag!("+"), mult_expr) => acc = Expr::Add(box acc, box e.clone())) |
            tap!(e: preceded!(tag!("-"), mult_expr) => acc = Expr::Sub(box acc, box e.clone()))
                )
              ) ~
        multispace?,
        || acc));

named!(mult_expr <Expr>, chain!(
        multispace? ~
        mut acc: apply_expr ~
        multispace? ~
        many0!(
            alt!(
                tap!(e: preceded!(tag!("*"), apply_expr) => acc = Expr::Mult(box acc, box e.clone())) |

                tap!(e: preceded!(tag!("/"), apply_expr) => acc = Expr::Div(box acc, box e.clone())))
            ) ~
        multispace?,
        || acc));

named!(apply_expr <Expr>, chain!(
        multispace? ~
        mut acc: factor ~
        multispace? ~
        many0!(
              tap!(e: preceded!(tag!("@"), factor) => acc = Expr::Apply(box acc, box e.clone()))) ~
        multispace?,
        || acc));

named!(factor <Expr>, alt!(
        lambda_expr |
        number |
        boolean |
        unit |
        variable |
        paren
        ));

// func x: Int => x
named!(lambda_expr <Expr>, chain!(
        tag!("func") ~
        multispace? ~
        name: identifier ~
        multispace? ~
        tag!(":") ~
        multispace? ~
        ty: type_ ~
        multispace? ~
        tag!("=>") ~
        body: expr ~
        multispace?,
        || Expr::Lambda(name, box ty, box body)
        ));

named!(boolean <Expr>, alt!(
        map!(tag!("true"), |_| Expr::Bool(true)) |
        map!(tag!("false"), |_| Expr::Bool(false))));

named!(unit <Expr>, alt!(
        map!(tag!("unit"), |_| Expr::Unit)));

named!(number <Expr>,
       map!(
           map_res!(
               digit,
               str::from_utf8),
        |s: &str| Expr::Number(i64::from_str_radix(s, 10).unwrap())
        ));

named!(variable <Expr>,
       map!(identifier, |s: String| Expr::Var(s.clone())));

named!(paren <Expr>, chain!(
        tag!("(") ~
        e: expr ~
        tag!(")"),
        || e));

named!(type_ <Type>,
       map!(identifier, |s: String| Type::Primitive(s)));

named!(identifier <String>,
       map!(
           map_res!(
               alphanumeric,
               str::from_utf8),
        |s: &str| s.to_string()));





