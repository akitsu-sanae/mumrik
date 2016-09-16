
use ast::*;
use tpe::Type;

pub fn generate(program: Vec<Function>) -> String {
    let mut result = String::new();
    result += common_data().as_str();
    for func in program {
        result += format!("define {} @{}({}) {{\n{}\n}}\n",
        "i32",
        func.name,
        args(func.args),
        expr(func.body)).as_str()
    }
    result
}

fn common_data() -> String {
    format!("declare i32 @printf(i8*, ...)\n")
}

fn args(args: Vec<Arg>) -> String {
    args.iter().map(|arg| {
        let ty = match arg.tpe {
            box Type::Primitive(ref name) => {
                if name == "int" {
                    "i32".to_string()
                } else if name == "bool" {
                    "i1".to_string()
                } else {
                    format!("%{}", name)
                }
            },
        };
        format!("{} %{}", ty, arg.name)
    }).collect::<Vec<String>>().join(", ")
}

fn expr(_: Expression) -> String {
    "    ret i32 0".to_string()
}

