use expr::Expr;
use std::collections::HashMap;
use type_::Type;

mod normal_form;

fn to_kazuma_params(_params: Vec<(String, Type)>) -> Vec<(String, kazuma::typ::Type)> {
    unimplemented!()
}
fn to_kazuma_type(_ty: Type) -> kazuma::typ::Type {
    unimplemented!()
}
fn to_kazuma_expr(_e: Expr) -> kazuma::program::Expr {
    unimplemented!()
}

fn to_kazuma_module(name: &str, nf: normal_form::NormalForm) -> kazuma::program::Module {
    let mut funcs = vec![];
    for let_ in nf.lets.into_iter() {
        funcs.push(kazuma::program::Func {
            name: let_.name,
            args: to_kazuma_params(let_.params),
            ret_type: to_kazuma_type(let_.ret_type),
            body: vec![kazuma::program::Statement::Expr(to_kazuma_expr(let_.body))],
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

pub fn codegen(e: Expr, name: &str) {
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
