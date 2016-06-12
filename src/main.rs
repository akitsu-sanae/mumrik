#![feature(plugin)]
#![plugin(peg_syntax_ext)]

#![feature(box_syntax)]

mod ast;
mod test;

peg_file! syntax("syntax_rule");

fn main() {
}

