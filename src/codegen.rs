use ast::*;
use ident::Ident;

fn conv_expr(e: Expr) -> nf::Nf {
    match e {
        Expr::Const(lit) => conv_lit(lit),
        Expr::Var(name, _) => nf::Nf {
            funcs: vec![],
            body: nf::Expr::Load(box nf::Expr::Var(name.to_nf_ident())),
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
        Expr::LetRec(name, typ, box e1, box e2, _) => todo!(),
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
        Expr::FieldAccess(_, _, _) => todo!(),
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
        Literal::Record(_) => todo!(),
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
        Type::Record(fields) => nf::Type::Struct(
            fields
                .into_iter()
                .map(|(label, typ)| (label.to_nf_ident(), conv_ty(typ)))
                .collect(),
        ),
        Type::Var(_) => unreachable!(),
    }
}

pub fn codegen(e: Expr, filename: &str) {
    let nf = conv_expr(e);
    println!("{:?}", nf);

    use std::fs;
    let mut f = fs::File::create(filename).unwrap_or_else(|_| panic!("failed: open {}.", filename));
    nf.codegen(filename, &mut f).unwrap();
}
