
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
    Equal(Box<Expr>, Box<Expr>),
    NotEqual(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Record(Vec<(String, Box<Expr>)>),
    Dot(Box<Expr>, String),
    Variant(String, Box<Expr>, Box<Type>),
    // match expr {
    //     Hoge x => x + 1,
    //     Fuga x => x * 3,
    // }
    Match(Box<Expr>, Vec<(String, String, Box<Expr>)>),
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
            &Expr::Equal(box ref e1, box ref e2) => {
                match (e1.eval(context), e2.eval(context)) {
                    (Expr::Number(l), Expr::Number(r)) => Expr::Bool(l == r),
                    (Expr::Bool(l), Expr::Bool(r)) => Expr::Bool(l == r),
                    _ => panic!("can not {:?} = {:?}", e1, e2)
                }
            },
            &Expr::NotEqual(box ref e1, box ref e2) => {
                match (e1.eval(context), e2.eval(context)) {
                    (Expr::Number(l), Expr::Number(r)) => Expr::Bool(l != r),
                    (Expr::Bool(l), Expr::Bool(r)) => Expr::Bool(l != r),
                    _ => panic!("can not {:?} = {:?}", e1, e2)
                }
            },
            &Expr::Add(box ref e1, box ref e2) => {
                match (e1.eval(context), e2.eval(context)) {
                    (Expr::Number(l), Expr::Number(r)) => Expr::Number(l+r),
                    _ => panic!("can not unnumeric values"),
                }
            },
            &Expr::Sub(box ref e1, box ref e2) => {
                match (e1.eval(context), e2.eval(context)) {
                    (Expr::Number(l), Expr::Number(r)) => Expr::Number(l-r),
                    _ => panic!("can not unnumeric values"),
                }
            },
            &Expr::Mult(box ref e1, box ref e2) => {
                match (e1.eval(context), e2.eval(context)) {
                    (Expr::Number(l), Expr::Number(r)) => Expr::Number(l*r),
                    _ => panic!("can not unnumeric values"),
                }
            },
            &Expr::Div(box ref e1, box ref e2) => {
                match (e1.eval(context), e2.eval(context)) {
                    (Expr::Number(l), Expr::Number(r)) => Expr::Number(l/r),
                    _ => panic!("can not unnumeric values"),
                }
            },
            &Expr::Dot(box ref e, ref label) => {
                match e.eval(context) {
                    Expr::Record(v) => {
                        let found = v.iter().find(|e| {
                            e.0 == label.clone()
                        });
                        if let Some(branch) = found {
                            *branch.1.clone()
                        } else {
                            panic!("not found such filed in {:?} : {}", e, label)
                        }
                    },
                    _ => panic!("can not apply dot operator for non record")
                }
            },
            &Expr::Match(box ref e, ref branches) => {
                match e {
                    &Expr::Variant(ref label, box ref e, box ref ty) => {
                        let found = branches.iter().find(|br| {
                            label.clone() == br.0
                        });
                        let ty = match ty {
                            &Type::Variant(ref v) => {
                                v.iter().find(|x| {
                                    x.0 == label.clone()
                                })
                            },
                            _ => panic!("nyan"),
                        }.unwrap();
                        if let Some(branch) = found {
                            let new_context = context.add_expr(&branch.1, e);
                            branch.2.eval(&new_context)
                        } else {
                            panic!("can not find such label in {:?}: {}", ty, label)
                        }
                    },
                    _ => panic!("can not apply match operator for non variant"),
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
            &Expr::Equal(box ref e1, box ref e2) |
            &Expr::NotEqual(box ref e1, box ref e2) => {
                match (e1.type_of(context), e2.type_of(context)) {
                    (Type::Primitive(l), Type::Primitive(r)) => {
                        if l == r {
                            Type::Primitive("Bool".to_string())
                        } else {
                            panic!("unmatch types : {:?} and {:?}", l, r)
                        }
                    },
                    _ => panic!("non primitive value!!"),
                }
            },
            &Expr::Add(box ref e1, box ref e2) |
            &Expr::Sub(box ref e1, box ref e2) |
            &Expr::Mult(box ref e1, box ref e2) |
            &Expr::Div(box ref e1, box ref e2) => {
                match (e1.type_of(context), e2.type_of(context)) {
                    (Type::Primitive(l), Type::Primitive(r)) => {
                        if l == r  && l == "Int".to_string() {
                            Type::Primitive("Int".to_string())
                        } else {
                            panic!("can not add non numeric values")
                        }
                    },
                    _ => panic!("can not ass non numeric values")
                }
            },
            &Expr::Record(ref v) => {
                Type::Record(v.iter().map(|e| {
                    (e.0.clone(), box e.1.type_of(context))
                }).collect())
            },
            // [+ tag = e] as ty
            &Expr::Variant(ref tag, box ref e, box ref ty) => {
                match ty {
                    &Type::Variant(ref v) => {
                        let found = v.iter().find(|e|{
                            e.0 == tag.clone()
                        });
                        if let Some(branch) = found {
                            let e_ty = e.type_of(context);
                            if e_ty == *branch.1 {
                                ty.clone()
                            } else {
                                panic!("not much variant type: tag {} is related to {:?}, not {:?}", branch.0, ty, branch.1)
                            }
                        } else {
                            panic!("not found such tag {} in variant {:?}", tag, ty)
                        }
                    },
                    _ => panic!("variant type specifier must be variant type")
                }
            },
            // ([* hoge = 1, fuga = 3] as [+ hoge: Int, fuga: Int]).hoge
            &Expr::Dot(box ref e, ref label) => {
                match e.eval(context) {
                    Expr::Record(v) => {
                        let found = v.iter().find(|e| {
                            e.0 == label.clone()
                        });
                        if let Some(branch) = found {
                            branch.1.type_of(context)
                        } else {
                            panic!("not found such filed in {:?} : {}", e, label)
                        }
                    },
                    _ => panic!("can not apply dot operator for non record")
                }
            },
            &Expr::Match(box ref e, ref branches) => Expr::match_typecheck(e, branches, context),
        }
    }


    fn match_typecheck(
        e: &Expr,
        branches: &Vec<(String, String, Box<Expr>)>,
        context: &Context) -> Type
    {
        let expr_branches = match e {
            &Expr::Variant(_, _, box ref ty) => {
                match ty {
                    &Type::Variant(ref v) => v.clone(),
                    _ => panic!("type error variant expr must have variant type"),
                }
            },
            _ => panic!("type error: can not match for non variant expr"),
        };
        let labels: Vec<_> = branches.iter().map(|x| x.0.clone()).collect();
        let mut ret_types = vec![];
        for label in labels {
            let expr_branch: &(String, Box<Type>) = expr_branches.iter().find(|x| {
                x.0 == label
            }).unwrap();
            let body_branch: &(String, String, Box<Expr>) = branches.iter().find(|x| {
                x.0 == label
            }).unwrap();
            let new_context = context.add_type(&body_branch.1, &*expr_branch.1);
            ret_types.push(body_branch.2.type_of(&new_context));
        }
        if ret_types.iter().all(|x| {
            x.clone() == ret_types[0]
        }) {
            ret_types[0].clone()
        } else {
            panic!("can not much all match return types")
        }
     }

    fn is_value(&self) -> bool {
        match self {
            &Expr::Number(_) | &Expr::Bool(_) => true,
            &Expr::Unit => true,
            &Expr::Lambda(_, _, _) => true,
            &Expr::Record(_) => true,
            &Expr::Variant(_, _, _) => true,
            _ => false,
        }
    }
}

