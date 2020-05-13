use ast::*;

mod auxprocess;

fn conv_toplevel_expr(e: Expr) -> nf::Nf {
    match e {
        Expr::Func {
            name: func_name,
            param_name,
            param_type,
            ret_type,
            box body,
            box left,
            pos: _,
        } => {
            let mut nf = conv_toplevel_expr(left);
            let body = conv_expr(body);
            let body = if param_name.is_omitted_param_name() {
                if let Type::Record(fields) = param_type.clone() {
                    let param_name = param_name.clone().to_nf_ident();
                    fields
                        .into_iter()
                        .enumerate()
                        .fold(body, |acc, (n, (name, typ))| {
                            nf::Expr::Let(
                                name.to_nf_ident(),
                                conv_ty(typ),
                                box nf::Expr::Load(box nf::Expr::TupleAt(
                                    box nf::Expr::Var(param_name.clone()),
                                    n,
                                )),
                                box acc,
                            )
                        })
                } else {
                    unreachable!()
                }
            } else {
                body
            };
            nf.funcs.push(nf::Func {
                name: func_name.clone().to_nf_ident(),
                params: vec![(param_name.to_nf_ident(), conv_ty(param_type))],
                ret_type: conv_ty(ret_type),
                body: body,
            });
            nf
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
        Expr::Func { .. } => unreachable!(),
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
                    .position(|(label_, _)| &label == label_)
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
        Type::Var(_) | Type::EmptyMark => unreachable!(),
    }
}

fn exec_command(command_name: &str, args: Vec<&str>) {
    let mut command = std::process::Command::new(command_name);
    command.args(args);

    let result = command
        .output()
        .unwrap_or_else(|_| panic!("failed to execute {}", command_name));

    if !result.status.success() {
        eprintln!(
            "\u{001B}[31m[internal codegen error]\u{001B}[39m in {}",
            command_name
        );
        let stdout = std::str::from_utf8(&result.stdout)
            .expect("unrecognized output")
            .trim();
        if !stdout.is_empty() {
            eprintln!("stdout from `{:?}`:", command);
            eprintln!("```");
            eprintln!("{}", stdout);
            eprintln!("```");
        }
        let stderr = std::str::from_utf8(&result.stderr)
            .expect("unrecognized output")
            .trim();
        if !stderr.is_empty() {
            eprintln!("stderr from `{:?}`:", command);
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

pub fn codegen(expr: Expr, filename: &str) {
    let nf = conv_toplevel_expr(auxprocess::pre(expr));

    let mut ll_file = tempfile::Builder::new()
        .suffix(".ll")
        .tempfile()
        .expect("failed: create temporary file.");

    if let Err(err) = nf.codegen("output", &mut ll_file) {
        eprintln!("\u{001B}[31m[internal codegen error]\u{001B}[39m {}", err);
        eprintln!(
            "please report this issue to akitsu-sanae <akitsu.sanae@gmail.com>, the developer of mumrik language"
        );
        std::process::exit(-1);
    }

    let ll_filename = ll_file.path().to_str().unwrap().to_string();

    let obj_file = tempfile::Builder::new()
        .suffix(".o")
        .tempfile()
        .expect("failed: create temporary file.");

    let obj_filename = obj_file.path().to_str().unwrap();

    exec_command(
        "llc",
        vec!["-filetype=obj", ll_filename.as_str(), "-o", obj_filename],
    );
    exec_command("gcc", vec![obj_filename, "-o", filename]);
}
