use expr::{
    Expr::{self, *},
    Literal::*,
};
use type_::Type;

pub struct Let {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub ret_type: Type,
    pub body: Expr,
}

pub struct NormalForm {
    pub lets: Vec<Let>,
    pub expr: Expr,
}

impl NormalForm {
    fn expr_(e: Expr) -> NormalForm {
        NormalForm {
            lets: vec![],
            expr: e,
        }
    }
    pub fn from(e: Expr) -> NormalForm {
        match e {
            Const(Number(_)) | Const(Bool(_)) | Const(Char(_)) | Const(Unit) => {
                NormalForm::expr_(e)
            }
            Const(List(_es)) => unimplemented!(),
            Const(Variant(_, box _, box _)) => unimplemented!(),
            Const(Record(_)) => unimplemented!(),
            Var(name) => NormalForm::expr_(Var(name)),
            Lambda(_name, box _ty, box _e) => unimplemented!(),
            Apply(box e1, box e2) => {
                let mut nf1 = NormalForm::from(e1);
                let mut nf2 = NormalForm::from(e2);
                nf1.lets.append(&mut nf2.lets);
                NormalForm {
                    lets: nf1.lets,
                    expr: Apply(box nf1.expr, box nf2.expr),
                }
            }
            Let(name, box e1, box e2) => {
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
                let e2 = e2.subst_expr(
                    &name,
                    &free_vars.iter().fold(Var(name.clone()), |acc, var| {
                        Apply(box acc, box Var(var.0.clone()))
                    }),
                );

                let mut lets = vec![];
                let mut nf1 = NormalForm::from(e1);
                let mut nf2 = NormalForm::from(e2);
                lets.append(&mut nf1.lets);
                lets.append(&mut nf2.lets);
                lets.push(Let {
                    name: name,
                    params: free_vars,
                    ret_type: Type::Int, // TODO
                    body: nf1.expr,
                });
                NormalForm {
                    lets: lets,
                    expr: nf2.expr,
                }
            }
            LetRec(_name, box _ty, box _e1, box _e2) => unimplemented!(),
            LetType(_name, box _ty, box _e) => unimplemented!(),
            If(box cond, box e1, box e2) => {
                let nf_cond = NormalForm::from(cond);
                let mut nf1 = NormalForm::from(e1);
                let mut nf2 = NormalForm::from(e2);
                let mut lets = nf_cond.lets;
                lets.append(&mut nf1.lets);
                lets.append(&mut nf2.lets);
                NormalForm {
                    lets: lets,
                    expr: If(box nf_cond.expr, box nf1.expr, box nf2.expr),
                }
            }
            BinOp(op, box e1, box e2) => {
                let nf1 = NormalForm::from(e1);
                let mut nf2 = NormalForm::from(e2);
                let mut lets = nf1.lets;
                lets.append(&mut nf2.lets);
                NormalForm {
                    lets: lets,
                    expr: BinOp(op, box nf1.expr, box nf2.expr),
                }
            }
            Dot(box _e, _label) => unimplemented!(),
            Match(box _e, _branches) => unimplemented!(),
            Println(box _e) => unimplemented!(),
        }
    }
}
