use ast::parsed::{
    Expr, Func, Let, LetType, Literal, PatternMatchArm, Position, Program, RecFunc, ToplevelExpr,
    Type,
};
use ast::BinOp;
use ident::Ident;
use peg;

peg::parser!(grammar rules() for str {

pub rule type_() -> Type
    = __ ty:func_type() { ty }

rule func_type() -> Type
    = start:position!() head:primitive_type() tail:(ARROW() typ:func_type() end:position!() { (typ, end) })* {
        tail.into_iter().fold(head, |acc, (typ, end)| {
            Type::Func(box acc, box typ, Position {start: start, end: end})
        })
    }

rule primitive_type() -> Type
    = variant_type()
    / record_type()
    / start:position!() INT() end:position!() { Type::Int(Position {start: start, end:end }) }
    / start:position!() BOOL() end:position!() { Type::Bool(Position { start:start, end: end }) }
    / start:position!() CHAR() end:position!() { Type::Char(Position { start:start, end: end }) }
    / start:position!() UNIT_T() end:position!() { Type::Unit(Position {start:start, end: end}) }
    / start:position!() name:ident() end:position!() { Type::Var(name, Position {start:start, end: end}) }
    / LEFT_PAREN() ty:func_type() RIGHT_PAREN() { ty }

rule variant_type() -> Type
    = start:position!() ENUM() LEFT_BRACE() arms:(tag:ident() COLON() ty:type_() COMMA()? { (tag, ty) })+ RIGHT_BRACE() end:position!() {
        Type::Variant(arms, Position{start:start, end:end})
    }

rule record_type() -> Type
    = start:position!() LEFT_BRACE() arms:(label:ident() COLON() ty:type_() COMMA()? { (label, ty) })* RIGHT_BRACE() end:position!() {
        Type::Record(arms, Position{start: start, end: end})
    }

pub rule program() -> Program
    = __ p:toplevel_expr()* e:expr() { (p, e) }

rule toplevel_expr() -> ToplevelExpr
    = start:position!() FUNC()  name:ident() param_name:ident() COLON() param_type:type_() LEFT_BRACE() e:expr() RIGHT_BRACE() end:position!() {
        ToplevelExpr::Func(
            Func {
                name: name,
                param_name: param_name,
                param_type: param_type,
                body: e,
            },
            Position {
                start: start,
                end: end,
            }
        )
    }
    / start:position!() REC() FUNC()  name:ident() param_name:ident() COLON() param_type:type_() COLON() ret_type:type_() LEFT_BRACE() e:expr() RIGHT_BRACE() end:position!() {
        ToplevelExpr::RecFunc(
            RecFunc {
                name: name,
                param_name: param_name,
                param_type: param_type,
                ret_type: ret_type,
                body: e,
            },
            Position {
                start: start,
                end: end,
            }
        )
    }
    / start:position!() LET() name:ident() EQUAL() init:expr() SEMICOLON() end:position!() {
        ToplevelExpr::Let(
            Let{
                name: name,
                init: init,
            },
            Position {
                start: start,
                end: end,
            }
        )
    }
    / start:position!() TYPE() name:ident() EQUAL() typ:type_() SEMICOLON() end:position!() {
        ToplevelExpr::LetType(
            LetType {
                name: name,
                typ: typ
            },
            Position {
                start: start,
                end: end,
            })
    }

rule expr() -> Expr
    = start:position!() LET() name:ident() EQUAL() e1:inner_expr() SEMICOLON() e2:expr() end:position!() {
        Expr::Let(name, box e1, box e2, Position {start: start, end: end})
    }
    / start:position!() TYPE() name:ident() EQUAL() typ:type_() SEMICOLON() e:expr() end:position!() {
        Expr::LetType(name, typ, box e, Position {start: start, end: end})
    }
    / es:(inner_expr() ** SEMICOLON()) {
        let mut es = es;
        if es.len() == 1 {
            es.pop().unwrap()
        } else {
            Expr::Sequence(es)
        }
    }

rule inner_expr() -> Expr
    = if_expr()
    / match_expr()


rule if_expr() -> Expr
    = start:position!() IF() cond:expr() LEFT_BRACE() e1:expr() RIGHT_BRACE() ELSE() LEFT_BRACE() e2:expr() RIGHT_BRACE() end:position!() {
        Expr::If(box cond, box e1, box e2, Position {start: start, end: end})
    }

rule match_expr() -> Expr
    = start:position!() MATCH() e:expr() LEFT_BRACE() arms:match_arm()* RIGHT_BRACE() end:position!() {
        Expr::PatternMatch(box e, arms, Position {start: start, end: end})
    }
    / binop_expr()

rule match_arm() -> (PatternMatchArm, Position)
    = start:position!() label:ident() name:ident() FAT_ARROW() e:expr() COMMA()? end:position!() {
        (
            PatternMatchArm {
                label:label,
                name: name,
                body: e
            },
            Position {
                start: start,
                end: end
            }
        )
    }

rule binop_expr() -> Expr = precedence! {
    x:(@) DOUBLE_EQUAL() y:@ { Expr::BinOp(BinOp::Eq, box x, box y) }
    x:(@) NOT_EQUAL() y:@ { Expr::BinOp(BinOp::Neq, box x, box y) }
    x:(@) LEFT_ANGLE_BRACKET() y:@ { Expr::BinOp(BinOp::Lt, box x, box y) }
    x:(@) RIGHT_ANGLE_BRACKET() y:@ { Expr::BinOp(BinOp::Gt, box x, box y) }
    --
    x:(@) PLUS() y:@ { Expr::BinOp(BinOp::Add, box x, box y) }
    x:(@) MINUS() y:@ { Expr::BinOp(BinOp::Sub, box x, box y) }
    --
    x:(@) STAR() y:@ { Expr::BinOp(BinOp::Mult, box x, box y) }
    x:(@) SLASH() y:@ { Expr::BinOp(BinOp::Div, box x, box y) }
    --
    e:apply_expr() { e }
}

rule apply_expr() -> Expr
    = e1:field_access_expr() e2:apply_expr() {
        Expr::Apply(box e1, box e2)
    }
    / field_access_expr()

rule field_access_expr() -> Expr
    = start:position!() e:factor_expr() labels:(DOT() label:ident() end:position!() { (label, end) })* {
        labels.into_iter().fold(e, |acc, (label, end)| Expr::FieldAccess(box acc, label, Position {start: start, end: end}))
    }

rule factor_expr() -> Expr
    = lambda_expr()
    / record_expr()
    / tuple_expr()
    / variant_expr()
    / number_expr()
    / boolean_expr()
    / unit_expr()
    / char_expr()
    / println_expr()
    / var_expr()
    / LEFT_PAREN() e:expr() RIGHT_PAREN() { e }

rule lambda_expr() -> Expr
    = start:position!() FUNC() name:ident() COLON() typ:type_() FAT_ARROW() body:expr() end:position!() {
        Expr::Lambda(name, typ, box body, Position {start: start, end: end})
    }

rule record_expr() -> Expr
    = start:position!() LEFT_BRACE() arms:(label:ident() EQUAL() e:expr() COMMA()? { (label, e) })* RIGHT_BRACE() end:position!() {
        Expr::Const(Literal::Record(arms, Position {start: start, end: end}))
    }

rule tuple_expr() -> Expr
    = start:position!() LEFT_PAREN() es:(expr() ** COMMA()) RIGHT_PAREN() end:position!() {?
        if es.len() >= 2 {
            Ok(Expr::Const(Literal::Tuple(es, Position {start: start, end: end})))
        } else {
            Err("length of tuple must be greater than 1")
        }
    }

rule variant_expr() -> Expr
    = start:position!() typ:type_() DOUBLE_COLON() label:ident() LEFT_PAREN() e:expr() RIGHT_PAREN() end:position!() {
        Expr::Const(Literal::Variant(label, box e, typ, Position {start: start, end: end}))
    }

rule number_expr() -> Expr
    = start:position!() n:number() end:position!() {
        Expr::Const(Literal::Number(n, Position {start: start, end: end}))
    }

rule boolean_expr() -> Expr
    = start:position!() TRUE() end:position!() {
        Expr::Const(Literal::Bool(true, Position {start: start, end: end}))
    }
    / start:position!() FALSE() end:position!() {
        Expr::Const(Literal::Bool(false, Position {start: start, end: end}))
    }

rule unit_expr() -> Expr
    = start:position!() UNIT_V() end:position!() {
        Expr::Const(Literal::Unit(Position { start:start, end:end }))
    }

rule char_expr() -> Expr
    = start:position!() SINGLE_QUOTE() c:$([_]) SINGLE_QUOTE() end:position!() {
        Expr::Const(Literal::Char(c.chars().nth(0).unwrap(), Position {start: start, end: end}))
    }

rule println_expr() -> Expr
    = start:position!() PRINTLN() e:inner_expr() end:position!() { Expr::Println(box e, Position {start: start, end: end}) }

rule var_expr() -> Expr
    = start:position!() name:ident() end:position!() {
        Expr::Var(name, Position{start: start, end: end})
    }

rule number() -> i32
    = n:$(['0'..='9']+) __ { n.parse().unwrap() }

rule ident() -> Ident
    = !IS_KEYWORD() s:$(quiet!{['a'..='z'|'A'..='Z'|'_']['a'..='z'|'A'..='Z'|'0'..='9'|'_']*}) __ { Ident::new(s) }
    / expected!("<identifier>")

rule IS_KEYWORD()
    = TYPE() / ENUM() / MATCH() / LET() / REC() / FUNC() / IF() / ELSE() / INT() / BOOL() / TRUE() / FALSE() / UNIT_V() / PRINTLN()

rule TYPE() = "type" !ident() __
rule ENUM() = "enum" !ident() __
rule MATCH() = "match" !ident() __
rule LET() = "let" !ident() __
rule REC() = "rec" !ident() __
rule FUNC() = "func" !ident() __
rule IF() = "if" !ident() __
rule ELSE() = "else" !ident() __
rule INT() = "Int" !ident() __
rule BOOL() = "Bool" !ident() __
rule CHAR() = "Char" !ident() __
rule UNIT_T() = "Unit" !ident() __
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
rule ARROW() = "->" __
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
