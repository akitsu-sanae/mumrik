use type_::Type;

pub mod eval;
pub mod parser;
pub mod printer;

#[cfg(test)]
mod test;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mult,
    Div,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Number(i32),
    Bool(bool),
    Char(char),
    Unit,

    List(Vec<Expr>),
    Variant(String, Box<Expr>, Box<Type>),
    Record(Vec<(String, Box<Expr>)>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Const(Literal),
    Var(String),
    Lambda(String, Box<Type>, Box<Expr>),
    Apply(Box<Expr>, Box<Expr>),
    Let(String, Box<Expr>, Box<Expr>),
    LetRec(String, Box<Type>, Box<Expr>, Box<Expr>),
    LetType(String, Box<Type>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    Dot(Box<Expr>, String),
    Match(Box<Expr>, Vec<(String, String, Box<Expr>)>),
    Println(Box<Expr>),
}

impl Expr {
    fn is_value(&self) -> bool {
        match self {
            &Expr::Const(_) | &Expr::Lambda(_, _, _) => true,
            _ => false,
        }
    }
}
