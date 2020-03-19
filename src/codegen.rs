use expr;
use std::collections::HashMap;
use type_::Type;

mod normal_form;

fn to_kazuma_params(params: Vec<(String, Type)>) -> Vec<(String, kazuma::typ::Type)> {
    params
        .into_iter()
        .map(|(name, ty)| (name, to_kazuma_type(ty)))
        .collect()
}
fn to_kazuma_type(ty: Type) -> kazuma::typ::Type {
    match ty {
        Type::Int => kazuma::typ::Type::Int,
        Type::Bool => kazuma::typ::Type::Bool,
        Type::Char => kazuma::typ::Type::Char,
        Type::Unit => kazuma::typ::Type::Void,
        Type::Function(box ty1, box ty2) => {
            kazuma::typ::Type::Func(vec![to_kazuma_type(ty1)], box to_kazuma_type(ty2))
        }
        _ => unreachable!(),
    }
}

fn to_kazuma_literal(l: expr::Literal) -> kazuma::program::Literal {
    use expr::Literal::*;
    match l {
        Number(n) => kazuma::program::Literal::Int(n),
        Bool(b) => kazuma::program::Literal::Bool(b),
        Char(c) => kazuma::program::Literal::Char(c),
        Unit => kazuma::program::Literal::Int(0), // dummy
        List(_) => unimplemented!(),
        Variant(_, _, _) => unimplemented!(),
        Record(_) => unimplemented!(),
    }
}

fn to_kazuma_binop(op: expr::BinOp) -> kazuma::program::BinOp {
    use expr::BinOp::*;
    use kazuma::program::BinOp;
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

fn to_kazuma_expr(e: normal_form::Nexpr) -> kazuma::program::Expr {
    use codegen::normal_form::Nexpr::*;
    use kazuma::program::Expr;
    match e {
        Const(lit) => Expr::Literal(to_kazuma_literal(lit)),
        Var(name) => Expr::Var(name),
        Apply(box e1, args) => Expr::Call(
            box to_kazuma_expr(e1),
            args.into_iter().map(to_kazuma_expr).collect(),
        ),
        If(box cond, box e1, box e2) => Expr::If(
            box to_kazuma_expr(cond),
            box to_kazuma_expr(e1),
            box to_kazuma_expr(e2),
        ),
        BinOp(op, box e1, box e2) => Expr::BinOp(
            to_kazuma_binop(op),
            box to_kazuma_expr(e1),
            box to_kazuma_expr(e2),
        ),
    }
}

fn to_kazuma_module(name: &str, nf: normal_form::NormalForm) -> kazuma::program::Module {
    let mut funcs = vec![];
    for func in nf.funcs.into_iter() {
        funcs.push(kazuma::program::Func {
            name: func.name,
            args: to_kazuma_params(func.params),
            ret_type: to_kazuma_type(func.ret_type),
            body: vec![kazuma::program::Statement::Expr(to_kazuma_expr(func.body))],
        });
    }

    funcs.push(kazuma::program::Func {
        name: "main".to_string(),
        args: vec![],
        ret_type: kazuma::typ::Type::Int,
        body: vec![
            kazuma::program::Statement::PrintNum(to_kazuma_expr(nf.expr)),
            kazuma::program::Statement::Return(kazuma::program::Expr::Literal(
                kazuma::program::Literal::Int(0),
            )),
        ],
    });

    kazuma::program::Module {
        name: name.to_string(),
        struct_types: vec![],
        global_var: HashMap::new(),
        funcs: funcs,
    }
}

pub fn codegen(e: expr::Expr, name: &str) {
    let e = e.make_name_unique();
    let normal_form = normal_form::NormalForm::from(e);
    let module = to_kazuma_module(name, normal_form);

    // write llvm-ir
    use std::fs;
    use std::io::Write;
    let mut f = fs::File::create(name).unwrap();
    match kazuma::generate(module) {
        Ok(code) => write!(f, "{}", code).unwrap(),
        Err(err) => panic!("{}", err),
    }
}
