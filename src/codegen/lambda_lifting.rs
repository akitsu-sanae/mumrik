use ast::*;
use ident::Ident;

pub fn lift(e: Expr) -> Expr {
    let (f, e) = lift_impl(e);
    f(e)
}

fn lift_func_case(
    func_name: Ident,
    param_name: Ident,
    param_type: Type,
    ret_type: Type,
    body: Expr,
    pos: Position,
) -> (Box<dyn Fn(Expr) -> Expr>, Expr) {
    let func_type = Type::Func(box param_type.clone(), box ret_type.clone());
    let (f, body) = lift_impl(body);
    let var = Expr::Var(func_name.clone(), func_type.clone(), pos);
    (
        box move |e| {
            Expr::LetRec(
                func_name.clone(),
                func_type.clone(),
                box Expr::Const(Literal::Func {
                    param_name: param_name.clone(),
                    param_type: param_type.clone(),
                    ret_type: ret_type.clone(),
                    body: box body.clone(),
                    pos: pos.clone(),
                }),
                box f(e),
                pos,
            )
        },
        var,
    )
}

fn lift_impl(e: Expr) -> (Box<dyn Fn(Expr) -> Expr>, Expr) {
    match e {
        Expr::LetRec(
            func_name,
            _,
            box Expr::Const(Literal::Func {
                param_name,
                param_type,
                ret_type,
                box body,
                pos: _,
            }),
            box e2,
            pos,
        ) => {
            let (f1, _) = lift_func_case(func_name, param_name, param_type, ret_type, body, pos);
            let (f2, e2) = lift_impl(e2);
            (box move |e: Expr| f1(f2(e)), e2)
        }
        Expr::Const(Literal::Func {
            param_name,
            param_type,
            ret_type,
            box body,
            pos,
        }) => {
            let func_name = Ident::fresh();
            lift_func_case(func_name, param_name, param_type, ret_type, body, pos)
        }
        Expr::Const(Literal::Record(fields)) => {
            let init_f: Box<dyn Fn(Expr) -> Expr> = box |e: Expr| e;
            let (f, fields) = fields.into_iter().fold(
                (init_f, vec![]),
                |(acc_f, mut acc_fields), (label, expr)| {
                    let (f, expr) = lift_impl(expr);
                    acc_fields.push((label, expr));
                    (box move |e: Expr| acc_f(f(e)), acc_fields)
                },
            );
            (f, Expr::Const(Literal::Record(fields)))
        }
        Expr::Const(Literal::Number(_))
        | Expr::Const(Literal::Bool(_))
        | Expr::Const(Literal::Char(_))
        | Expr::Const(Literal::Unit)
        | Expr::Var(_, _, _) => (box |e: Expr| e, e),
        Expr::Apply(box e1, box e2, pos) => {
            let (f1, e1) = lift_impl(e1);
            let (f2, e2) = lift_impl(e2);
            (
                box move |e: Expr| f1(f2(e)),
                Expr::Apply(box e1, box e2, pos),
            )
        }
        Expr::Let(name, typ, box e1, box e2, pos) => {
            let (f1, e1) = lift_impl(e1);
            let (f2, e2) = lift_impl(e2);
            (
                box move |e: Expr| f1(f2(e)),
                Expr::Let(name, typ, box e1, box e2, pos),
            )
        }
        Expr::LetRec(_, _, _, _, _) | Expr::LetType(_, _, _) => unreachable!(),
        Expr::If(box cond, box e1, box e2, pos) => {
            let (f_cond, cond) = lift_impl(cond);
            let (f1, e1) = lift_impl(e1);
            let (f2, e2) = lift_impl(e2);
            (
                box move |e: Expr| f_cond(f1(f2(e))),
                Expr::If(box cond, box e1, box e2, pos),
            )
        }
        Expr::BinOp(op, box e1, box e2, pos) => {
            let (f1, e1) = lift_impl(e1);
            let (f2, e2) = lift_impl(e2);
            (
                box move |e: Expr| f1(f2(e)),
                Expr::BinOp(op, box e1, box e2, pos),
            )
        }
        Expr::FieldAccess(box e, typ, label, pos) => {
            let (f, e) = lift_impl(e);
            (
                box move |e: Expr| f(e),
                Expr::FieldAccess(box e, typ, label, pos),
            )
        }
        Expr::Println(box e) => {
            let (f, e) = lift_impl(e);
            (box move |e: Expr| f(e), Expr::Println(box e))
        }
        Expr::EmptyMark => unreachable!(),
    }
}
