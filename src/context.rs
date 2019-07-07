use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Context<T: Clone + Debug> {
    binding: Vec<(String, T)>,
}

impl<T: Clone + Debug> Context<T> {
    pub fn new() -> Self {
        Context { binding: vec![] }
    }

    pub fn add(&self, name: &String, v: &T) -> Self {
        let mut new_context = self.clone();
        new_context.binding.insert(0, (name.clone(), v.clone()));
        new_context
    }

    pub fn lookup(&self, name: &String) -> Result<T, String> {
        let res = self.binding.iter().find(|ref e| e.0 == name.clone());
        match res {
            Some(res) => Ok(res.clone().1),
            None => Err(format!("unbound: {} in {:?}", name, self.binding)),
        }
    }
}
