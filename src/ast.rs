
use tpe::Type;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Number(i64),
    Bool(bool),
    Closure(String, Box<Type>, Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mult(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    GreaterThan(Box<Expression>, Box<Expression>),
    LessThan(Box<Expression>, Box<Expression>),
    Equal(Box<Expression>, Box<Expression>),
    NotEqual(Box<Expression>, Box<Expression>),
    Apply(Box<Expression>, Box<Expression>),
    If(Box<Expression>, Box<Expression>, Box<Expression>),
    Var(String),
    Let(String, Box<Expression>, Box<Expression>),

    Error(String),
}

