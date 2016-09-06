
#![feature(box_syntax)]
#![feature(box_patterns)]

use std::io;
use std::io::Write;
use std::io::stdout;

#[macro_use]
extern crate nom;

mod ast;
mod parser;
mod eval;
mod test;

use parser::expression;
use ast::Expression;

fn main() {
    println!("    \u{001B}[34m-=-=--=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-\u{001B}[39m");
    println!("             Mumrik   version 0.0.1             ");
    println!("    \u{001B}[34m-=-=--=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-\u{001B}[39m");
    println!("");
    loop {
        print!("# ");
        stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        match line.as_str().trim() {
            "quit" => return,
            "help" => {
                println!("how to use")
            },
            _ => {
                let expr = eval::eval(expression(line.as_bytes()), &vec![]);
                match expr {
                    Expression::Error(msg) =>
                        println!("\u{001B}[31merror\u{001B}[39m: {}", msg),
                    _ =>
                        println!("{:?}", expr),
                }
            },
        }
    }
}

