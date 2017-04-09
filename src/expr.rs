
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
    Let(String, Box<Expr>, Box<Expr>),
    LetRec(String, Box<Type>, Box<Expr>, Box<Expr>),
    Fix(Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Equal(Box<Expr>, Box<Expr>),
    NotEqual(Box<Expr>, Box<Expr>),
    LessThan(Box<Expr>, Box<Expr>),
    GreaterThan(Box<Expr>, Box<Expr>),
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
    TypeAlias(String, Box<Type>, Box<Expr>),
    Println(Box<Expr>),
}

impl Expr {
    pub fn eval(&self, context: &Context<Expr>) -> Result<Expr, String> {
        match self {
            &Expr::Apply(box ref f, box ref arg) => {
                match f {
                    &Expr::Lambda(ref name, _, box ref body) if arg.is_value() => {
                        let new_context = context.add(name, arg);
                        body.eval(&new_context)
                    },
                    &Expr::Lambda(_, _, _) => {
                        let arg = try!(arg.eval(context));
                        Expr::Apply(box f.clone(), box arg).eval(context)
                    }
                    _ => {
                        let f = try!(f.eval(context));
                        Expr::Apply(box f, box arg.clone()).eval(context)
                    }
                }
            },
            &Expr::Let(ref name, box ref init, box ref after) => {
                let new_context = context.add(name, init);
                after.eval(&new_context)
            },
            &Expr::LetRec(ref name, _, box ref init, box ref body) => {
                let new_context = context.add(name, init);
                body.eval(&new_context)
            },
            &Expr::Fix(box ref e) => {
                match try!(e.eval(context)) {
                    Expr::Lambda(name, _, box body) => {
                        let new_context = context.add(&name, e);
                        body.eval(&new_context)
                    },
                    _ => {
                        if e.is_value() {
                            Ok(e.clone())
                        } else {
                            e.eval(context)
                        }
                    },
                }
            },
            &Expr::Sequence(box ref e1, box ref e2) => {
                Expr::Apply(
                    box Expr::Lambda("_".to_string(), box Type::Primitive("Unit".to_string()), box e2.clone()),
                    box e1.clone()).eval(context)
            },
            &Expr::If(box ref cond, box ref tr, box ref fl) => {
                match try!(cond.eval(context)) {
                    Expr::Bool(c) => {
                        if c {
                            tr.eval(context)
                        } else {
                            fl.eval(context)
                        }
                    },
                    _ => Err(format!("if condition must be bool: {:?}", cond))
                }
            },
            &Expr::Equal(box ref e1, box ref e2) => {
                match (try!(e1.eval(context)), try!(e2.eval(context))) {
                    (Expr::Number(l), Expr::Number(r)) => Ok(Expr::Bool(l == r)),
                    (Expr::Bool(l), Expr::Bool(r)) => Ok(Expr::Bool(l == r)),
                    _ => Err(format!("can not {:?} = {:?}", e1, e2))
                }
            },
            &Expr::NotEqual(box ref e1, box ref e2) => {
                match (try!(e1.eval(context)), try!(e2.eval(context))) {
                    (Expr::Number(l), Expr::Number(r)) => Ok(Expr::Bool(l != r)),
                    (Expr::Bool(l), Expr::Bool(r)) => Ok(Expr::Bool(l != r)),
                    _ => Err(format!("can not {:?} = {:?}", e1, e2))
                }
            },
            &Expr::LessThan(box ref e1, box ref e2) => {
                match (try!(e1.eval(context)), try!(e2.eval(context))) {
                    (Expr::Number(l), Expr::Number(r)) => Ok(Expr::Bool(l < r)),
                    _ => Err(format!("can not compare unnumeric values"))
                }
            },
            &Expr::GreaterThan(box ref e1, box ref e2) => {
                match (try!(e1.eval(context)), try!(e2.eval(context))) {
                    (Expr::Number(l), Expr::Number(r)) => Ok(Expr::Bool(l > r)),
                    _ => Err(format!("can not compare unnumeric values"))
                }
            },
            &Expr::Add(box ref e1, box ref e2) => {
                match (try!(e1.eval(context)), try!(e2.eval(context))) {
                    (Expr::Number(l), Expr::Number(r)) => Ok(Expr::Number(l+r)),
                    _ => Err(format!("can not unnumeric values")),
                }
            },
            &Expr::Sub(box ref e1, box ref e2) => {
                match (try!(e1.eval(context)), try!(e2.eval(context))) {
                    (Expr::Number(l), Expr::Number(r)) => Ok(Expr::Number(l-r)),
                    _ => Err(format!("can not unnumeric values")),
                }
            },
            &Expr::Mult(box ref e1, box ref e2) => {
                match (try!(e1.eval(context)), try!(e2.eval(context))) {
                    (Expr::Number(l), Expr::Number(r)) => Ok(Expr::Number(l*r)),
                    _ => Err(format!("can not unnumeric values")),
                }
            },
            &Expr::Div(box ref e1, box ref e2) => {
                match (try!(e1.eval(context)), try!(e2.eval(context))) {
                    (Expr::Number(l), Expr::Number(r)) => Ok(Expr::Number(l/r)),
                    _ => Err(format!("can not unnumeric values")),
                }
            },
            &Expr::Dot(box ref e, ref label) => {
                match try!(e.eval(context)) {
                    Expr::Record(v) => {
                        let found = v.iter().find(|e| {
                            e.0 == label.clone()
                        });
                        if let Some(branch) = found {
                            Ok(*branch.1.clone())
                        } else {
                            Err(format!("not found such filed in {:?} : {}", e, label))
                        }
                    },
                    _ => Err(format!("can not apply dot operator for non record"))
                }
            },
            &Expr::Match(box ref e, ref branches) => {
                match e {
                    &Expr::Variant(ref label, box ref e, box ref ty) => {
                        let found = branches.iter().find(|br| {
                            label.clone() == br.0
                        });
                        if let Some(branch) = found {
                            let new_context = context.add(&branch.1, e);
                            branch.2.eval(&new_context)
                        } else {
                            Err(format!("can not find such label in {:?}: {}", ty, label))
                        }
                    },
                    _ => Err(format!("can not apply match operator for non variant")),
                }
            },
            &Expr::Println(box ref e) => {
                match e.eval(context) {
                    Ok(e) => {
                        match try!(e.eval(context)) {
                            Expr::Number(n) => println!("{}", n),
                            Expr::Bool(b) => println!("{}", b),
                            Expr::Unit => println!("unit"),
                            Expr::Lambda(name, box ty, box e) => println!("func {}: {:?} -> {:?}", name, ty, e),
                            Expr::Record(branches) => {
                                print!("{{");
                                for branch in branches {
                                    print!("{}: {:?}, ", branch.0, branch.1)
                                }
                                println!("}}")
                            },
                            Expr::Variant(label, box expr, box ty) => {
                                print!("{:?}::{}({:?})", ty, label, expr)
                            },
                            _ => panic!("internal error: {:?} is not value", e)
                        };
                        Ok(Expr::Unit)
                    },
                    Err(e) => Err(e)
                }
                            },
            &Expr::TypeAlias(_, _, box ref e) => e.eval(context),
            &Expr::Var(ref name) => context.lookup(name),
            _ => Ok(self.clone()),
        }
    }

