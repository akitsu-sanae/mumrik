pub mod grammar {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

use self::grammar::*;
use program::*;

pub fn parse(src: &str) -> Result<Program, ParseError> {
    grammar::program(src)
}
