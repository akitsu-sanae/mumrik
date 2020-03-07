use super::{BinOp, Expr};
use context::Context;
use type_::Type;

impl BinOp {
    fn eval(&self, e1: &Expr, e2: &Expr) -> Result<Expr, String> {
        match (self, e1, e2) {
            (&BinOp::Equal, &Expr::Number(ref n1), &Expr::Number(ref n2)) => {
                Ok(Expr::Bool(n1 == n2))
            }
            (&BinOp::Equal, &Expr::Bool(ref b1), &Expr::Bool(ref b2)) => Ok(Expr::Bool(b1 == b2)),

            (&BinOp::NotEqual, &Expr::Number(ref n1), &Expr::Number(ref n2)) => {
                Ok(Expr::Bool(n1 != n2))
            }
            (&BinOp::NotEqual, &Expr::Bool(ref b1), &Expr::Bool(ref b2)) => {
                Ok(Expr::Bool(b1 != b2))
            }

            (&BinOp::LessThan, &Expr::Number(ref n1), &Expr::Number(ref n2)) => {
                Ok(Expr::Bool(n1 < n2))
            }
            (&BinOp::GreaterThan, &Expr::Number(ref n1), &Expr::Number(ref n2)) => {
                Ok(Expr::Bool(n1 > n2))
            }

            (&BinOp::Add, &Expr::Number(ref n1), &Expr::Number(ref n2)) => {
                Ok(Expr::Number(n1 + n2))
            }
            (&BinOp::Sub, &Expr::Number(ref n1), &Expr::Number(ref n2)) => {
                Ok(Expr::Number(n1 - n2))
            }
            (&BinOp::Mult, &Expr::Number(ref n1), &Expr::Number(ref n2)) => {
                Ok(Expr::Number(n1 * n2))
            }
            (&BinOp::Div, &Expr::Number(ref n1), &Expr::Number(ref n2)) => {
                Ok(Expr::Number(n1 / n2))
            }
            (op, e1, e2) => Err(format!("cannot {} for {} and {}", op, e1, e2)),
        }
    }
}

impl Expr {
    pub fn eval(&self, context: &Context<Expr>) -> Result<Expr, String> {
        match self {
            &Expr::Apply(box ref f, box ref arg) => match f {
                &Expr::Lambda(ref name, _, box ref body) if arg.is_value() => {
                    let new_context = context.add(name, arg);
                    body.eval(&new_context)
                }
                &Expr::Lambda(_, _, _) => {
                    let arg = arg.eval(context)?;
                    Expr::Apply(box f.clone(), box arg).eval(context)
                }
                _ => {
                    let f = f.eval(context)?;
                    Expr::Apply(box f, box arg.clone()).eval(context)
                }
            },
            &Expr::Let(ref name, box ref init, box ref after) => {
                let new_context = context.add(name, init);
                after.eval(&new_context)
            }
            &Expr::LetRec(ref name, _, box ref init, box ref body) => {
                let new_context = context.add(name, init);
                body.eval(&new_context)
            }
            &Expr::LetType(_, _, box ref body) => body.eval(&context),
            &Expr::Sequence(box ref e1, box ref e2) => Expr::Apply(
                box Expr::Lambda("_".to_string(), box Type::Unit, box e2.clone()),
                box e1.clone(),
            )
            .eval(context),
            &Expr::If(box ref cond, box ref tr, box ref fl) => match cond.eval(context)? {
                Expr::Bool(c) => {
                    if c {
                        tr.eval(context)
                    } else {
                        fl.eval(context)
                    }
                }
                _ => Err(format!("if condition must be bool: {:?}", cond)),
            },
            &Expr::BinOp(op, box ref e1, box ref e2) => {
                op.eval(&e1.eval(context)?, &e2.eval(context)?)
            }
            &Expr::Dot(box ref e, ref label) => match e.eval(context)? {
                Expr::Record(v) => {
                    let found = v.iter().find(|e| e.0 == label.clone());
                    if let Some(branch) = found {
                        Ok(*branch.1.clone())
                    } else {
                        Err(format!("not found such filed in {:?} : {}", e, label))
                    }
                }
                _ => Err(format!("can not apply dot operator for non record")),
            },
            &Expr::Match(box ref e, ref branches) => match e {
                &Expr::Variant(ref label, box ref e, box ref ty) => {
                    let found = branches.iter().find(|br| label.clone() == br.0);
                    if let Some(branch) = found {
                        let new_context = context.add(&branch.1, e);
                        branch.2.eval(&new_context)
                    } else {
                        Err(format!("can not find such label in {:?}: {}", ty, label))
                    }
                }
                _ => Err(format!("can not apply match operator for non variant")),
            },
            &Expr::Println(box ref e) => match e.eval(context) {
                Ok(e) => {
                    match e.eval(context)? {
                        Expr::Number(n) => println!("{}", n),
                        Expr::Bool(b) => println!("{}", b),
                        Expr::Unit => println!("unit"),
                        Expr::Lambda(name, box ty, box e) => {
                            println!("func {}: {:?} -> {:?}", name, ty, e)
                        }
                        Expr::Record(branches) => {
                            print!("{{");
                            for branch in branches {
                                print!("{}: {:?}, ", branch.0, branch.1)
                            }
                            println!("}}")
                        }
                        Expr::Variant(label, box expr, box ty) => {
                            print!("{:?}::{}({:?})", ty, label, expr)
                        }
                        _ => panic!("internal error: {:?} is not value", e),
                    };
                    Ok(Expr::Unit)
                }
                Err(e) => Err(e),
            },
            &Expr::Var(ref name) => context.lookup(name),
            _ => Ok(self.clone()),
        }
    }
}
