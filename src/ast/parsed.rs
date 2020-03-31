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
    Func(Func, Position),
    RecFunc(RecFunc, Position),
    Let(Let, Position),
    LetType(LetType, Position),
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
    Lambda(Ident, Type, Box<Expr>, Position),
    Apply(Box<Expr>, Box<Expr>),
    Let(Ident, Box<Expr>, Box<Expr>, Position),
    LetType(Ident, Type, Box<Expr>, Position),
    If(Box<Expr>, Box<Expr>, Box<Expr>, Position),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    Sequence(Vec<Expr>),
    FieldAccess(Box<Expr>, Ident, Position),
    PatternMatch(Box<Expr>, Vec<(PatternMatchArm, Position)>, Position),
    Println(Box<Expr>, Position),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Number(i32, Position),
    Bool(bool, Position),
    Char(char, Position),
    Unit(Position),

    Variant(Ident, Box<Expr>, Type, Position),
    Record(Vec<(Ident, Expr)>, Position),
    Tuple(Vec<Expr>, Position),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int(Position),
    Bool(Position),
    Char(Position),
    Unit(Position),
    Var(Ident, Position),
    Func(Box<Type>, Box<Type>, Position),
    Record(Vec<(Ident, Type)>, Position),
    Variant(Vec<(Ident, Type)>, Position),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternMatchArm {
    pub label: Ident,
    pub name: Ident,
    pub body: Expr,
}