    pub fn type_of(&self, context: &Context<Type>) -> Type {
        match self {
            &Expr::Number(_) => Type::Primitive("Int".to_string()),
            &Expr::Bool(_) => Type::Primitive("Bool".to_string()),
            &Expr::Unit => Type::Primitive("Unit".to_string()),
            &Expr::Var(ref name) => context.lookup(name).unwrap(),
            &Expr::Lambda(ref name, box ref ty, box ref e) => {
                let ty = Expr::desugar_type(ty, context);
                let new_context = context.add(name, &ty);
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
            &Expr::Let(ref name, box ref init, box ref after) => {
                let new_context = context.add(name, &init.type_of(context));
                after.type_of(&new_context)
            },
            &Expr::LetRec(ref name, box ref ty, box ref init, box ref body) => {
                let ty = Expr::desugar_type(ty, context);
                let new_context = context.add(name, &ty);
                assert!(init.type_of(&new_context) == ty);
                body.type_of(&new_context)
            }
            &Expr::Fix(box ref e) => {
                let t = e.type_of(context);
                match t.clone() {
                    Type::Function(box ty1, box ty2) => {
                        if ty1 == ty2 {
                            ty1
                        } else {
                            panic!("can not apply fix to this type: {:?}", t)
                        }
                    },
                    _ => panic!("can not fix unfunctional type"),
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
            &Expr::NotEqual(box ref e1, box ref e2) |
            &Expr::LessThan(box ref e1, box ref e2) |
            &Expr::GreaterThan(box ref e1, box ref e2) => {
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
            &Expr::Variant(ref tag, box ref e, box ref ty) => {
                match Expr::desugar_type(ty, context) {
                    Type::Variant(v) => {
                        let found = v.iter().find(|e|{
                            e.0 == tag.clone()
                        });
                        if let Some(branch) = found {
                            let e_ty = Expr::desugar_type(&e.type_of(context), context);
                            let branch_ty = Expr::desugar_type(branch.1.as_ref(), context);
                            if e_ty == branch_ty {
                                ty.clone()
                            } else {
                                panic!("not much variant type: tag {} is related to {:?}, not {:?}", branch.0, e_ty, branch.1)
                            }
                        } else {
                            panic!("not found such tag {} in variant {:?}", tag, ty)
                        }
                    },
                    _ => panic!("variant type specifier must be variant type")
                }
            },
            &Expr::Dot(box ref e, ref label) => {
                match e.type_of(context) {
                    Type::Record(branches) => {
                        if let Some(branch) = branches.iter().find(|e| {
                            e.0 == label.clone()
                        }) {
                            *branch.1.clone()
                        } else {
                            panic!("not found such filed in {:?} : {}", e, label)
                        }
                    },
                    _ => panic!("can not apply dot operator for non record")
                }
            },
            &Expr::Match(box ref e, ref branches) => Expr::match_typecheck(e, branches, context),
            &Expr::Println(box ref e) => {
                e.type_of(context);
                Type::Primitive("Unit".to_string())
            },
            &Expr::TypeAlias(ref name, box ref ty, box ref e) => {
                let new_context = context.add(name, &Expr::desugar_type(&ty, context));
                e.type_of(&new_context)
            },
        }
    }


    fn match_typecheck(
        e: &Expr,
        branches: &Vec<(String, String, Box<Expr>)>,
        context: &Context<Type>) -> Type
    {
        let expr_branches = match e {
            &Expr::Variant(_, _, box ref ty) => {
                match Expr::desugar_type(ty, context) {
                    Type::Variant(v) => v.clone(),
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
            let new_context = context.add(&body_branch.1, &Expr::desugar_type(&*expr_branch.1, context));
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

    fn desugar_type(ty: &Type, context: &Context<Type>) -> Type {
        match ty {
            &Type::Primitive(ref x) => {
                if let Ok(ty) = context.lookup(x) {
                    ty
                } else {
                    ty.clone()
                }
            },
            _ => ty.clone()
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

