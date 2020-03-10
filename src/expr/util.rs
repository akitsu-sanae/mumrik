use expr::{
    Expr::{self, *},
    Literal::*,
};

impl Expr {
    pub fn subst_expr(self, name: &str, e: &Expr) -> Expr {
        match self {
            Const(Number(_)) | Const(Bool(_)) | Const(Char(_)) | Const(Unit) => self,
            Const(List(es)) => Const(List(
                es.into_iter().map(|e_| e_.subst_expr(name, e)).collect(),
            )),
            Const(Variant(label, box e_, box ty)) => {
                Const(Variant(label, box e_.subst_expr(name, e), box ty))
            }
            Const(Record(branches)) => Const(Record(
                branches
                    .into_iter()
                    .map(|(label, box e_)| (label, box e_.subst_expr(name, e)))
                    .collect(),
            )),
            Var(name_) if name == &name_ => e.clone(),
            Var(name_) => Var(name_),
            Lambda(param, box ty, box e_) if name != &param => {
                Lambda(param, box ty, box e_.subst_expr(name, e))
            }
            Lambda(param, box ty, box e_) => Lambda(param, box ty, box e_),
            Apply(box e1, box e2) => Apply(box e1.subst_expr(name, e), box e2.subst_expr(name, e)),
            Let(name_, box e1, box e2) if name != &name_ => Let(
                name_,
                box e1.subst_expr(name, e),
                box e2.subst_expr(name, e),
            ),
            Let(name, box e1, box e2) => Let(name, box e1, box e2),
            LetRec(name_, box ty, box e1, box e2) if name != &name_ => LetRec(
                name_,
                box ty,
                box e1.subst_expr(name, e),
                box e2.subst_expr(name, e),
            ),
            LetRec(name, box ty, box e1, box e2) => LetRec(name, box ty, box e1, box e2),
            LetType(name_, box ty, box e_) if name != &name_ => {
                LetType(name_, box ty, box e_.subst_expr(name, e))
            }
            LetType(name, box ty, box e) => LetType(name, box ty, box e),
            If(box cond, box e1, box e2) => If(
                box cond.subst_expr(name, e),
                box e1.subst_expr(name, e),
                box e2.subst_expr(name, e),
            ),
            BinOp(op, box e1, box e2) => {
                BinOp(op, box e1.subst_expr(name, e), box e2.subst_expr(name, e))
            }
            Dot(box e_, label) => Dot(box e_.subst_expr(name, e), label),
            Match(box e_, branches) => Match(
                box e_.subst_expr(name, e),
                branches
                    .into_iter()
                    .map(|(label, name_, box body)| {
                        let body = if name == &name_ {
                            body
                        } else {
                            body.subst_expr(name, e)
                        };
                        (label, name_, box body)
                    })
                    .collect(),
            ),
            Println(box e_) => Println(box e_.subst_expr(name, e)),
        }
    }
}
