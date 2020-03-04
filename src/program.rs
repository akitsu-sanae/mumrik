use std::collections::HashMap;

use expr::*;
use type_::*;

#[derive(Debug)]
pub struct Program {
    pub type_aliases: HashMap<String, Type>,
    pub expr: Expr,
}
