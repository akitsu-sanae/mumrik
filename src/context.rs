
use expr::Expr;

#[derive(Debug, Clone)]
pub struct Context {
    detail: Vec<(String, Expr)>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            detail: vec![],
        }
    }

    pub fn add(&self, name: &String, expr: &Expr) -> Self {
        let mut new_context = self.clone();
        new_context.detail.insert(0, (name.clone(), expr.clone()));
        new_context
    }

    pub fn loockup(&self, name: &String) -> Expr {
        let res = self.detail.iter().find(|ref e| {
            e.0 == name.clone()
        }).expect("no such variable");
        res.clone().1
    }
}

