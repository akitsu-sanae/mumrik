#![feature(box_patterns)]
#![feature(box_syntax)]

mod expr;
mod parser;
mod context;
mod program;
mod type_;

#[cfg(test)]
mod test;

use std::io::Read;
use std::env;
use std::fs::File;
use type_::Type;
use context::Context;

fn main() {
    let mut src = String::new();
    let filename = env::args().nth(1).expect("filename is required");
    let f = File::open(filename.clone()).and_then(|mut f| {
        f.read_to_string(&mut src)
    });
    if f.is_ok() {
        exec(&src)
    } else {
        use std::process;
        eprintln!("can not load file: {}", filename);
        process::abort();
    }
}

fn exec(src: &str) {
    use program::Program;
    match parser::parse(src) {
        Ok(Program{expr, typ_aliases}) => {
            let mut expr = expr;
            expr.subst_typealias(&typ_aliases);
            let ty = Type::from_expr(&expr, &Context::new()).expect("type error");
            let value = expr.eval(&Context::new()).expect("invalid operation");
            println!("{}: {}", value, ty);
        },
        Err(err) => {
            let lines: Vec<_> = src.split('\n').collect();
            println!("{}", lines[err.line - 1]);
            println!("\u{001B}[31m{}^", " ".repeat(err.column - 1));
            println!("syntax error at line:{} column: {}\nexpected: {:?}\u{001B}[39m", err.line, err.column, err.expected)
        }
    };
}

