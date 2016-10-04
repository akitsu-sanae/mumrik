use expr::Expr;
use type_::Type;

#[derive(Debug, Clone)]
pub struct Context {
    value_binding: Vec<(String, Expr)>,
    type_binding: Vec<(String, Type)>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            value_binding: vec![],
            type_binding: vec![],
        }
    }

    pub fn add_expr(&self, name: &String, expr: &Expr) -> Self {
        let mut new_context = self.clone();
        new_context.value_binding.insert(0, (name.clone(), expr.clone()));
        new_context
    }

    pub fn lookup_expr(&self, name: &String) -> Expr {
        let res = self.value_binding.iter().find(|ref e| {
            e.0 == name.clone()
        }).expect("no such variable");
        res.clone().1
    }

    pub fn add_type(&self, name: &String, ty: &Type) -> Self {
        let mut new_context = self.clone();
        new_context.type_binding.insert(0, (name.clone(), ty.clone()));
        new_context
    }

    pub fn lookup_type(&self, name: &String) -> Type {
        let res = self.type_binding.iter().find(|ref e| {
            e.0 == name.clone()
        }).expect("no such type");
        res.clone().1
    }


}

