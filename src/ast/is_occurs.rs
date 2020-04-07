use super::Type;
use ident::Ident;

impl Type {
    pub fn is_occurs(&self, name: &Ident) -> bool {
        match self {
            Type::Func(box ref ty1, box ref ty2) => ty1.is_occurs(name) || ty2.is_occurs(name),
            Type::Record(ref fields) => fields.iter().any(|(_, ty)| ty.is_occurs(name)),
            Type::Var(ref name_) if name == name_ => true,
            _ => false,
        }
    }
}
