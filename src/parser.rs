use ast::*;
use ident::Ident;
use peg;

peg::parser!(grammar rules() for str {

pub rule type_() -> Type
    = __ ty:func_type() { ty }

rule func_type() -> Type
    = head:primitive_type() tail:(ARROW() typ:func_type() { typ })* {
        tail.into_iter().fold(head, |acc, typ| {
            Type::Func(box acc, box typ)
        })
    }

rule primitive_type() -> Type
    = record_type()
    / INT() { Type::Int }
    / BOOL() { Type::Bool }
    / CHAR() { Type::Char }
    / UNIT_T() { Type::Unit }
    / name:ident() { Type::Var(name) }
    / LEFT_PAREN() ty:func_type() RIGHT_PAREN() { ty }

rule record_type() -> Type
    = LEFT_BRACE() arms:(label:ident() COLON() ty:type_() COMMA()? { (label, ty) })* RIGHT_BRACE() {
        Type::Record(arms)
    }

pub rule program() -> Expr
    = __ e:toplevel_expr() { e }

rule toplevel_expr() -> Expr
    = start:position!() FUNC() name:ident() param_name:ident() COLON() param_type:type_()  ret_type:(COLON() typ:type_() { typ })? LEFT_BRACE() body:expr() RIGHT_BRACE() end:position!() left:toplevel_expr() {
        Expr::Let(
            name,
            box Expr::Const(Literal::Func {
                param_name: param_name,
                param_type: param_type,
                ret_type: ret_type.unwrap_or_else(|| Type::Var(Ident::fresh())),
                body: box body,
                pos: Position {start: start, end: end}
            }),
            box left)
    }
    / start:position!() REC() FUNC()  name:ident() param_name:ident() COLON() param_type:type_() COLON() ret_type:type_() LEFT_BRACE() body:expr() RIGHT_BRACE() end:position!() left:toplevel_expr() {
        Expr::LetRec(
            name, Type::Func(box param_type.clone(), box ret_type.clone()),
            box Expr::Const(Literal::Func {
                param_name: param_name,
                param_type: param_type,
                ret_type: ret_type,
                body: box body,
                pos: Position {start: start, end: end}
            }),
            box left,
            Position {start: start, end: end})
    }
    / LET() name:ident() EQUAL() init:expr() SEMICOLON() left:toplevel_expr() {
        Expr::Let(name, box init, box left)
    }
    / TYPE() name:ident() EQUAL() typ:type_() SEMICOLON() left:toplevel_expr() {
        Expr::LetType(name, typ, box left)
    }
    / expr()

rule expr() -> Expr
    = LET() name:ident() EQUAL() e1:inner_expr() SEMICOLON() e2:expr() {
        Expr::Let(name, box e1, box e2)
    }
    / TYPE() name:ident() EQUAL() typ:type_() SEMICOLON() e:expr() {
        Expr::LetType(name, typ, box e)
    }
    / es:(inner_expr() ** SEMICOLON()) {?
        let mut es = es;
        if let Some(head) = es.pop() {
            Ok(es.into_iter().rev().fold(head, |acc, e| Expr::Let(Ident::new("<dummy-sequence>"), box e, box acc)))
        } else {
            Err("no expr found")
        }
    }

rule inner_expr() -> Expr
    = if_expr()
    / binop_expr()


rule if_expr() -> Expr
    = start:position!() IF() cond:expr() LEFT_BRACE() e1:expr() RIGHT_BRACE() ELSE() LEFT_BRACE() e2:expr() RIGHT_BRACE() end:position!() {
        Expr::If(box cond, box e1, box e2, Position {start: start, end: end})
    }

rule binop_expr() -> Expr = precedence! {
    x:(@) start:position!() DOUBLE_EQUAL() end:position!() y:@ { Expr::BinOp(BinOp::Eq, box x, box y, Position {start: start, end: end}) }
    x:(@) start:position!() NOT_EQUAL() end:position!() y:@ { Expr::BinOp(BinOp::Neq, box x, box y, Position {start: start, end: end}) }
    x:(@) start:position!() LEFT_ANGLE_BRACKET() end:position!() y:@ { Expr::BinOp(BinOp::Lt, box x, box y, Position {start: start, end: end}) }
    x:(@) start:position!() RIGHT_ANGLE_BRACKET() end:position!() y:@ { Expr::BinOp(BinOp::Gt, box x, box y, Position {start: start, end: end}) }
    --
    x:(@) start:position!() PLUS() end:position!() y:@ { Expr::BinOp(BinOp::Add, box x, box y, Position {start: start, end: end}) }
    x:(@) start:position!() MINUS() end:position!() y:@ { Expr::BinOp(BinOp::Sub, box x, box y, Position {start: start, end: end}) }
    --
    x:(@) start:position!() STAR() end:position!() y:@ { Expr::BinOp(BinOp::Mult, box x, box y, Position {start: start, end: end}) }
    x:(@) start:position!() SLASH() end:position!() y:@ { Expr::BinOp(BinOp::Div, box x, box y, Position {start: start, end: end}) }
    --
    e:apply_expr() { e }
}

rule apply_expr() -> Expr
    = start:position!() e1:field_access_expr() e2:apply_expr() end:position!() {
        Expr::Apply(box e1, box e2, Position {start: start, end: end})
    }
    / field_access_expr()

rule field_access_expr() -> Expr
    = start:position!() e:factor_expr() labels:(DOT() label:ident() end:position!() { (label, end) })* {
        labels.into_iter().fold(e, |acc, (label, end)| Expr::FieldAccess(box acc, label, Position {start: start, end: end}))
    }

rule factor_expr() -> Expr
    = func_expr()
    / record_expr()
    / tuple_expr()
    / number_expr()
    / boolean_expr()
    / unit_expr()
    / char_expr()
    / println_expr()
    / var_expr()
    / LEFT_PAREN() e:expr() RIGHT_PAREN() { e }

rule func_expr() -> Expr
    = start:position!() FUNC() name:ident() COLON() typ:type_() ret_type:(COLON() typ:type_() { typ })? FAT_ARROW() body:expr() end:position!() {
        Expr::Const(Literal::Func{
            param_name: name,
            param_type: typ,
            ret_type: ret_type.unwrap_or_else(|| Type::Var(Ident::fresh())),
            body: box body,
            pos: Position {start: start, end: end}
        })
    }

rule record_expr() -> Expr
    = LEFT_BRACE() arms:(label:ident() EQUAL() e:expr() COMMA()? { (label, e) })* RIGHT_BRACE() {
        Expr::Const(Literal::Record(arms))
    }

rule tuple_expr() -> Expr
    = LEFT_PAREN() es:(expr() ** COMMA()) RIGHT_PAREN() {?
        if es.len() >= 2 {
            Ok(Expr::Const(Literal::Record(es.into_iter().enumerate().map(|(n, e)| (Ident::new(&n.to_string()), e)).collect())))
        } else {
            Err("length of tuple must be greater than 1")
        }
    }

rule number_expr() -> Expr
    = n:number() {
        Expr::Const(Literal::Number(n))
    }

rule boolean_expr() -> Expr
    = TRUE() {
        Expr::Const(Literal::Bool(true))
    }
    / FALSE() {
        Expr::Const(Literal::Bool(false))
    }

rule unit_expr() -> Expr
    = UNIT_V() {
        Expr::Const(Literal::Unit)
    }

rule char_expr() -> Expr
    = SINGLE_QUOTE() c:$([_]) SINGLE_QUOTE() {
        Expr::Const(Literal::Char(c.chars().nth(0).unwrap()))
    }

rule println_expr() -> Expr
    = PRINTLN() e:inner_expr() { Expr::Println(box e) }

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
