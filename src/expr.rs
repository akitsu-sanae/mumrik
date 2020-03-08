use type_::Type;

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
    pub fn is_value(&self) -> bool {
        match self {
            &Expr::Const(_) | &Expr::Lambda(_, _, _) => true,
            _ => false,
        }
    }
}

use std::fmt;

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BinOp::Add => "+",
                BinOp::Sub => "-",
                BinOp::Mult => "*",
                BinOp::Div => "/",
                BinOp::Equal => "==",
                BinOp::NotEqual => "/=",
                BinOp::LessThan => "<",
                BinOp::GreaterThan => ">",
            }
        )
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use expr::Literal::*;
        match self {
            Number(ref n) => write!(f, "{}", n),
            Bool(ref b) => write!(f, "{}", b),
            Char(ref c) => write!(f, "{}", c),
            Unit => write!(f, "unit"),
            List(ref es) => {
                write!(f, "[")?;
                for e in es.iter() {
                    write!(f, "{}, ", e)?;
                }
                write!(f, "]")
            }
            Variant(ref label, box ref e, box ref ty) => write!(f, "{}::{}({})", ty, label, e),
            Record(ref branches) => {
                write!(f, "{{ ")?;
                for &(ref label, box ref e) in branches.iter() {
                    write!(f, "{}: {}", label, e)?;
                }
                write!(f, " }}")
            }
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use expr::Expr::*;
        match self {
            Const(ref lit) => write!(f, "{}", lit),
            Var(ref name) => write!(f, "{}", name),
            Lambda(ref arg, box ref arg_ty, box ref body) => {
                write!(f, "func {}: {} => {}", arg, arg_ty, body)
            }
            Apply(box ref func, box ref arg) => write!(f, "{} {}", func, arg),
            Let(ref ident, box ref init, box ref body) => {
                write!(f, "let {} = {}; {}", ident, init, body)
            }
            LetRec(ref ident, box ref ty, box ref init, box ref body) => {
                write!(f, "rec let {}: {} = {}; {}", ident, ty, init, body)
            }
            LetType(ref ident, box ref ty, box ref body) => {
                write!(f, "type {} = {}; {}", ident, ty, body)
            }
            If(box ref cond, box ref then, box ref else_) => {
                write!(f, "if {} {{ {} }} else {{ {} }}", cond, then, else_)
            }
            BinOp(op, box ref lhs, box ref rhs) => write!(f, "{} {} {}", lhs, op, rhs),
            Dot(box ref e, ref label) => write!(f, "{}.{}", e, label),
            Match(box ref e, ref branches) => {
                write!(f, "match {} {{", e)?;
                for &(ref label, ref name, box ref e) in branches.iter() {
                    write!(f, "{} {} => {},", label, name, e)?;
                }
                write!(f, "}}")
            }
            Println(box ref e) => write!(f, "println {}", e),
        }
    }
}
