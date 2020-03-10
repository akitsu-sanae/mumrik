use expr::{
    Expr::{self, *},
    Literal::*,
};
use kazuma::program;
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

fn make_name_unique(e: Expr) -> Expr {
    match e {
        Const(Number(_)) | Const(Bool(_)) | Const(Char(_)) | Const(Unit) => e,
        Const(List(es)) => Const(List(es.into_iter().map(|e| make_name_unique(e)).collect())),
        Const(Variant(label, box e, box ty)) => {
            Const(Variant(label, box make_name_unique(e), box ty))
        }
        Const(Record(branches)) => Const(Record(
            branches
                .into_iter()
                .map(|(label, box e)| (label, box make_name_unique(e)))
                .collect(),
        )),
        Var(name) => Var(name),
        Lambda(name, box ty, box e) => {
            let fresh = fresh_var();
            let e = make_name_unique(e.subst_expr(&name, &Var(fresh.clone())));
            Lambda(fresh, box ty, box e)
        }
        Apply(box e1, box e2) => Apply(box make_name_unique(e1), box make_name_unique(e2)),
        Let(name, box e1, box e2) => {
            let e1 = make_name_unique(e1);
            let fresh = fresh_var();
            let e2 = make_name_unique(e2.subst_expr(&name, &Var(fresh.clone())));
            Let(fresh, box e1, box e2)
        }
        LetRec(name, box ty, box e1, box e2) => {
            let fresh = fresh_var();
            let fresh_expr = Var(fresh.clone());
            let e1 = make_name_unique(e1.subst_expr(&name, &fresh_expr));
            let e2 = make_name_unique(e2.subst_expr(&name, &fresh_expr));
            LetRec(fresh, box ty, box e1, box e2)
        }
        LetType(name, box ty, box e) => {
            let fresh = fresh_var();
            let e = make_name_unique(e.subst_expr(&name, &Var(fresh)));
            LetType(name, box ty, box e)
        }
        If(box cond, box e1, box e2) => If(
            box make_name_unique(cond),
            box make_name_unique(e1),
            box make_name_unique(e2),
        ),
        BinOp(op, box e1, box e2) => BinOp(op, box make_name_unique(e1), box make_name_unique(e2)),
        Dot(box e, label) => Dot(box make_name_unique(e), label),
        Match(box e, branches) => {
            let e = make_name_unique(e);
            let branches = branches
                .into_iter()
                .map(|(label, name, box e)| {
                    let fresh = fresh_var();
                    let e = make_name_unique(e.subst_expr(&name, &Var(fresh.clone())));
                    (label, fresh, box e)
                })
                .collect();
            Match(box e, branches)
        }
        Println(box e) => Println(box make_name_unique(e)),
    }
}

fn lift(e: Expr) -> (Vec<program::Func>, Vec<program::Statement>) {
    fn lift_impl(
        e: Expr,
        acc: (Vec<program::Func>, Vec<program::Statement>),
    ) -> (Vec<program::Func>, Vec<program::Statement>) {
        match e {
            Const(Number(_)) | Const(Bool(_)) | Const(Char(_)) | Const(Unit) => acc,
            _ => unimplemented!(),
        }
    }
    lift_impl(e, (vec![], vec![]))
}

pub fn codegen(e: Expr) {
    let e = make_name_unique(e);
    let funcs = lift(e);
}
