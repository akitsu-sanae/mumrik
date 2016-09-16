
#![feature(box_syntax)]
#![feature(box_patterns)]

use std::io::Write;
use std::io::Read;
use std::fs::File;
use std::env;

#[macro_use]
extern crate nom;

mod ast;
mod tpe;
mod parser;
mod generator;


use parser::program;
use generator::generate;

fn main() {
    let filename = env::args().nth(1).expect("filename was not given");

    let mut src = String::new();
    File::open(filename.as_str().trim()).and_then(|mut f| {
        f.read_to_string(&mut src)
    }).expect("not such file");

    use nom::IResult;
    let ast = match program(src.as_bytes()) {
        IResult::Done(_, o) => o,
        IResult::Error(e) => panic!(format!("parsing error: {:?}", e)),
        IResult::Incomplete(n) => panic!(format!("imcomplete: {:?}", n)),
    };

    let output = generate(ast);
    File::create("output.ll").and_then(|mut f| {
        write!(f, "{}", output)
    }).expect("fail to create file: output.ll");
}

