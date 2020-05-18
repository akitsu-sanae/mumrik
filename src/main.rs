#![feature(box_patterns)]
#![feature(box_syntax)]

extern crate nf2llvmir as nf;
extern crate peg;
extern crate tempfile;

mod ast;
mod codegen;
mod command;
mod env;
mod ident;
mod parser;
mod typecheck;
mod util;

use std::collections::VecDeque;

#[cfg(test)]
mod tests;

fn main() {
    let args: VecDeque<_> = std::env::args().collect();
    command::parse_toplevel(args).work();
}
