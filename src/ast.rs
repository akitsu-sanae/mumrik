
#[derive(Debug, Clone)]
pub enum Expression {
    Number(i64),
    Bool(bool),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mult(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Equal(Box<Expression>, Box<Expression>),
    Apply(Box<Expression>, Box<Expression>),
    If(Box<Expression>, Box<Expression>, Box<Expression>),
    Var(String),
    Let(String, Box<Expression>, Box<Expression>),
    Closure(String, Box<Expression>),

    Error(String),
}

