use std::fmt;

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Const(ref lit) => write!(f, "{}", lit),
            Expr::Var(ref name, _) => write!(f, "{:?}", name),
            Expr::Apply(box ref e1, box ref e2) => write!(f, "({} {})", e1, e2),
            Expr::Let(ref name, box ref e1, box ref e2) => {
                write!(f, "let {} = {}; {}", name, e1, e2)
            }
        }
    }
}
