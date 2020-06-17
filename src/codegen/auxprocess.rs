use ast::*;
use ident::Ident;
use std::collections::HashMap;

type Params = (Ident, HashMap<Ident, Type>);

pub fn pre(e: Expr) -> Expr {
    let (f, e, appended_params) = lift_impl(e, &vec![]);
    let e = f(e);
    let func_types = gather_func_types(&e);
    fix_param_type_toplevel(e, &func_types, &appended_params)
}

fn lift_impl(
    e: Expr,
    func_names: &Vec<Ident>,
) -> (Box<dyn Fn(Expr) -> Expr>, Expr, HashMap<Ident, Params>) {
    match e {
        Expr::Func { .. } => {
            let mut free_vars: HashMap<Ident, Type> = e
                .free_term_vars()
                .into_iter()
                .filter(|(name, _)| !func_names.clone().contains(name))
                .collect();

            let (func_name, param_name, param_type, ret_type, body, left, pos) =
                if let Expr::Func {
                    name,
                    param_name,
                    param_type,
                    ret_type,
                    box body,
                    box left,
                    pos,
                } = e
                {
                    (name, param_name, param_type, ret_type, body, left, pos)
                } else {
                    unreachable!()
                };

            let mut func_names = func_names.clone();
            func_names.push(func_name.clone());

            let mut appended_params = HashMap::new();
            appended_params.insert(func_name.clone(), (param_name.clone(), free_vars.clone()));

            let (body_f, body, appended_params_body) = lift_impl(body, &func_names);
            let (left_f, left, appended_params_left) = lift_impl(left, &func_names);
            appended_params.extend(appended_params_body);
            appended_params.extend(appended_params_left);

            let (param_name, param_type) = if param_name.is_omitted_param_name() {
                if let Type::Record(fields) = param_type {
                    free_vars.extend(fields);
                    (Ident::omitted_param_name(), Type::Record(free_vars))
                } else {
                    unreachable!()
                }
            } else if free_vars.is_empty() {
                (param_name, param_type)
            } else {
                free_vars.insert(param_name, param_type);
                (Ident::omitted_param_name(), Type::Record(free_vars))
            };
            (
                box move |e| Expr::Func {
                    name: func_name.clone(),
                    param_name: param_name.clone(),
                    param_type: param_type.clone(),
                    ret_type: ret_type.clone(),
                    body: box body.clone(),
                    left: box left_f(body_f(e)),
                    pos: pos,
                },
                left,
                appended_params,
            )
        }
        Expr::Const(Literal::Record(fields)) => {
            let init_f: Box<dyn Fn(Expr) -> Expr> = box |e: Expr| e;
            let (f, fields, appended_params) = fields.into_iter().fold(
                (init_f, HashMap::new(), HashMap::new()),
                |(acc_f, mut acc_fields, mut acc_appended_params), (label, expr)| {
                    let (f, expr, appended_params) = lift_impl(expr, func_names);
                    acc_fields.insert(label, expr);
                    acc_appended_params.extend(appended_params);
                    (
                        box move |e: Expr| acc_f(f(e)),
                        acc_fields,
                        acc_appended_params,
                    )
                },
            );
            (f, Expr::Const(Literal::Record(fields)), appended_params)
        }
        Expr::Const(Literal::Number(_))
        | Expr::Const(Literal::Bool(_))
        | Expr::Const(Literal::Char(_))
        | Expr::Const(Literal::Unit)
        | Expr::Var(_, _, _) => (box |e: Expr| e, e, HashMap::new()),
        Expr::Apply(box e1, box e2, pos) => {
            let (f1, e1, appended_params1) = lift_impl(e1, func_names);
            let (f2, e2, appended_params2) = lift_impl(e2, func_names);
            let mut appended_params = HashMap::new();
            appended_params.extend(appended_params1);
            appended_params.extend(appended_params2);
            (
                box move |e: Expr| f1(f2(e)),
                Expr::Apply(box e1, box e2, pos),
                appended_params,
            )
        }
        Expr::Let(name, typ, box e1, box e2, pos) => {
            let (f1, e1, appended_params1) = lift_impl(e1, func_names);
            let (f2, e2, appended_params2) = lift_impl(e2, func_names);
            let mut appended_params = HashMap::new();
            appended_params.extend(appended_params1);
            appended_params.extend(appended_params2);
            (
                box move |e: Expr| f1(f2(e)),
                Expr::Let(name, typ, box e1, box e2, pos),
                appended_params,
            )
        }
        Expr::LetType(_, _, _) => unreachable!(),
        Expr::If(box cond, box e1, box e2, pos) => {
            let (f_cond, cond, appended_params_cond) = lift_impl(cond, func_names);
            let (f1, e1, appended_params1) = lift_impl(e1, func_names);
            let (f2, e2, appended_params2) = lift_impl(e2, func_names);
            let mut appended_params = HashMap::new();
            appended_params.extend(appended_params_cond);
            appended_params.extend(appended_params1);
            appended_params.extend(appended_params2);
            (
                box move |e: Expr| f_cond(f1(f2(e))),
                Expr::If(box cond, box e1, box e2, pos),
                appended_params,
            )
        }
        Expr::BinOp(op, box e1, box e2, pos) => {
            let (f1, e1, appended_params1) = lift_impl(e1, func_names);
            let (f2, e2, appended_params2) = lift_impl(e2, func_names);
            let mut appended_params = HashMap::new();
            appended_params.extend(appended_params1);
            appended_params.extend(appended_params2);
            (
                box move |e: Expr| f1(f2(e)),
                Expr::BinOp(op, box e1, box e2, pos),
                appended_params,
            )
        }
        Expr::FieldAccess(box e, typ, label, pos) => {
            let (f, e, appended_params) = lift_impl(e, func_names);
            (
                box move |e: Expr| f(e),
                Expr::FieldAccess(box e, typ, label, pos),
                appended_params,
            )
        }
        Expr::Println(box e) => {
            let (f, e, appended_params) = lift_impl(e, func_names);
            (
                box move |e: Expr| f(e),
                Expr::Println(box e),
                appended_params,
            )
        }
        Expr::EmptyMark => unreachable!(),
    }
}

