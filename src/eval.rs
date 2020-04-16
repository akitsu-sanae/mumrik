use ast::*;

pub fn expr(e: Expr) -> Expr {
    match e {
        Expr::Const(lit) => Expr::Const(literal(lit)),
        Expr::Var(_, _, _) => e,
        Expr::Apply(box e1, box e2, _) => {
            let f = expr(e1);
            if let Expr::Const(Literal::Func {
                param_name,
                param_type: _,
                ret_type: _,
                box body,
                pos: _,
            }) = f
            {
                let e2 = expr(e2);
                expr(body.subst_expr(&param_name, &e2))
            } else {
                unreachable!()
            }
        }
        Expr::Let(name, _, box e1, box e2, _) => {
            let e1 = expr(e1);
            expr(e2.subst_expr(&name, &e1))
        }
        Expr::LetRec(name, typ, box e1, box e2, pos) => {
            let e1 = e1.clone().subst_expr(
                &name,
                &Expr::LetRec(
                    name.clone(),
                    typ.clone(),
                    box e1,
                    box Expr::Var(name.clone(), typ, Position { start: 0, end: 0 }),
                    pos,
                ),
            );
            expr(e2.subst_expr(&name, &e1))
        }
        Expr::LetType(_, _, box e) => expr(e),
        Expr::If(box cond, box e1, box e2, _) => match expr(cond) {
            Expr::Const(Literal::Bool(b)) => {
                if b {
                    expr(e1)
                } else {
                    expr(e2)
                }
            }
            _ => unreachable!(),
        },
        Expr::BinOp(op, box e1, box e2, _) => {
            let (lit1, lit2) = if let (Expr::Const(lit1), Expr::Const(lit2)) = (expr(e1), expr(e2))
            {
                (lit1, lit2)
            } else {
                unreachable!()
            };
            Expr::Const(match (op, lit1, lit2) {
                (BinOp::Add, Literal::Number(n1), Literal::Number(n2)) => Literal::Number(n1 + n2),
                (BinOp::Sub, Literal::Number(n1), Literal::Number(n2)) => Literal::Number(n1 - n2),
                (BinOp::Mult, Literal::Number(n1), Literal::Number(n2)) => Literal::Number(n1 * n2),
                (BinOp::Div, Literal::Number(n1), Literal::Number(n2)) => Literal::Number(n1 / n2),

                (BinOp::Eq, lit1, lit2) => Literal::Bool(lit1 == lit2),
                (BinOp::Neq, lit1, lit2) => Literal::Bool(lit1 != lit2),

                (BinOp::Lt, Literal::Number(n1), Literal::Number(n2)) => Literal::Bool(n1 < n2),
                (BinOp::Gt, Literal::Number(n1), Literal::Number(n2)) => Literal::Bool(n1 > n2),
                _ => unreachable!(),
            })
        }
        Expr::FieldAccess(box e, _, label, _) => {
            if let Expr::Const(Literal::Record(fields)) = expr(e) {
                expr(
                    fields
                        .into_iter()
                        .find(|(ref label_, _)| &label == label_)
                        .unwrap()
                        .1,
                )
            } else {
                unreachable!()
            }
        }
        Expr::Println(box e) => {
            println!("{}", expr(e));
            Expr::Const(Literal::Unit)
        }
    }
}

fn literal(lit: Literal) -> Literal {
    match lit {
        Literal::Func {
            param_name,
            param_type,
            ret_type,
            body,
            pos,
        } => Literal::Func {
            param_name,
            param_type,
            ret_type,
            body,
            pos,
        },
        Literal::Number(n) => Literal::Number(n),
        Literal::Bool(b) => Literal::Bool(b),
        Literal::Char(c) => Literal::Char(c),
        Literal::Unit => Literal::Unit,
        Literal::Record(fields) => Literal::Record(
            fields
                .into_iter()
                .map(|(label, e)| (label, expr(e)))
                .collect(),
        ),
    }
}
