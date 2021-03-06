use super::*;
use std::fmt;

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Const(ref lit) => write!(f, "{}", lit),
            Expr::Var(ref name, ref typ, _) => write!(f, "{} as {}", name, typ),
            Expr::Func {
                name,
                param_name,
                param_type,
                ret_type,
                box body,
                box left,
                pos: _,
            } => write!(
                f,
                "let rec {} = (func {}:{} :{} => {}); {}",
                name, param_name, param_type, ret_type, body, left
            ),
            Expr::Apply(box ref e1, box ref e2, _) => write!(f, "({}) ({})", e1, e2),
            Expr::Let(ref name, ref typ, box ref e1, box ref e2, _) => {
                write!(f, "let {}: {} = {}; {}", name, typ, e1, e2)
            }
            Expr::LetType(ref name, ref typ, box ref e) => {
                write!(f, "let type {} = {}; {}", name, typ, e)
            }
            Expr::If(box cond, box e1, box e2, _) => {
                write!(f, "if {} {{ {} }} else {{ {} }}", cond, e1, e2)
            }
            Expr::BinOp(ref op, box ref e1, box ref e2, _) => write!(f, "({}) {} ({})", e1, op, e2),
            Expr::RecordGet(box ref e, _, ref label, _) => write!(f, "({}).{}", e, label),
            Expr::ArrayGet(box ref e1, box ref e2, _) => write!(f, "{}[{}]", e1, e2),
            Expr::Assign(box ref e1, box ref e2, _) => write!(f, "{} <- {}", e1, e2),
            Expr::Println(box ref e) => write!(f, "println {}", e),
            Expr::EmptyMark => unreachable!(),
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Number(ref n) => write!(f, "{}", n),
            Literal::Bool(ref b) => write!(f, "{}", b),
            Literal::Char(ref c) => write!(f, "{}", c),
            Literal::Unit => write!(f, "unit"),
            Literal::Record(ref fields) => {
                write!(f, "{{")?;
                for (ref label, ref e) in fields.iter() {
                    write!(f, "{} = {},", label, e)?;
                }
                write!(f, "}}")
            }
            Literal::Array(ref elems, _) => {
                write!(f, "[")?;
                for e in elems {
                    write!(f, "{}, ", e)?;
                }
                write!(f, "]")
            }
        }
    }
}

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
                BinOp::Eq => "==",
                BinOp::Neq => "/=",
                BinOp::Lt => "<",
                BinOp::Gt => ">",
            }
        )
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Int => write!(f, "Int"),
            Type::Bool => write!(f, "Bool"),
            Type::Char => write!(f, "Char"),
            Type::Unit => write!(f, "Unit"),
            Type::Func(box ref typ1, box ref typ2) => write!(f, "{} -> ({})", typ1, typ2),
            Type::Record(ref fields) => {
                write!(f, "{{")?;
                for (ref label, ref typ) in fields.iter() {
                    write!(f, "{}: {},", label, typ)?;
                }
                write!(f, "}}")
            }
            Type::Array(box ref elem_typ, ref size) => write!(f, "[{}; {}]", elem_typ, size),
            Type::Var(ref name) => write!(f, "{}", name),
            Type::EmptyMark => unreachable!(),
        }
    }
}
