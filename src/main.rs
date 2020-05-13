#![feature(box_patterns)]
#![feature(box_syntax)]

extern crate nf2llvmir as nf;
extern crate peg;
extern crate tempfile;

mod args;
mod ast;
mod codegen;
mod env;
mod ident;
mod parser;
mod typecheck;
mod util;

#[cfg(test)]
mod tests;

fn main() {
    let args = args::Args::new();
    let (expr, _) = read_file(&args.input_filename);
    codegen::codegen(expr, &args.output_filename);
}

fn read_file(filename: &str) -> (ast::Expr, ast::Type) {
    use std::io::Read;
    let mut input_src = String::new();
    let f = std::fs::File::open(filename).and_then(|mut f| f.read_to_string(&mut input_src));
    if !f.is_ok() {
        eprintln!("can not read file: {}", filename);
        std::process::exit(-1);
    }

    let lines: Vec<_> = input_src.split('\n').collect();
    let program = match parser::program(&input_src) {
        Ok(program) => program,
        Err(err) => {
            let msg = format!(
                "{}\n{}\u{001B}[31m^\u{001B}[39m\nexpected: {}",
                lines[err.location.line - 1],
                " ".repeat(err.location.column - 1),
                parser::Expected::from(err.expected),
            );
            eprintln!(
                "\u{001B}[31m[syntax error]\u{001B}[39m at ({}, {})\n{}",
                err.location.line, err.location.column, msg
            );
            std::process::exit(-1)
        }
    };

    let expr = program
        .imports
        .into_iter()
        .fold(program.expr, |acc, import| {
            let filename = format!("{}.mm", import);
            let (expr, _) = read_file(&filename);
            match expr {
                ast::Expr::Let(name, typ, box e, box ast::Expr::EmptyMark, pos) => {
                    ast::Expr::Let(name, typ, box e, box acc, pos)
                }
                ast::Expr::Func {
                    name,
                    param_name,
                    param_type,
                    ret_type,
                    box body,
                    left: box ast::Expr::EmptyMark,
                    pos,
                } => ast::Expr::Func {
                    name: name,
                    param_name: param_name,
                    param_type: param_type,
                    ret_type: ret_type,
                    body: box body,
                    left: box acc,
                    pos: pos,
                },
                _ => unreachable!(),
            }
        });

    match typecheck::check(expr) {
        Ok((expr, typ)) => (expr, typ),
        Err(err) => match err {
            typecheck::Error::RecursiveOccurrence { pos, var, typ } => {
                let start = util::pos_to_location(&input_src, pos.start);
                let end = util::pos_to_location(&input_src, pos.end);
                eprintln!(
                    "\u{001B}[31m[type error]\u{001B}[39m at ({}, {})-({}, {})",
                    start.0, start.1, end.0, end.1
                );
                let lines: Vec<_> = input_src.split('\n').collect();
                eprintln!("```");
                for line_i in start.0..end.0 {
                    eprintln!("{}", lines[line_i - 1]);
                }
                eprintln!("```");
                eprintln!("type variable {} occurs recursively in {}", var, typ);
                std::process::exit(-1)
            }
            typecheck::Error::UnmatchType {
                pos,
                expected,
                actual,
            } => {
                let start = util::pos_to_location(&input_src, pos.start);
                let end = util::pos_to_location(&input_src, pos.end);
                eprintln!(
                    "\u{001B}[31m[type error]\u{001B}[39m at ({}, {})-({}, {})",
                    start.0, start.1, end.0, end.1
                );
                let lines: Vec<_> = input_src.split('\n').collect();
                eprintln!("```");
                for line_i in start.0..end.0 {
                    eprintln!("{}", lines[line_i]);
                }
                eprintln!("```");
                eprintln!(
                    "expected type is {}, but actual type is {}",
                    expected, actual
                );
                std::process::exit(-1)
            }
            typecheck::Error::UnboundVariable { pos, name } => {
                let (line, column_start) = util::pos_to_location(&input_src, pos.start);
                let (_, column_end) = util::pos_to_location(&input_src, pos.end);
                eprintln!(
                    "\u{001B}[31m[type error]\u{001B}[39m at line {}, unbound variable: {}",
                    line, name
                );
                let lines: Vec<_> = input_src.split('\n').collect();
                eprintln!("> {}", lines[line]);
                eprintln!(
                    "  {}{}",
                    " ".repeat(column_start - 1),
                    "^".repeat(column_end - column_start)
                );
                std::process::exit(-1)
            }
        },
    }
}
