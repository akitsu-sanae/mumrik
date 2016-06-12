/*============================================================================
  Copyright (C) 2015-2016 akitsu sanae
  https://github.com/akitsu-sanae/mumrik
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

#![feature(plugin)]
#![plugin(peg_syntax_ext)]

#![feature(box_syntax)]

mod ast;
mod test;

peg_file! syntax("syntax_rule");

fn main() {
}

