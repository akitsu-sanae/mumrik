use crate::{expr::Expr, program::Program, type_::Type};
use peg;
use std::collections::HashMap;
use std::iter::FromIterator;

peg::parser!(grammar rules() for str {

pub rule program() -> Program
    = __ type_aliases:type_aliases() e:expr() {
        Program {
            expr: e,
            type_aliases: type_aliases
        }
    }

rule type_aliases() -> HashMap<String, Type>
    = aliases:(TYPE() name:ident() EQUAL() ty:type_() {(name, ty)})* {
        HashMap::from_iter(aliases.into_iter())
    }

pub rule type_() -> Type
    = __ ty:variant_type() { ty }
    / __ ty:primitive_type() { ty }

rule variant_type() -> Type
    = ENUM() LEFT_BRACE() branches:(tag:ident() COLON() ty:type_() COMMA()? { (tag, box ty) })+ RIGHT_BRACE() {
        Type::Variant(branches)
    }

rule primitive_type() -> Type
    = INT() { Type::Int }
    / BOOL() { Type::Bool }
    / name:ident() { Type::Variable(name) }

pub rule expr() -> Expr
    = __ e:func_expr() { e }

rule func_expr() -> Expr
    = FUNC() name:ident() arg:ident() COLON() arg_ty:type_() LEFT_BRACE() body:sequence_expr() RIGHT_BRACE() after:expr() {
      // `func f arg: T = expr`
      // is syntax sugar of `let f = func arg: T -> expr`
      Expr::Let(name, box Expr::Lambda(arg, box arg_ty, box body), box after)
    }
    / REC() FUNC() name:ident() arg:ident() COLON() arg_ty:type_() COLON() ret_ty:type_() LEFT_BRACE() body:sequence_expr() RIGHT_BRACE() after:expr() {
      Expr::LetRec(name, box Type::Function(box arg_ty.clone(), box ret_ty), box Expr::Lambda(arg, box arg_ty, box body), box after)
    }
    / sequence_expr()

rule sequence_expr() -> Expr
    = e1:inner_expr() SEMICOLON() e2:sequence_expr() {
        Expr::Sequence(box e1, box e2)
    }
    / e:inner_expr() { e }

rule inner_expr() -> Expr
    = if_expr()

rule if_expr() -> Expr
    = IF() cond:expr() LEFT_BRACE() tr:expr() RIGHT_BRACE() ELSE() LEFT_BRACE() fl:expr() RIGHT_BRACE() {
        Expr::If(box cond, box tr, box fl)
    }
    / match_expr()

rule match_expr() -> Expr
    = MATCH() e:expr() LEFT_BRACE() branches:(label:ident() name:ident() FAT_ARROW() e:expr() COMMA()? { (label, name, box e) })* RIGHT_BRACE() {
        Expr::Match(box e, branches)
    }
    / binop_expr()

rule binop_expr() -> Expr = precedence! {
    x:(@) DOUBLE_EQUAL() y:@ { Expr::Equal(box x, box y) }
    x:(@) NOT_EQUAL() y:@ { Expr::NotEqual(box x, box y) }
    x:(@) LEFT_ANGLE_BRACKET() y:@ { Expr::LessThan(box x, box y) }
    x:(@) RIGHT_ANGLE_BRACKET() y:@ { Expr::GreaterThan(box x, box y) }
    --
    x:(@) PLUS() y:@ { Expr::Add(box x, box y) }
    x:(@) MINUS() y:@ { Expr::Sub(box x, box y) }
    --
    x:(@) STAR() y:@ { Expr::Mult(box x, box y) }
    x:(@) SLASH() y:@ { Expr::Div(box x, box y) }
    --
    e:apply_expr() { e }
}

rule apply_expr() -> Expr
    = e1:dot_expr() e2:apply_expr() {
        Expr::Apply(box e1, box e2)
    }
    / dot_expr()

rule dot_expr() -> Expr
    = e:factor_expr() labels:(DOT() id:ident() {id})* {
        labels.into_iter().fold(e, |acc, label| Expr::Dot(box acc, label))
    }

rule factor_expr() -> Expr
    = lambda_expr()
    / n:number() { Expr::Number(n) }
    / boolean_expr()
    / unit_expr()
    / char_expr()
    / string_expr()
    / record_expr()
    / tuple_expr()
    / variant_expr()
    / list_expr()
    / println_expr()
    / name:ident() { Expr::Var(name) }
    / LEFT_PAREN() e:expr() RIGHT_PAREN() { e }

rule lambda_expr() -> Expr
    = FUNC() name:ident() COLON() ty:type_() FAT_ARROW() body:expr() {
        Expr::Lambda(name, box ty, box body)
    }

rule record_expr() -> Expr
    = LEFT_BRACE() branches:(label:ident() EQUAL() e:expr() COMMA()? {(label, box e)})* RIGHT_BRACE() {
        Expr::Record(branches)
    }

rule tuple_expr() -> Expr
    = LEFT_PAREN() head:expr() tail:(COMMA() e:expr() {e})+ RIGHT_PAREN() {
        let mut exprs = tail;
        exprs.insert(0, head);
        let branches: Vec<_> = exprs.into_iter().enumerate().map(|(i, e)| {
            (i.to_string(), box e)
        }).collect();
        Expr::Record(branches)
    }

rule variant_expr() -> Expr
    = ty:type_() DOUBLE_COLON() label:ident() LEFT_PAREN() e:expr() RIGHT_PAREN() {
        Expr::Variant(label, box e, box ty)
    }

rule list_expr() -> Expr
    = LEFT_SQUARE_BRACKET()  exprs:(expr() ** COMMA()) RIGHT_SQUARE_BRACKET() {
        Expr::List(exprs)
    }

rule println_expr() -> Expr
    = PRINTLN() e:inner_expr() { Expr::Println(box e) }

rule number() -> i32
    = n:$(['0'..='9']+) __ { n.parse().unwrap() }

rule boolean_expr() -> Expr
    = TRUE() { Expr::Bool(true) }
    / FALSE() { Expr::Bool(false) }

rule unit_expr() -> Expr
    = UNIT_V() { Expr::Unit }

rule char_expr() -> Expr
    = SINGLE_QUOTE() c:$([_]) SINGLE_QUOTE() { Expr::Char(c.chars().nth(0).unwrap()) }

rule string_expr() -> Expr
    = DOUBLE_QUOTE() s:$((!"\"" [_])*) DOUBLE_QUOTE() {
        Expr::List(s.chars().map(|c| Expr::Char(c)).collect())
    }

rule ident() -> String
    = !IS_KEYWORD() s:$(quiet!{['a'..='z'|'A'..='Z']['a'..='z'|'A'..='Z'|'0'..='9'|'_']*}) __ { s.to_string() }
    / expected!("<identifier>")

rule IS_KEYWORD()
    = TYPE() / ENUM() / MATCH() / REC() / FUNC() / IF() / ELSE() / INT() / BOOL() / TRUE() / FALSE() / UNIT_V() / PRINTLN()

rule TYPE() = "type" !ident() __
rule ENUM() = "enum" !ident() __
rule MATCH() = "match" !ident() __
rule REC() = "rec" !ident() __
rule FUNC() = "func" !ident() __
rule IF() = "if" !ident() __
rule ELSE() = "else" !ident() __
rule INT() = "Int" !ident() __
rule BOOL() = "Bool" !ident() __
rule TRUE() = "true" !ident() __
rule FALSE() = "false" !ident() __
rule UNIT_V() = "unit" !ident() __
rule PRINTLN() = "println" !ident() __

rule EQUAL() = "=" __
rule COMMA() = "," __
rule DOT() = "." __
rule COLON() = ":" __
rule DOUBLE_COLON() = "::" __
rule SEMICOLON() = ";" __
rule DOUBLE_EQUAL() = "==" __
rule NOT_EQUAL() = "/=" __
rule FAT_ARROW() = "=>" __
rule PLUS() = "+" __
rule MINUS() = "-" __
rule STAR() = "*" __
rule SLASH() = "/" __
rule SINGLE_QUOTE() = "'" __
rule DOUBLE_QUOTE() = "\"" __
rule LEFT_PAREN() = "(" __
rule RIGHT_PAREN() = ")" __
rule LEFT_BRACE() = "{" __
rule RIGHT_BRACE() = "}" __
rule LEFT_SQUARE_BRACKET() = "[" __
rule RIGHT_SQUARE_BRACKET() = "]" __
rule LEFT_ANGLE_BRACKET() = "<" __
rule RIGHT_ANGLE_BRACKET() = ">" __

rule __() = [' '|'\t'|'\r'|'\n']*

});

pub use self::rules::*;
