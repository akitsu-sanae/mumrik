use ident::Ident;

mod subst;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Const(Literal),
    Var(Ident, Position),
    Apply(Box<Expr>, Box<Expr>, Position),
    Let(Ident, Box<Expr>, Box<Expr>),
    LetRec(Ident, Type, Box<Expr>, Box<Expr>, Position),
    LetType(Ident, Type, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>, Position),
    BinOp(BinOp, Box<Expr>, Box<Expr>, Position),
    FieldAccess(Box<Expr>, Ident, Position),
    Println(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Func(Ident, Type, Option<Type>, Box<Expr>),
    Number(i32),
    Bool(bool),
    Char(char),
    Unit,
    Record(Vec<(Ident, Expr)>),
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
    Record(Vec<(Ident, Type)>),
    Var(Ident),
}
