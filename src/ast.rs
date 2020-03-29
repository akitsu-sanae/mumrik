pub mod parsed;
pub mod typed;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mult,
    Div,
    Eq,
    Neq,
    Lt,
    Gt,
}
