
use context::Context;
use type_::Type;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Number(i64),
    Bool(bool),
    Unit,
    Var(String),
    Lambda(String, Box<Type>, Box<Expr>),
    Apply(Box<Expr>, Box<Expr>),
    Sequence(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn eval(&self, context: &Context) -> Expr {
        match self {
            &Expr::Apply(box ref f, box ref arg) => {
                match f {
                    &Expr::Lambda(ref name, _, box ref body) if arg.is_value() => {
                        let new_context = context.add_expr(name, arg);
                        body.eval(&new_context)
                    },
                    &Expr::Lambda(_, _, _) =>
                        Expr::Apply(box f.clone(), box arg.eval(context)).eval(context),
                    _ => Expr::Apply(box f.eval(context), box arg.clone()).eval(context),
                }
            },
            &Expr::Sequence(box ref e1, box ref e2) => {
                Expr::Apply(
                    box Expr::Lambda("_".to_string(), box Type::Primitive("Unit".to_string()), box e2.clone()),
                    box e1.clone()).eval(context)
            },
            &Expr::If(box ref cond, box ref tr, box ref fl) => {
                match cond.eval(context) {
                    Expr::Bool(c) => {
                        if c {
                            tr.eval(context)
                        } else {
                            fl.eval(context)
                        }
                    },
                    _ => panic!("if condition must be bool: {:?}", cond),
                }
            },
            &Expr::Var(ref name) => context.lookup_expr(name),
            _ => self.clone(),
        }
    }

    pub fn type_of(&self, context: &Context) -> Type {
        match self {
            &Expr::Number(_) => Type::Primitive("Int".to_string()),
            &Expr::Bool(_) => Type::Primitive("Bool".to_string()),
            &Expr::Unit => Type::Primitive("Unit".to_string()),
            &Expr::Var(ref name) => context.lookup_type(name),
            &Expr::Lambda(ref name, box ref ty, box ref e) => {
                let new_context = context.add_type(name, ty);
                let ret_ty = e.type_of(&new_context);
                Type::Function(box ty.clone(), box ret_ty)
            },
            &Expr::Apply(box ref e1, box ref e2) => {
                let para = e2.type_of(context);
                match e1.type_of(context) {
                    Type::Function(box arg, box ret) => {
                        if arg == para {
                            ret
                        } else {
                            panic!("not much type {:?} and {:?}", arg, para)
                        }
                    },
                    _ => panic!("can not apply to non functional type")
                }
            },
            &Expr::Sequence(box ref e1, box ref e2) => {
                if e1.type_of(context) == Type::Primitive("Unit".to_string()) {
                    e2.type_of(context)
                } else {
                    panic!("{:?} is not unit type", e1)
                }
            },
            &Expr::If(box ref cond, box ref tr, box ref fl) => {
                match cond.type_of(context) {
                    Type::Primitive(c) => {
                        if c == "Bool" {
                            let tr_ty = tr.type_of(context);
                            let fl_ty = fl.type_of(context);
                            if tr_ty == fl_ty {
                                tr_ty
                            } else {
                                panic!("unmatch type: {:?} and {:?}", tr_ty, fl_ty)
                            }
                        } else {
                            panic!("if condition must be Bool")
                        }
                    },
                    _ => panic!("if condition must be Bool")
                }
            },
        }
    }

    fn is_value(&self) -> bool {
        match self {
            &Expr::Number(_) | &Expr::Bool(_) => true,
            &Expr::Unit => true,
            &Expr::Lambda(_, _, _) => true,
            _ => false,
        }
    }
}

