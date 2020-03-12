use expr::{
    Expr::{self, *},
    Literal::*,
};
use std::collections::HashSet;
use std::sync::RwLock;

lazy_static! {
    static ref FRESH_IDENT_COUNT: RwLock<i32> = RwLock::new(0);
}

fn fresh_var() -> String {
    let mut counter = FRESH_IDENT_COUNT.write().unwrap();
    let ident = format!("<fresh{}>", *counter);
    *counter += 1;
    ident
}

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

    pub fn make_name_unique(self: Expr) -> Expr {
        match self {
            Const(Number(_)) | Const(Bool(_)) | Const(Char(_)) | Const(Unit) => self,
            Const(List(es)) => Const(List(es.into_iter().map(|e| e.make_name_unique()).collect())),
            Const(Variant(label, box e, box ty)) => {
                Const(Variant(label, box e.make_name_unique(), box ty))
            }
            Const(Record(branches)) => Const(Record(
                branches
                    .into_iter()
                    .map(|(label, box e)| (label, box e.make_name_unique()))
                    .collect(),
            )),
            Var(name) => Var(name),
            Lambda(name, box ty, box e) => {
                let fresh = fresh_var();
                let e = e.subst_expr(&name, &Var(fresh.clone())).make_name_unique();
                Lambda(fresh, box ty, box e)
            }
            Apply(box e1, box e2) => Apply(box e1.make_name_unique(), box e2.make_name_unique()),
            Let(name, box e1, box e2) => {
                let e1 = e1.make_name_unique();
                let fresh = fresh_var();
                let e2 = e2.subst_expr(&name, &Var(fresh.clone())).make_name_unique();
                Let(fresh, box e1, box e2)
            }
            LetRec(name, box ty, box e1, box e2) => {
                let fresh = fresh_var();
                let fresh_expr = Var(fresh.clone());
                let e1 = e1.subst_expr(&name, &fresh_expr).make_name_unique();
                let e2 = e2.subst_expr(&name, &fresh_expr).make_name_unique();
                LetRec(fresh, box ty, box e1, box e2)
            }
            LetType(name, box ty, box e) => {
                let fresh = fresh_var();
                let e = e.subst_expr(&name, &Var(fresh)).make_name_unique();
                LetType(name, box ty, box e)
            }
            If(box cond, box e1, box e2) => If(
                box cond.make_name_unique(),
                box e1.make_name_unique(),
                box e2.make_name_unique(),
            ),
            BinOp(op, box e1, box e2) => {
                BinOp(op, box e1.make_name_unique(), box e2.make_name_unique())
            }
            Dot(box e, label) => Dot(box e.make_name_unique(), label),
            Match(box e, branches) => {
                let e = e.make_name_unique();
                let branches = branches
                    .into_iter()
                    .map(|(label, name, box e)| {
                        let fresh = fresh_var();
                        let e = e.subst_expr(&name, &Var(fresh.clone())).make_name_unique();
                        (label, fresh, box e)
                    })
                    .collect();
                Match(box e, branches)
            }
            Println(box e) => Println(box e.make_name_unique()),
        }
    }

    pub fn free_vars(&self) -> HashSet<String> {
        match self {
            Const(Number(_)) | Const(Bool(_)) | Const(Char(_)) | Const(Unit) => {
                HashSet::<String>::new()
            }
            Const(List(es)) => es
                .iter()
                .fold(HashSet::new(), |mut acc: HashSet<String>, e| {
                    acc.extend(e.free_vars());
                    acc
                }),
            Const(Variant(_, box ref e, _)) => e.free_vars(),
            Const(Record(ref branches)) => {
                branches.iter().fold(HashSet::new(), |mut acc, (_, box e)| {
                    acc.extend(e.free_vars());
                    acc
                })
            }
            Var(ref name) => {
                let mut vars = HashSet::new();
                vars.insert(name.clone());
                vars
            }
            Lambda(ref param, _, box ref e) => {
                let mut vars = e.free_vars();
                vars.remove(param);
                vars
            }
            Apply(box e1, box e2) => {
                let mut vars1 = e1.free_vars();
                let vars2 = e2.free_vars();
                vars1.extend(vars2);
                vars1
            }
            Let(name, box e1, box e2) => {
                let mut vars1 = e1.free_vars();
                let mut vars2 = e2.free_vars();
                vars2.remove(name);
                vars1.extend(vars2);
                vars1
            }
            LetRec(name, _, box e1, box e2) => {
                let mut vars1 = e1.free_vars();
                let mut vars2 = e2.free_vars();
                vars1.remove(name);
                vars2.remove(name);
                vars1.extend(vars2);
                vars1
            }
            LetType(name, _, box e) => {
                let mut vars = e.free_vars();
                vars.remove(name);
                vars
            }
            If(box cond, box e1, box e2) => {
                let mut vars = cond.free_vars();
                let vars1 = e1.free_vars();
                let vars2 = e2.free_vars();
                vars.extend(vars1);
                vars.extend(vars2);
                vars
            }
            BinOp(_, box e1, box e2) => {
                let mut vars1 = e1.free_vars();
                let vars2 = e2.free_vars();
                vars1.extend(vars2);
                vars1
            }
            Dot(box e, _) => e.free_vars(),
            Match(box _e, _branches) => unimplemented!(),
            Println(box e) => e.free_vars(),
        }
    }
}
