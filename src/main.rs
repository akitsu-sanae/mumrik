#![feature(box_patterns)]
#![feature(box_syntax)]

extern crate nf2llvmir as nf;
extern crate peg;

mod ast;
mod typecheck;
// mod codegen;
mod env;
// mod eval;
mod ident;
mod parser;

#[cfg(test)]
mod tests;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut src = String::new();
    let filename = std::env::args().nth(1).expect("filename is required");
    let f = File::open(filename.clone()).and_then(|mut f| f.read_to_string(&mut src));
    if f.is_ok() {
        exec(&src)
    } else {
        use std::process;
        eprintln!("can not load file: {}", filename);
        process::abort();
    }
}

struct Expected(Vec<String>);

impl Expected {
    pub fn from(set: peg::error::ExpectedSet) -> Expected {
        Expected(
            set.tokens()
                .filter(|s| *s != "\' \' | \'\\t\' | \'\\r\' | \'\\n\'")
                .map(|s| s.to_string())
                .collect(),
        )
    }
}

use std::fmt;
impl fmt::Display for Expected {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.join(", "))
    }
}

fn exec(src: &str) {
    match parser::program(src) {
        Ok(program) => {
            let expr = typecheck::check_program(&program).expect("type error");
            // let value = eval::expr(expr);
            // println!("{}: {}", value, ty);
            // codegen::codegen(expr, "output.ll");
        }
        Err(err) => {
            let lines: Vec<_> = src.split('\n').collect();
            println!("{}", lines[err.location.line - 1]);
            println!("\u{001B}[31m{}^", " ".repeat(err.location.column - 1));
            println!(
                "syntax error at line:{} column: {}\nexpected: {}\u{001B}[39m",
                err.location.line,
                err.location.column,
                Expected::from(err.expected)
            )
        }
    };
}
