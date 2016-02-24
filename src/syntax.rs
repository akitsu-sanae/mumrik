
#[derive(PartialEq, Eq, Debug)]
pub enum Type {
    Atomic(String),
    List(Box<Type>),
    Function(Box<Type>, Box<Type>),
    Variant(Box<Type>, Box<Type>),
    Tuple(Box<Type>, Box<Type>)
}

#[derive(PartialEq, Eq, Debug)]
pub enum Expression {
    Number(i64),
    Identifier(String),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mult(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Let(String, Type, Box<Expression>, Box<Expression>)
}

#[derive(PartialEq, Eq, Debug)]
pub struct Function {
    pub name : String,
    pub expr : Expression,
    pub type_ : Type
}

#[derive(PartialEq, Eq, Debug)]
pub struct Variable {
    pub name : String,
    pub expr : Expression,
    pub type_ : Type
}

#[derive(PartialEq, Eq, Debug)]
pub struct Class {
    pub name : String,
    pub functions : Vec<Function>,
    pub variables : Vec<Variable>
}

#[derive(PartialEq, Eq, Debug)]
pub struct Program {
    pub classes : Vec<Class>
}

pub fn parse_program(str : &str) -> Program {
    parse::program(str).unwrap()
}

pub fn parse_class(str : &str) -> Class {
    parse::class(str).unwrap()
}

pub fn parse_expr(str : &str) -> Expression {
    parse::expr(str).unwrap()
}

peg_file! parse("syntax_rule");



