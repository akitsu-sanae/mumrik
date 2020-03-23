use expr;

fn subst_expr_lit(lit: nf::Literal, name: &str, e: &nf::Expr) -> nf::Literal {
    match lit {
        nf::Literal::Bool(_) | nf::Literal::Char(_) | nf::Literal::Int(_) => lit,
        nf::Literal::Array(es, box ty) => nf::Literal::Array(
            es.into_iter().map(|e_| subst_expr(e_, name, e)).collect(),
            box ty,
        ),
        nf::Literal::Struct(fields) => nf::Literal::Struct(
            fields
                .into_iter()
                .map(|(label, e_)| (label, subst_expr(e_, name, e)))
                .collect(),
        ),
    }
}

fn subst_expr(e_: nf::Expr, name: &str, e: &nf::Expr) -> nf::Expr {
    match e_ {
        nf::Expr::Const(lit) => nf::Expr::Const(subst_expr_lit(lit, name, e)),
        nf::Expr::Var(name_) if name_ == nf::Ident::new(name) => e.clone(),
        nf::Expr::Var(_) => e_,
        nf::Expr::Call(box f, args) => nf::Expr::Call(
            box subst_expr(f, name, e),
            args.into_iter()
                .map(|arg| subst_expr(arg, name, e))
                .collect(),
        ),
        nf::Expr::If(box cond, box e1, box e2) => nf::Expr::If(
            box subst_expr(cond, name, e),
            box subst_expr(e1, name, e),
            box subst_expr(e2, name, e),
        ),
        nf::Expr::BinOp(op, box e1, box e2) => {
            nf::Expr::BinOp(op, box subst_expr(e1, name, e), box subst_expr(e2, name, e))
        }
        _ => unimplemented!(),
    }
}

fn to_nf(e: expr::Expr) -> nf::Nf {
    match e {
        expr::Expr::Const(lit) => nf::Nf {
            funcs: vec![],
            body: nf::Expr::Const(to_nf_literal(lit)),
        },
        expr::Expr::Var(name) => nf::Nf {
            funcs: vec![],
            body: nf::Expr::Var(nf::Ident::new(&name)),
        },
        expr::Expr::Lambda(_, _, _) => unimplemented!(),
        expr::Expr::Apply(box e1, box e2) => {
            let mut nf1 = to_nf(e1);
            let mut nf2 = to_nf(e2);
            nf1.funcs.append(&mut nf2.funcs);
            nf::Nf {
                funcs: nf1.funcs,
                body: nf::Expr::Call(box nf1.body, vec![nf2.body]),
            }
        }
        expr::Expr::Let(name, box e1, box e2) => {
            let free_vars: Vec<_> = e1
                .free_vars()
                .into_iter()
                .map(|var| (nf::Ident::new(&var), nf::Type::Int) /* TODO */)
                .collect();
            let mut funcs = vec![];
            let mut nf1 = to_nf(e1);
            let mut nf2 = to_nf(e2);
            funcs.append(&mut nf1.funcs);
            funcs.append(&mut nf2.funcs);
            funcs.push(nf::Func {
                name: nf::Ident::new(&name),
                params: free_vars.clone(),
                ret_type: nf::Type::Int, // TODO
                body: nf1.body,
            });
            let e = subst_expr(
                nf2.body,
                &name,
                &nf::Expr::Call(
                    box nf::Expr::Var(nf::Ident::new(&name)),
                    free_vars.into_iter().map(|v| nf::Expr::Var(v.0)).collect(),
                ),
            );
            nf::Nf {
                funcs: funcs,
                body: e,
            }
        }
        expr::Expr::LetRec(_, _, _, _) => unimplemented!(),
        expr::Expr::LetType(_, _, _) => unimplemented!(),
        expr::Expr::If(box cond, box e1, box e2) => {
            let nf_cond = to_nf(cond);
            let mut nf1 = to_nf(e1);
            let mut nf2 = to_nf(e2);
            let mut funcs = nf_cond.funcs;
            funcs.append(&mut nf1.funcs);
            funcs.append(&mut nf2.funcs);
            nf::Nf {
                funcs: funcs,
                body: nf::Expr::If(box nf_cond.body, box nf1.body, box nf2.body),
            }
        }
        expr::Expr::BinOp(op, box e1, box e2) => {
            let nf1 = to_nf(e1);
            let mut nf2 = to_nf(e2);
            let mut funcs = nf1.funcs;
            funcs.append(&mut nf2.funcs);
            nf::Nf {
                funcs: funcs,
                body: nf::Expr::BinOp(to_nf_binop(op), box nf1.body, box nf2.body),
            }
        }
        _ => unimplemented!(),
    }
}

fn to_nf_literal(l: expr::Literal) -> nf::Literal {
    use expr::Literal::*;
    match l {
        Number(n) => nf::Literal::Int(n),
        Bool(b) => nf::Literal::Bool(b),
        Char(c) => nf::Literal::Char(c),
        Unit => nf::Literal::Int(0), // dummy
        List(_) => unimplemented!(),
        Variant(_, _, _) => unimplemented!(),
        Record(_) => unimplemented!(),
    }
}

fn to_nf_binop(op: expr::BinOp) -> nf::BinOp {
    use expr::BinOp::*;
    use nf::BinOp;
    match op {
        Add => BinOp::Add,
        Sub => BinOp::Sub,
        Mult => BinOp::Mult,
        Div => BinOp::Div,
        Equal => BinOp::Eq,
        NotEqual => BinOp::Neq,
        LessThan => BinOp::Lt,
        GreaterThan => BinOp::Gt,
    }
}

pub fn codegen(e: expr::Expr, name: &str) {
    let e = e.make_name_unique();
    let nf = to_nf(e);

    // write llvm-ir
    use std::fs;
    let mut f = fs::File::create(name).unwrap();
    match nf.codegen(name, &mut f) {
        Ok(()) => (),
        Err(err) => panic!("{}", err),
    }
}
