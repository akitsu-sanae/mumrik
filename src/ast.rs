use ident::Ident;
use std::collections::HashMap;

mod free_vars;
mod is_occurs;
mod printer;
mod subst;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    pub start: usize,
    pub end: usize,
}

impl Position {
    pub fn dummy() -> Self {
        Position { start: 0, end: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Import {
    pub dirs: Vec<Ident>,
    pub module_name: Ident,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub imports: Vec<Import>,
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Const(Literal),
    Var(Ident, Type, Position),
    Func {
        name: Ident,
        param_name: Ident,
        param_type: Type,
        ret_type: Type,
        body: Box<Expr>,
        left: Box<Expr>,
        pos: Position,
    },
    Apply(Box<Expr>, Box<Expr>, Position),
    Let(Ident, Type, Box<Expr>, Box<Expr>, Position),
    LetType(Ident, Type, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>, Position),
    BinOp(BinOp, Box<Expr>, Box<Expr>, Position),
    FieldAccess(Box<Expr>, Type, Ident, Position),
    Println(Box<Expr>),
    EmptyMark,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Number(i32),
    Bool(bool),
    Char(char),
    Unit,
    Record(HashMap<Ident, Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mult,
    Div,
    Eq,
    Neq,
    Lt,
    Gt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Bool,
    Char,
    Unit,
    Func(Box<Type>, Box<Type>),
    Record(HashMap<Ident, Type>),
    Var(Ident),
    EmptyMark,
}
