
#[derive(Debug, Clone)]
pub enum Expression {
    Number(i64),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mult(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Var(String),
    Lambda(String, Box<Expression>),
    Apply(Box<Expression>, Box<Expression>),
}

