use ident::Ident;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Env<T: Clone + Debug> {
    binding: Vec<(Ident, T)>,
}

impl<T: Clone + Debug> Env<T> {
    pub fn new() -> Self {
        Env { binding: vec![] }
    }

    pub fn add(&self, name: Ident, v: T) -> Self {
        let mut new_context = self.clone();
        new_context.binding.insert(0, (name, v));
        new_context
    }

    pub fn lookup(&self, name: &Ident) -> Result<T, String> {
        let res = self.binding.iter().find(|e| &e.0 == name);
        match res {
            Some(res) => Ok(res.clone().1),
            None => Err(format!("unbound: {:?} in {:?}", name, self.binding)),
        }
    }
}
