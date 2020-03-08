use super::Type;
use std::fmt;

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use type_::Type::*;
        match *self {
            Int => write!(f, "Int"),
            Bool => write!(f, "Bool"),
            Char => write!(f, "Char"),
            Unit => write!(f, "Unit"),
            Variable(ref name) => write!(f, "{}", name),
            Function(box ref from, box ref to) => write!(f, "{} -> {}", from, to),
            Record(ref data) => {
                write!(f, "{{")?;
                for &(ref label, ref ty) in data {
                    write!(f, "{}: {}", label, ty)?;
                }
                write!(f, "}}")
            }
            Variant(ref data) => {
                write!(f, "{{")?;
                for &(ref label, ref ty) in data {
                    write!(f, "{}: {}", label, ty)?;
                }
                write!(f, "}}")
            }
            List(box ref ty) => write!(f, "List[{}]", ty),
        }
    }
}
