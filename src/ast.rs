
use tpe::Type;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub name: String,
    pub args: Vec<Arg>,
    pub body: Expression,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arg {
    pub name: String,
    pub tpe: Box<Type>,
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Number(i64),
    Bool(bool),
    Closure(String, Box<Type>, Box<Expression>),
    Var(String),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mult(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Mod(Box<Expression>, Box<Expression>),
    Less(Box<Expression>, Box<Expression>),
    Greater(Box<Expression>, Box<Expression>),
    LessOrEqual(Box<Expression>, Box<Expression>),
    GreaterOrEqual(Box<Expression>, Box<Expression>),
    Equal(Box<Expression>, Box<Expression>),
    NotEqual(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Not(Box<Expression>),
    Apply(Box<Expression>, Box<Expression>),
    Dot(Box<Expression>, Box<Expression>),
    If(Box<Expression>, Box<Expression>, Box<Expression>),
    Let(String, Box<Expression>, Box<Expression>),
}

