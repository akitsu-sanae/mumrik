use super::BinOp;
use ident::Ident;

mod subst_type;

pub type Program = (Vec<ToplevelExpr>, Expr);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToplevelExpr {
    Func(Func),
    RecFunc(RecFunc),
    Let(Let),
    LetType(LetType),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Func {
    pub name: Ident,
    pub param_name: Ident,
    pub param_type: Type,
    pub body: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecFunc {
    pub name: Ident,
    pub param_name: Ident,
    pub param_type: Type,
    pub ret_type: Type,
    pub body: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Let {
    pub name: Ident,
    pub init: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetType {
    pub name: Ident,
    pub typ: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Const(Literal),
    Var(Ident, Position),
    Lambda(Ident, Type, Box<Expr>),
    Apply(Box<Expr>, Box<Expr>, Position),
    Let(Ident, Box<Expr>, Box<Expr>),
    LetType(Ident, Type, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>, Position),
    BinOp(BinOp, Box<Expr>, Box<Expr>, Position),
    Sequence(Vec<Expr>),
    FieldAccess(Box<Expr>, Ident, Position),
    PatternMatch(Box<Expr>, Vec<PatternMatchArm>, Position),
    Println(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Number(i32),
    Bool(bool),
    Char(char),
    Unit,

    Variant(Ident, Box<Expr>, Type, Position),
    Record(Vec<(Ident, Expr)>),
    Tuple(Vec<Expr>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Bool,
    Char,
    Unit,
    Var(Ident, Position),
    Func(Box<Type>, Box<Type>),
    Record(Vec<(Ident, Type)>),
    Variant(Vec<(Ident, Type)>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternMatchArm {
    pub label: Ident,
    pub name: Ident,
    pub body: Expr,
}
