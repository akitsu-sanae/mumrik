
mod grammar {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

use program::*;
use self::grammar::*;

pub fn parse(src: &str) -> Result<Program, ParseError> {
    grammar::program(src)
}


