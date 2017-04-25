use std::collections::HashMap;
use context::Context;
use type_::Type;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Number(i64),
    Bool(bool),
    Char(char),
    Unit,
    List(Vec<Expr>),
    Var(String),
    Lambda(String, Box<Type>, Box<Expr>),
    Apply(Box<Expr>, Box<Expr>),
    Sequence(Box<Expr>, Box<Expr>),
    Let(String, Box<Expr>, Box<Expr>),
    LetRec(String, Box<Type>, Box<Expr>, Box<Expr>),
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
            &Expr::Sequence(box ref e1, box ref e2) => {
                Expr::Apply(
                    box Expr::Lambda("_".to_string(), box Type::Unit, box e2.clone()),
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
            &Expr::Var(ref name) => context.lookup(name),
            _ => Ok(self.clone()),
        }
    }

    pub fn subst_typealias(&mut self, alias: &HashMap<String, Type>) {
        use expr::Expr::*;
        match *self {
            List(ref mut exprs) => {
                for expr in exprs {
                    expr.subst_typealias(alias);
                }
            },
            Lambda(_, box ref mut ty, box ref mut expr) |
            Variant(_, box ref mut expr, box ref mut ty) => {
                ty.subst(alias); expr.subst_typealias(alias)
            },
            LetRec(_, box ref mut ty, box ref mut e, box ref mut body) => {
                ty.subst(alias);
                e.subst_typealias(alias);
                body.subst_typealias(alias);
            },
            If(box ref mut cond, box ref mut tr, box ref mut fl) => {
                cond.subst_typealias(alias);
                tr.subst_typealias(alias);
                fl.subst_typealias(alias);
            },
            Let(_, box ref mut e1, box ref mut e2) |
            Apply(box ref mut e1, box ref mut e2) |
            Sequence(box ref mut e1, box ref mut e2) |
            Equal(box ref mut e1, box ref mut e2) | NotEqual(box ref mut e1, box ref mut e2) |
            LessThan(box ref mut e1, box ref mut e2) | GreaterThan(box ref mut e1, box ref mut e2) |
            Add(box ref mut e1, box ref mut e2) | Sub(box ref mut e1, box ref mut e2) |
            Mult(box ref mut e1, box ref mut e2) | Div(box ref mut e1, box ref mut e2) => {
                e1.subst_typealias(alias);
                e2.subst_typealias(alias);
            },
            Dot(box ref mut e, _) | Println(box ref mut e) => e.subst_typealias(alias),
            Record(ref mut params) => {
                for &mut (_, ref mut e) in params.iter_mut() {
                    e.subst_typealias(alias);
                }
            },
            Match(box ref mut e, ref mut branches) => {
                e.subst_typealias(alias);
                for &mut (_, _, box ref mut e) in branches {
                    e.subst_typealias(alias)
                }
            },
            Number(_) | Bool(_) | Char(_) | Unit | Var(_) => (),
        }
    }

    fn is_value(&self) -> bool {
        match self {
            &Expr::Number(_) | &Expr::Bool(_) | &Expr::Char(_) => true,
            &Expr::Unit => true,
            &Expr::Lambda(_, _, _) => true,
            &Expr::Record(_) => true,
            &Expr::Variant(_, _, _) => true,
            &Expr::List(_) => true,
            _ => false,
        }
    }
}

