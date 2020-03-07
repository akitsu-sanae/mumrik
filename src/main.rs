#![feature(box_patterns)]
#![feature(box_syntax)]

extern crate peg;

mod context;
mod expr;
mod type_;

use context::Context;
use std::env;
use std::fs::File;
use std::io::Read;
use type_::Type;

fn main() {
    let mut src = String::new();
    let filename = env::args().nth(1).expect("filename is required");
    let f = File::open(filename.clone()).and_then(|mut f| f.read_to_string(&mut src));
    if f.is_ok() {
        exec(&src)
    } else {
        use std::process;
        eprintln!("can not load file: {}", filename);
        process::abort();
    }
}

fn exec(src: &str) {
    match expr::parser::expr(src) {
        Ok(expr) => {
            let ty = Type::from_expr(&expr, &Context::new()).expect("type error");
            let value = expr.eval(&Context::new()).expect("invalid operation");
            println!("{}: {}", value, ty);
        }
        Err(err) => {
            let lines: Vec<_> = src.split('\n').collect();
            println!("{}", lines[err.location.line - 1]);
            println!("\u{001B}[31m{}^", " ".repeat(err.location.column - 1));
            println!(
                "syntax error at line:{} column: {}\nexpected: {:?}\u{001B}[39m",
                err.location.line, err.location.column, err.expected
            )
        }
    };
}
