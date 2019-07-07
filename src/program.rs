use std::collections::HashMap;

use expr::*;
use type_::*;

pub struct Program {
    pub expr: Expr,
    pub typ_aliases: HashMap<String, Type>,
}
