
#![feature(box_syntax)]
#![feature(box_patterns)]

use std::io;
use std::io::Write;
use std::io::Read;
use std::io::stdout;
use std::fs::File;

#[macro_use]
extern crate nom;

mod ast;
mod parser;
mod eval;
mod tpe;

#[cfg(test)]
mod test;

use parser::expression;
use ast::Expression;
use tpe::Type;

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
            "load" => {
                print!("filename: ");
                io::stdout().flush().unwrap();
                let mut filename = String::new();
                io::stdin().read_line(&mut filename).unwrap();
                let mut src = String::new();
                File::open(filename.as_str().trim()).and_then(|mut f| {
                    f.read_to_string(&mut src)
                }).expect("not such file");
                exec(src)
            },
            _ => exec(line),
        }
    }
}

fn exec(src: String) {
    let ast = expression(src.as_bytes());
    let ty = tpe::check(&ast, &vec![]);
    match ty {
        Type::Error(msg) =>
            println!("\u{001B}[31mtype error\u{001B}[39m: {}", msg),
        _ => {
            println!("type: {:?}", ty);
            let expr = eval::eval(&ast, &vec![]);
            match expr {
                Expression::Error(msg) =>
                    println!("\u{001B}[31mtype error\u{001B}[39m: {}", msg),
                _ => println!("value: {:?}", expr),
            }
        },
    }
}

