
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Primitive(String),
    Function(Box<Type>, Box<Type>),
    Record(Vec<(String, Box<Type>)>),
    Variant(Vec<(String, Box<Type>)>)
}

