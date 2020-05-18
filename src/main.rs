#![feature(box_patterns)]
#![feature(box_syntax)]

#[macro_use]
extern crate lazy_static;
extern crate nf2llvmir as nf;
extern crate peg;
extern crate serde_derive;
extern crate tempfile;
extern crate toml;

mod ast;
mod codegen;
mod command;
mod config;
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