fn gather_func_types(e: &Expr) -> HashMap<Ident, Type> {
    if let Expr::Func {
        ref name,
        ref param_type,
        ref ret_type,
        box ref left,
        ..
    } = e
    {
        let mut func_types = gather_func_types(left);
        func_types.insert(
            name.clone(),
            Type::Func(box param_type.clone(), box ret_type.clone()),
        );
        func_types
    } else {
        HashMap::new()
    }
}

fn fix_param_type_toplevel(
    e: Expr,
    func_types: &HashMap<Ident, Type>,
    appended_params: &HashMap<Ident, Params>,
) -> Expr {
    if let Expr::Func {
        name,
        param_name,
        param_type,
        ret_type,
        box body,
        box left,
        pos,
    } = e
    {
        let body = fix_param_type_inner(body, func_types, appended_params);
        let left = fix_param_type_toplevel(left, func_types, appended_params);
        Expr::Func {
            name: name,
            param_name: param_name,
            param_type: param_type,
            ret_type: ret_type,
            body: box body,
            left: box left,
            pos: pos,
        }
    } else {
        fix_param_type_inner(e, func_types, appended_params)
    }
}

fn fix_param_type_inner(
    e: Expr,
    func_types: &HashMap<Ident, Type>,
    appended_params: &HashMap<Ident, Params>,
) -> Expr {
    match e {
        Expr::Const(lit) => Expr::Const(fix_param_type_literal(lit, func_types, appended_params)),
        Expr::Var(_, _, _) => e,
        Expr::Func { .. } | Expr::LetType(_, _, _) => unreachable!(),
        Expr::Apply(box Expr::Var(func_name, _, f_pos), box arg, app_pos) => {
            let func_type = func_types.get(&func_name).unwrap().clone();
            let appended_param = appended_params.get(&func_name).unwrap().clone();
            fix_params_apply_case(func_name, func_type, appended_param, f_pos, arg, app_pos)
        }
        Expr::Apply(_, _, _) => unreachable!(),
        Expr::Let(
            name,
            _,
            box Expr::Var(rhs_name, rhs_typ @ Type::Func { .. }, pos),
            box e2,
            _,
        ) => e2.subst_expr(&name, &Expr::Var(rhs_name, rhs_typ, pos)),
        Expr::Let(name, typ, box e1, box e2, pos) => Expr::Let(
            name,
            typ,
            box fix_param_type_inner(e1, func_types, appended_params),
            box fix_param_type_inner(e2, func_types, appended_params),
            pos,
        ),
        Expr::If(box cond, box e1, box e2, pos) => Expr::If(
            box fix_param_type_inner(cond, func_types, appended_params),
            box fix_param_type_inner(e1, func_types, appended_params),
            box fix_param_type_inner(e2, func_types, appended_params),
            pos,
        ),
        Expr::BinOp(op, box e1, box e2, pos) => Expr::BinOp(
            op,
            box fix_param_type_inner(e1, func_types, appended_params),
            box fix_param_type_inner(e2, func_types, appended_params),
            pos,
        ),
        Expr::FieldAccess(box e, typ, label, pos) => Expr::FieldAccess(
            box fix_param_type_inner(e, func_types, appended_params),
            typ,
            label,
            pos,
        ),
        Expr::Println(box e) => {
            Expr::Println(box fix_param_type_inner(e, func_types, appended_params))
        }
        Expr::EmptyMark => unreachable!(),
    }
}

fn fix_param_type_literal(
    lit: Literal,
    func_types: &HashMap<Ident, Type>,
    appended_params: &HashMap<Ident, Params>,
) -> Literal {
    match lit {
        Literal::Record(fields) => Literal::Record(
            fields
                .into_iter()
                .map(|(label, e)| (label, fix_param_type_inner(e, func_types, appended_params)))
                .collect(),
        ),
        _ => lit,
    }
}

// func f _:{y: int, x_: int} :int => x_ + y
//
// f: {int, int} -> int
// (f as int -> int) 3
fn fix_params_apply_case(
    func_name: Ident,
    func_type: Type,
    (param_name, free_vars): Params,
    f_pos: Position,
    arg: Expr,
    app_pos: Position,
) -> Expr {
    let f = Expr::Var(func_name, func_type, f_pos);
    let arg = if free_vars.is_empty() {
        arg
    } else {
        let arg_fields = if param_name.is_omitted_param_name() {
            if let Expr::Const(Literal::Record(fields)) = arg {
                fields
            } else {
                unreachable!()
            }
        } else {
            let mut arg_fields = HashMap::new();
            arg_fields.insert(param_name, arg);
            arg_fields
        };
        Expr::Const(Literal::Record(free_vars.into_iter().fold(
            arg_fields,
            |mut acc, (name, typ)| {
                let e = Expr::Var(name.clone(), typ, Position::dummy());
                acc.insert(name, e);
                acc
            },
        )))
    };
    Expr::Apply(box f, box arg, app_pos)
}
