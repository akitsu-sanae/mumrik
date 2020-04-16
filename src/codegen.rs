use ast::*;
use ident::Ident;

fn conv_expr(e: Expr) -> nf::Nf {
    match e {
        Expr::Const(lit) => conv_lit(lit),
        Expr::Var(name, typ, _) => match typ {
            Type::Func(_, _) | Type::Record(_) => nf::Nf {
                funcs: vec![],
                body: nf::Expr::Var(name.to_nf_ident()),
            },
            _ => nf::Nf {
                funcs: vec![],
                body: nf::Expr::Load(box nf::Expr::Var(name.to_nf_ident())),
            },
        },
        Expr::Apply(box e1, box e2, _) => {
            let mut funcs = vec![];
            let mut nf1 = conv_expr(e1);
            let mut nf2 = conv_expr(e2);
            funcs.append(&mut nf1.funcs);
            funcs.append(&mut nf2.funcs);
            nf::Nf {
                funcs: funcs,
                body: nf::Expr::Call(box nf1.body, vec![nf2.body]),
            }
        }
        Expr::Let(name, typ, box e1, box e2, _) => {
            let mut funcs = vec![];
            let mut nf1 = conv_expr(e1);
            let mut nf2 = conv_expr(e2);
            funcs.append(&mut nf1.funcs);
            funcs.append(&mut nf2.funcs);
            nf::Nf {
                funcs: funcs,
                body: nf::Expr::Let(name.to_nf_ident(), conv_ty(typ), box nf1.body, box nf2.body),
            }
        }
        Expr::LetRec(name, typ, box e1, box e2, _) => {
            let free_vars: Vec<_> = e1
                .free_term_vars()
                .into_iter()
                .filter(|(ref name_, _)| name_ != &name)
                .collect();
            let nf_name = name.to_nf_ident();
            let call_expr = nf::Expr::Call(
                box nf::Expr::Var(nf_name.clone()),
                free_vars
                    .iter()
                    .map(|(name, _)| nf::Expr::Var(name.clone().to_nf_ident()))
                    .collect(),
            );

            let nf1 = conv_expr(e1);
            let nf2 = conv_expr(e2);
            let mut nf1_funcs = nf1
                .funcs
                .into_iter()
                .map(|func| func.subst_expr(&nf_name, &call_expr))
                .collect();
            let nf1_body = nf1.body.subst_expr(&nf_name, &call_expr);
            let mut nf2_funcs = nf2
                .funcs
                .into_iter()
                .map(|func| func.subst_expr(&nf_name, &call_expr))
                .collect();
            let nf2_body = nf2.body.subst_expr(&nf_name, &call_expr);

            let mut funcs = vec![];
            funcs.append(&mut nf1_funcs);
            funcs.append(&mut nf2_funcs);
            funcs.push(nf::Func {
                name: nf_name,
                params: free_vars
                    .into_iter()
                    .map(|(name, typ)| (name.to_nf_ident(), conv_ty(typ)))
                    .collect(),
                ret_type: nf::Type::Pointer(box conv_ty(typ)),
                body: nf1_body,
            });

            nf::Nf {
                funcs: funcs,
                body: nf2_body,
            }
        }
        Expr::LetType(_, _, _) => unreachable!(),
        Expr::If(box cond, box e1, box e2, _) => {
            let nf_cond = conv_expr(cond);
            let mut nf1 = conv_expr(e1);
            let mut nf2 = conv_expr(e2);
            let mut funcs = nf_cond.funcs;
            funcs.append(&mut nf1.funcs);
            funcs.append(&mut nf2.funcs);
            nf::Nf {
                funcs: funcs,
                body: nf::Expr::If(box nf_cond.body, box nf1.body, box nf2.body),
            }
        }
        Expr::BinOp(op, box e1, box e2, _) => {
            let mut nf1 = conv_expr(e1);
            let mut nf2 = conv_expr(e2);
            let mut funcs = vec![];
            funcs.append(&mut nf1.funcs);
            funcs.append(&mut nf2.funcs);
            nf::Nf {
                funcs: funcs,
                body: nf::Expr::BinOp(conv_binop(op), box nf1.body, box nf2.body),
            }
        }
        Expr::FieldAccess(box e, typ, label, _) => {
            if let Type::Record(fields) = typ {
                let nf = conv_expr(e);
                let idx = fields
                    .iter()
                    .position(|(ref label_, _)| &label == label_)
                    .unwrap();
                nf::Nf {
                    funcs: nf.funcs,
                    body: nf::Expr::Load(box nf::Expr::TupleAt(box nf.body, idx)),
                }
            } else {
                unreachable!()
            }
        }
        Expr::Println(box e) => {
            let nf = conv_expr(e);
            nf::Nf {
                funcs: nf.funcs,
                body: nf::Expr::PrintNum(box nf.body),
            }
        }
    }
}

