use ast::*;

mod lambda_lifting;

fn conv_toplevel_expr(e: Expr) -> nf::Nf {
    match e {
        Expr::Let(
            func_name,
            _,
            box Expr::Const(Literal::Func {
                param_name,
                param_type,
                ret_type,
                box body,
                pos: _,
            }),
            box left,
            _,
        )
        | Expr::LetRec(
            func_name,
            _,
            box Expr::Const(Literal::Func {
                param_name,
                param_type,
                ret_type,
                box body,
                pos: _,
            }),
            box left,
            _,
        ) => {
            let mut nf_left = conv_toplevel_expr(left);
            let mut nf_body = conv_toplevel_expr(body);
            nf_left.funcs.append(&mut nf_body.funcs);
            let body = if param_name.is_omitted_param_name() {
                if let Type::Record(fields) = param_type.clone() {
                    let param_name = param_name.clone().to_nf_ident();
                    fields.into_iter().enumerate().fold(
                        nf_body.body.unwrap(),
                        |acc, (n, (name, typ))| {
                            nf::Expr::Let(
                                name.to_nf_ident(),
                                conv_ty(typ),
                                box nf::Expr::Load(box nf::Expr::TupleAt(
                                    box nf::Expr::Var(param_name.clone()),
                                    n,
                                )),
                                box acc,
                            )
                        },
                    )
                } else {
                    unreachable!()
                }
            } else {
                nf_body.body.unwrap()
            };
            nf_left.funcs.push(nf::Func {
                name: func_name.clone().to_nf_ident(),
                params: vec![(param_name.to_nf_ident(), conv_ty(param_type))],
                ret_type: conv_ty(ret_type),
                body: body,
            });
            nf_left
        }
        _ => nf::Nf {
            funcs: vec![],
            body: Some(conv_expr(e)),
        },
    }
}

fn conv_expr(e: Expr) -> nf::Expr {
    match e {
        Expr::Const(lit) => nf::Expr::Const(conv_lit(lit)),
        Expr::Var(name, typ, _) => match typ {
            Type::Func(_, _) => nf::Expr::Var(name.to_nf_ident()),
            _ => nf::Expr::Load(box nf::Expr::Var(name.to_nf_ident())),
        },
        Expr::Apply(box e1, box e2, _) => nf::Expr::Call(box conv_expr(e1), vec![conv_expr(e2)]),
        Expr::Let(name, typ, box e1, box e2, _) => nf::Expr::Let(
            name.to_nf_ident(),
            if let Type::Func(_, _) = typ {
                nf::Type::Pointer(box conv_ty(typ))
            } else {
                conv_ty(typ)
            },
            box conv_expr(e1),
            box conv_expr(e2),
        ),
        Expr::LetRec(_, _, _, _, _) => unreachable!(), // `Expr::LetRec` occurs only with function
        Expr::LetType(_, _, _) => unreachable!(),
        Expr::If(box cond, box e1, box e2, _) => {
            nf::Expr::If(box conv_expr(cond), box conv_expr(e1), box conv_expr(e2))
        }
        Expr::BinOp(op, box e1, box e2, _) => {
            nf::Expr::BinOp(conv_binop(op), box conv_expr(e1), box conv_expr(e2))
        }
        Expr::FieldAccess(box e, typ, label, _) => {
            if let Type::Record(fields) = typ {
                let idx = fields
                    .iter()
                    .position(|(ref label_, _)| &label == label_)
                    .unwrap();
                let body = if let nf::Expr::Load(box body) = conv_expr(e) {
                    body
                } else {
                    unreachable!()
                };
                nf::Expr::Load(box nf::Expr::TupleAt(box body, idx))
            } else {
                unreachable!()
            }
        }
        Expr::Println(box e) => nf::Expr::PrintNum(box conv_expr(e)),
        Expr::EmptyMark => unreachable!(),
    }
}

fn conv_lit(lit: Literal) -> nf::Literal {
    match lit {
        Literal::Func {
            param_name: _,
            param_type: _,
            ret_type: _,
            body: _,
            pos: _,
        } => unreachable!(),
        Literal::Number(n) => nf::Literal::Int(n),
        Literal::Bool(b) => nf::Literal::Bool(b),
        Literal::Char(c) => nf::Literal::Char(c),
        Literal::Unit => nf::Literal::Int(0), // dummy,
        Literal::Record(fields) => {
            let elems = fields.into_iter().map(|(_, e)| conv_expr(e)).collect();
            nf::Literal::Tuple(elems)
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
        Type::EmptyMark => unreachable!(),
    }
}

pub fn codegen(expr: Expr, filename: &str) {
    let mut temp = tempfile::Builder::new()
        .suffix(".ll")
        .tempfile()
        .expect("failed: create temporary file.");
    let nf = conv_toplevel_expr(lambda_lifting::lift(expr));
    if let Err(err) = nf.codegen("output", &mut temp) {
        eprintln!("\u{001B}[31m[internal codegen error]\u{001B}[39m {}", err);
        eprintln!(
            "please report this issue to akitsu-sanae <akitsu.sanae@gmail.com>, the developer of mumrik language"
        );
        std::process::exit(-1);
    }

    let temp_filename = temp.path().to_str().unwrap().to_string();

    let result = std::process::Command::new("clang")
        .arg(temp_filename)
        .arg("-o")
        .arg(filename)
        .output()
        .expect("failed to execute clang");

    if !result.status.success() {
        eprintln!(
            "\u{001B}[31m[internal codegen error]\u{001B}[39m clang didn't terminate successfully"
        );
        let stdout = std::str::from_utf8(&result.stdout)
            .expect("unrecognized output")
            .trim();
        if !stdout.is_empty() {
            eprintln!("stdout from `clang`:");
            eprintln!("```");
            eprintln!("{}", stdout);
            eprintln!("```");
        }
        let stderr = std::str::from_utf8(&result.stderr)
            .expect("unrecognized output")
            .trim();
        if !stderr.is_empty() {
            eprintln!("stderr from `clang`:");
            eprintln!("```");
            eprintln!("{}", stderr);
            eprintln!("```");
        }
        eprintln!(
            "please report this issue to akitsu-sanae <akitsu.sanae@gmail.com>, the developer of mumrik language"
        );
        std::process::exit(-1);
    }
}
