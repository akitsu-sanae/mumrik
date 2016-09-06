
#![feature(box_syntax)]
#![feature(box_patterns)]

use std::str;

#[macro_use]
extern crate nom;

mod ast;
mod parser;
mod eval;

fn main() {
    println!("{:?}", parser::expression(b"1"));
    println!("{:?}", parser::expression(b"1+2"));
    println!("{:?}", parser::expression(b"1*2"));
    println!("{:?}", parser::expression(b"4+1*2"));
    println!("{:?}", parser::expression(b"5*4+1-4"));
    println!("{:?}", parser::expression(b"hoge"));
    println!("{:?}", parser::expression(b"hoge+1"));
    println!("{:?}", parser::expression(b"let a = 1+2; 12"));
    println!("{:?}", parser::expression(b"let a = 1+2; let b = 2+5; a*b"));
    println!("{}", eval::eval(parser::expression(b"123"), &vec![]));
    println!("{}", eval::eval(parser::expression(b"123+1"), &vec![]));
    println!("{}", eval::eval(parser::expression(b"123*2"), &vec![]));
    println!("{}", eval::eval(parser::expression(b"let a = 1+2; 12"), &vec![]));
    println!("{}", eval::eval(parser::expression(b"let b = 4*3; b+5"), &vec![]));
    println!("{}", eval::eval(parser::expression(b"let a = 4*3; let b = 3; a+b"), &vec![]));
}

