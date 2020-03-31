use super::BinOp;
use ident::Ident;

mod subst_type;
mod util;
pub use self::util::type_of;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Const(Literal),
    Var(Ident, Type),
    Lambda(Ident, Type, Box<Expr>),
    Apply(Box<Expr>, Box<Expr>),
    LetRec(Ident, Type, Box<Expr>, Box<Expr>),
    Let(Ident, Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    FieldAccess(Box<Expr>, Ident),
    PatternMatch(Box<Expr>, Vec<PatternMatchArm>),
    Println(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Number(i32),
    Bool(bool),
    Char(char),
    Unit,

    Variant(Ident, Box<Expr>, Type),
    Record(Vec<(Ident, Expr)>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternMatchArm {
    pub label: Ident,
    pub name: Ident,
    pub body: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Bool,
    Char,
    Unit,
    Func(Box<Type>, Box<Type>),
    Record(Vec<(Ident, Type)>),
    Variant(Vec<(Ident, Type)>),
}