fn conv_lit(lit: Literal) -> nf::Nf {
    match lit {
        Literal::Func {
            param_name,
            param_type,
            ret_type,
            box body,
            pos: _,
        } => {
            let nf_body = conv_expr(body);
            let func_name = Ident::fresh();
            let mut funcs = nf_body.funcs;
            funcs.push(nf::Func {
                name: func_name.clone().to_nf_ident(),
                params: vec![(param_name.to_nf_ident(), conv_ty(param_type))],
                ret_type: conv_ty(ret_type),
                body: nf_body.body,
            });
            nf::Nf {
                funcs: funcs,
                body: nf::Expr::Var(func_name.to_nf_ident()),
            }
        }
        Literal::Number(n) => nf::Nf {
            funcs: vec![],
            body: nf::Expr::Const(nf::Literal::Int(n)),
        },
        Literal::Bool(b) => nf::Nf {
            funcs: vec![],
            body: nf::Expr::Const(nf::Literal::Bool(b)),
        },
        Literal::Char(c) => nf::Nf {
            funcs: vec![],
            body: nf::Expr::Const(nf::Literal::Char(c)),
        },
        Literal::Unit => nf::Nf {
            funcs: vec![],
            body: nf::Expr::Const(nf::Literal::Int(0)), // dummy,
        },
        Literal::Record(fields) => {
            let mut funcs = vec![];
            let elems = fields
                .into_iter()
                .map(|(_, e)| {
                    let mut nf = conv_expr(e);
                    funcs.append(&mut nf.funcs);
                    nf.body
                })
                .collect();
            nf::Nf {
                funcs: funcs,
                body: nf::Expr::Const(nf::Literal::Tuple(elems)),
            }
        }
    }
}

fn conv_binop(op: BinOp) -> nf::BinOp {
    match op {
        BinOp::Add => nf::BinOp::Add,
        BinOp::Sub => nf::BinOp::Sub,
        BinOp::Mult => nf::BinOp::Mult,
        BinOp::Div => nf::BinOp::Div,
        BinOp::Eq => nf::BinOp::Eq,
        BinOp::Neq => nf::BinOp::Neq,
        BinOp::Lt => nf::BinOp::Lt,
        BinOp::Gt => nf::BinOp::Gt,
    }
}

fn conv_ty(ty: Type) -> nf::Type {
    match ty {
        Type::Int => nf::Type::Int,
        Type::Bool => nf::Type::Bool,
        Type::Char => nf::Type::Char,
        Type::Unit => nf::Type::Int, // dummy
        Type::Func(box ty1, box ty2) => nf::Type::Func(vec![conv_ty(ty1)], box conv_ty(ty2)),
        Type::Record(fields) => {
            nf::Type::Tuple(fields.into_iter().map(|(_, typ)| conv_ty(typ)).collect())
        }
        Type::Var(_) => unreachable!(),
    }
}

pub fn codegen(e: Expr, filename: &str) -> Result<(), nf::error::Error> {
    let nf = conv_expr(e);

    use std::fs;
    let mut f = fs::File::create(filename).unwrap_or_else(|_| panic!("failed: open {}.", filename));
    nf.codegen(filename, &mut f)?;
    Ok(())
}
