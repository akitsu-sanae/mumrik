use expr::{self, Expr, Literal};
use type_::Type;

#[derive(Debug, Clone)]
pub enum Nexpr {
    Const(Literal),
    Var(String),
    Apply(Box<Nexpr>, Vec<Nexpr>),
    If(Box<Nexpr>, Box<Nexpr>, Box<Nexpr>),
    BinOp(expr::BinOp, Box<Nexpr>, Box<Nexpr>),
}

impl Nexpr {
    pub fn subst_expr(self, name: &str, e: &Nexpr) -> Nexpr {
        match self {
            Nexpr::Const(_) => self, // TODO
            Nexpr::Var(name_) if &name_ == name => e.clone(),
            Nexpr::Var(_) => self,
            Nexpr::Apply(box f, args) => Nexpr::Apply(
                box f.subst_expr(name, e),
                args.into_iter()
                    .map(|arg| arg.subst_expr(name, e))
                    .collect(),
            ),
            Nexpr::If(box cond, box e1, box e2) => Nexpr::If(
                box cond.subst_expr(name, e),
                box e1.subst_expr(name, e),
                box e2.subst_expr(name, e),
            ),
            Nexpr::BinOp(op, box e1, box e2) => {
                Nexpr::BinOp(op, box e1.subst_expr(name, e), box e2.subst_expr(name, e))
            }
        }
    }
}

pub struct Nfunc {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub ret_type: Type,
    pub body: Nexpr,
}

pub struct NormalForm {
    pub funcs: Vec<Nfunc>,
    pub expr: Nexpr,
}

impl NormalForm {
    pub fn from(e: Expr) -> NormalForm {
        println!("converto from: {:?}", e);
        match e {
            Expr::Const(lit) => NormalForm {
                funcs: vec![],
                expr: Nexpr::Const(lit),
            },
            Expr::Var(name) => NormalForm {
                funcs: vec![],
                expr: Nexpr::Var(name),
            },
            Expr::Lambda(_name, box _ty, box _e) => unimplemented!(),
            Expr::Apply(box e1, box e2) => {
                let mut nf1 = NormalForm::from(e1);
                let mut nf2 = NormalForm::from(e2);
                nf1.funcs.append(&mut nf2.funcs);
                NormalForm {
                    funcs: nf1.funcs,
                    expr: Nexpr::Apply(box nf1.expr, vec![nf2.expr]),
                }
            }
            Expr::Let(name, box e1, box e2) => {
                // let b = 2;
                // let a = b + 1;
                // a + b
                //
                // func b() { 2 }
                // func a(b:Int) { b + 1 }
                // func main( a(b()) + b() )
                let free_vars = e1.free_vars();
                let free_vars: Vec<_> = free_vars
                    .into_iter()
                    .map(|var| (var, Type::Int) /* TODO: */)
                    .collect();

                let mut funcs = vec![];
                let mut nf1 = NormalForm::from(e1);
                let mut nf2 = NormalForm::from(e2);
                funcs.append(&mut nf1.funcs);
                funcs.append(&mut nf2.funcs);
                funcs.push(Nfunc {
                    name: name.clone(),
                    params: free_vars.clone(),
                    ret_type: Type::Int, // TODO
                    body: nf1.expr,
                });

                let e = nf2.expr.subst_expr(
                    &name,
                    &Nexpr::Apply(
                        box Nexpr::Var(name.clone()),
                        free_vars.into_iter().map(|v| Nexpr::Var(v.0)).collect(),
                    ),
                );

                NormalForm {
                    funcs: funcs,
                    expr: e,
                }
            }
            Expr::LetRec(_name, box _ty, box _e1, box _e2) => unimplemented!(),
            Expr::LetType(_name, box _ty, box _e) => unimplemented!(),
            Expr::If(box cond, box e1, box e2) => {
                let nf_cond = NormalForm::from(cond);
                let mut nf1 = NormalForm::from(e1);
                let mut nf2 = NormalForm::from(e2);
                let mut funcs = nf_cond.funcs;
                funcs.append(&mut nf1.funcs);
                funcs.append(&mut nf2.funcs);
                NormalForm {
                    funcs: funcs,
                    expr: Nexpr::If(box nf_cond.expr, box nf1.expr, box nf2.expr),
                }
            }
            Expr::BinOp(op, box e1, box e2) => {
                let nf1 = NormalForm::from(e1);
                let mut nf2 = NormalForm::from(e2);
                let mut funcs = nf1.funcs;
                funcs.append(&mut nf2.funcs);
                NormalForm {
                    funcs: funcs,
                    expr: Nexpr::BinOp(op, box nf1.expr, box nf2.expr),
                }
            }
            Expr::Dot(box _e, _label) => unimplemented!(),
            Expr::Match(box _e, _branches) => unimplemented!(),
            Expr::Println(box _e) => unimplemented!(),
        }
    }
}
