#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident(String);

impl Ident {
    pub fn new(name: &str) -> Ident {
        Ident(name.to_string())
    }
}
