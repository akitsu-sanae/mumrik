use std::str;
use std::i64;
use nom::*;

use expr::Expr;

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
        apply_expr));


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

// func x => x
named!(lambda_expr <Expr>, chain!(
        tag!("func") ~
        multispace? ~
        name: identifier ~
        multispace? ~
        tag!("=>") ~
        body: expr ~
        multispace?,
        || Expr::Lambda(name, box body)
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


named!(identifier <String>,
       map!(
           map_res!(
               alphanumeric,
               str::from_utf8),
        |s: &str| s.to_string()));





