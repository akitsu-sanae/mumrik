use std::str;
use std::i64;
use nom::*;

use expr::Expr;
use type_::Type;


named!(pub expr<Expr>, alt!(
        chain!(
            multispace? ~
            tag!("let") ~
            multispace? ~
            name: identifier ~
            multispace? ~
            tag!("=") ~
            multispace? ~
            init: type_alias ~
            multispace? ~
            tag!(";") ~
            multispace? ~
            after: expr ~
            multispace?,
            || Expr::Let(name, box init, box after)) |
        sequence_expr));

named!(sequence_expr<Expr>, chain!(
        multispace? ~
        mut acc: type_alias ~
        multispace? ~
        many0!(
              tap!(e: preceded!(tag!(";"), expr) => acc = Expr::Sequence(box acc, box e.clone()))) ~
        multispace?,
        || acc));

named!(type_alias <Expr>, alt!(
        chain!(
            multispace? ~
            tag!("type") ~
            multispace? ~
            name: identifier ~
            multispace? ~
            tag!(":") ~
            multispace? ~
            ty: type_ ~
            multispace? ~
            e: expr ~
            multispace?,
            || Expr::TypeAlias(name, box ty, box e)) |
        if_expr));

named!(if_expr <Expr>, alt!(
        chain!(
            multispace? ~
            tag!("if") ~
            cond: expr ~
            tr: expr ~
            fl: expr,
            || Expr::If(box cond, box tr, box fl)) |
        match_expr));

named!(match_branch <(String, String, Box<Expr>)>,
    chain!(
        multispace? ~
        label: identifier ~
        multispace? ~
        name: identifier ~
        multispace? ~
        tag!("=>") ~
        multispace? ~
        e: expr ~
        multispace?,
        || (label, name, box e)));

named!(match_expr <Expr>, alt!(
        chain!(
            multispace? ~
            tag!("match") ~
            multispace? ~
            e: expr ~
            multispace? ~
            tag!("{") ~
            multispace? ~
            branches: chain!(
                mut acc: map!(match_branch, |x| vec![x]) ~
                multispace? ~
                many0!(
                      tap!(b: preceded!(tag!(","), match_branch) => acc.push(b.clone()))
                ) ~
                multispace?,
                || acc) ~
            multispace? ~
            tag!("}") ~
            multispace?,
            || Expr::Match(box e, branches)
            ) |
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
        mut acc: dot_expr ~
        multispace? ~
        many0!(
              tap!(e: preceded!(tag!("@"), dot_expr) => acc = Expr::Apply(box acc, box e.clone()))) ~
        multispace?,
        || acc));


named!(dot_expr <Expr>, chain!(
        multispace? ~
        mut acc: factor ~
        multispace? ~
        many0!(
              tap!(label: preceded!(tag!("."), identifier) => acc = Expr::Dot(box acc, label.clone()))
              ) ~
        multispace?,
        || acc
        ));

named!(factor <Expr>, alt!(
        lambda_expr |
        number |
        boolean |
        unit |
        println |
        variable |
        record |
        variant |
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

// [* first = 1, second = 114]
named!(record <Expr>, chain!(
        tag!("[*") ~
        multispace? ~
        mut branches: map!(branch, |e| vec![e]) ~
        multispace? ~
        many0!(
            tap!(e: preceded!(tag!(","), branch) => {
                branches.push(e.clone());
                branches.clone()
            })
        ) ~
        multispace? ~
        tag!("]") ~
        multispace?,
        || Expr::Record(branches.clone())));

// [+ first = cond ] as [+ first: Int, second: Bool]
named!(variant <Expr>, chain!(
        tag!("[+") ~
        multispace? ~
        br: branch ~
        multispace? ~
        tag!("]") ~
        multispace? ~
        tag!("as") ~
        multispace? ~
        ty: type_ ~
        multispace?,
        || Expr::Variant(br.0.clone(), br.1.clone(), box ty)
        ));

named!(branch <(String, Box<Expr>)>, chain!(
        multispace? ~
        label: identifier ~
        multispace? ~
        tag!("=") ~
        e: expr ~
        multispace?,
        || (label, box e)));

named!(paren <Expr>, chain!(
        tag!("(") ~
        e: expr ~
        tag!(")"),
        || e));

named!(type_ <Type>, alt!(
        variant_type |
        record_type |
        primitive_type));

named!(type_branch <(String, Box<Type>)>, chain!(
    multispace? ~
    label: identifier ~
    multispace? ~
    tag!(":") ~
    multispace? ~
    ty: type_ ~
    multispace?,
    || (label, box ty)));

named!(variant_type <Type>, chain!(
        multispace? ~
        tag!("[+") ~
        multispace? ~
        mut branches: map!(type_branch, |e| vec![e]) ~
        multispace? ~
        many0!(
              tap!(e: preceded!(tag!(","), type_branch) => {
                  branches.push(e.clone());
                  branches.clone()
              })) ~
        multispace? ~
        tag!("]") ~
        multispace?,
        || Type::Variant(branches)
        ));

named!(record_type <Type>, chain!(
        tag!("[*") ~
        multispace? ~
        mut branches: map!(type_branch, |e| vec![e]) ~
        multispace? ~
        many0!(
              tap!(e: preceded!(tag!(","), type_branch) => {
                  branches.push(e.clone());
                  branches.clone()
              })
        ) ~
        multispace? ~
        tag!("]") ~
        multispace?,
        || Type::Record(branches)));

named!(println <Expr>, chain!(
        multispace? ~
        tag!("println") ~
        multispace? ~
        e: type_alias ~
        multispace?,
        || Expr::Println(box e)));

named!(primitive_type <Type>,
       map!(identifier, |s: String| Type::Primitive(s)));

named!(identifier <String>,
       map!(
           map_res!(
               alphanumeric,
               str::from_utf8),
        |s: &str| s.to_string()));





