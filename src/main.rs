/*============================================================================
  Copyright (C) 2015-2016 akitsu sanae
  https://github.com/akitsu-sanae/mumrik
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

#![feature(plugin)]
#![plugin(peg_syntax_ext)]

#![feature(box_syntax, box_patterns)]

mod ast;
mod test;
mod code_generator;

use code_generator::*;

peg_file! syntax("syntax_rule");
use syntax::*;

fn main() {
    println!("{}", code_gen_expr(&expression("let x: Int = 12+42; 42+12*3").unwrap()));
    println!("{}", code_gen_func(&function("func main (args: List[String]) -> Int { let x: Int = 12; args.count + x }").unwrap()));
}

