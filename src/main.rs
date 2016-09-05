
#![feature(box_syntax)]

use std::str;

#[macro_use]
extern crate nom;

mod ast;
mod parser;

named!(parens, delimited!(char!('('), is_not!(")"), char!(')')));

fn main() {
    println!("{:?}", str::from_utf8(parens("(tests)".as_bytes()).unwrap().1));
    println!("{:?}", parser::expression(b"1"));
    println!("{:?}", parser::expression(b"1+2"));
    println!("{:?}", parser::expression(b"1*2"));
    println!("{:?}", parser::expression(b"4+1*2"));
    println!("{:?}", parser::expression(b"5*4+1-4"));
    println!("{:?}", parser::expression(b"hoge"));
    println!("{:?}", parser::expression(b"hoge+1"));
    println!("{:?}", parser::expression(b"let a = 1+2; 12"));
    println!("{:?}", parser::expression(b"let a = 1+2; let b = 2+5; a*b"));
}

