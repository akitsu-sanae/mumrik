
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int, Bool, Char, Unit,
    Variable(String),
    Function(Box<Type>, Box<Type>),
    Record(Vec<(String, Box<Type>)>),
    Variant(Vec<(String, Box<Type>)>),
    List(Box<Type>)
}

