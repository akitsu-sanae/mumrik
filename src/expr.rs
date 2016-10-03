
use context::Context;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Number(i64),
    Bool(bool),
    Unit,
    Var(String),
    Lambda(String, Box<Expr>),
    Apply(Box<Expr>, Box<Expr>),
    Sequence(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn eval(&self, context: &Context) -> Expr {
        match self {
            &Expr::Apply(box ref f, box ref arg) => {
                match f {
                    &Expr::Lambda(ref name, box ref body) if arg.is_value() => {
                        let new_context = context.add(name, arg);
                        body.eval(&new_context)
                    },
                    &Expr::Lambda(_, _) =>
                        Expr::Apply(box f.clone(), box arg.eval(context)).eval(context),
                    _ => Expr::Apply(box f.eval(context), box arg.clone()).eval(context),
                }
            },
            &Expr::Sequence(box ref e1, box ref e2) => {
                Expr::Apply(
                    box Expr::Lambda("_".to_string(), box e2.clone()),
                    box e1.clone()).eval(context)
            },
            &Expr::If(box ref cond, box ref tr, box ref fl) => {
                match cond {
                    &Expr::Bool(c) => {
                        if c {
                            tr.eval(context)
                        } else {
                            fl.eval(context)
                        }
                    },
                    _ => panic!("if condition must be bool: {:?}", cond),
                }
            },
            &Expr::Var(ref name) => context.loockup(name),
            _ => self.clone(),
        }
    }

    fn is_value(&self) -> bool {
        match self {
            &Expr::Number(_) | &Expr::Bool(_) => true,
            &Expr::Unit => true,
            &Expr::Lambda(_, _) => true,
            _ => false,
        }
    }
}

