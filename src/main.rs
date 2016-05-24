#![feature(plugin)]
#![plugin(peg_syntax_ext)]

#![feature(box_syntax)]


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Function {
    name: String,
    arg_name: String,
    arg_type: Box<Type>,
    body: Box<Expression>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expression {
    NumberLiteral(i32),
    Identifier(String),
    Lambda(String, Box<Type>, Box<Expression>),
    Range(Box<Expression>, Box<Expression>),

    Sequence(Box<Expression>, Box<Expression>),
    Let(String, Box<Type>, Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mult(Box<Expression>, Box<Expression>),
    Apply(Box<Expression>, Box<Expression>),
    Dot(Box<Expression>, Box<Expression>),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Type {
    Primary(String),
    Union(Box<Type>, Box<Type>),
    Tuple(Box<Type>, Box<Type>)
}

use syntax::*;
peg_file! syntax("syntax_rule");

#[test]
fn expression_test() {
    assert_eq!(expression("42"), Ok(Expression::NumberLiteral(42)));
    assert_eq!(expression("42+12"), Ok(Expression::Add(
                box Expression::NumberLiteral(42),
                box Expression::NumberLiteral(12)
                )));
    assert_eq!(expression("42+12*3"), Ok(Expression::Add(
                box Expression::NumberLiteral(42),
                box Expression::Mult(
                    box Expression::NumberLiteral(12),
                    box Expression::NumberLiteral(3)
                    )
                )));
    assert_eq!(expression("42; 42+12*3"), Ok(Expression::Sequence(
                box Expression::NumberLiteral(42),
                box Expression::Add(
                    box Expression::NumberLiteral(42),
                    box Expression::Mult(
                        box Expression::NumberLiteral(12),
                        box Expression::NumberLiteral(3)
                        )
                    )
                )));
    assert_eq!(expression("let x: Int = 12+42; 42+12*3"), Ok(
            Expression::Sequence(
                box Expression::Let("x".to_string(),
                    box Type::Primary("Int".to_string()),
                    box Expression::Add(
                        box Expression::NumberLiteral(12),
                        box Expression::NumberLiteral(42)
                        )),
                box Expression::Add(
                    box Expression::NumberLiteral(42),
                    box Expression::Mult(
                        box Expression::NumberLiteral(12),
                        box Expression::NumberLiteral(3)
                        )
                    )
                )));

    assert_eq!(expression("fizzbuzz@12*23"), Ok(
            Expression::Mult(
                box Expression::Apply(
                    box Expression::Identifier("fizzbuzz".to_string()),
                    box Expression::NumberLiteral(12)
                    ),
                box Expression::NumberLiteral(23),
                )));

    assert_eq!(expression("fizzbuzz@12*23"), Ok(
            Expression::Mult(
                box Expression::Apply(
                    box Expression::Identifier("fizzbuzz".to_string()),
                    box Expression::NumberLiteral(12)
                    ),
                box Expression::NumberLiteral(23),
                )));



}

#[test]
fn function_test() {
    assert_eq!(function("func main arg: Int { 0 }"), Ok(
        Function{
            name: "main".to_string(),
            arg_name: "arg".to_string(),
            arg_type: box Type::Primary("Int".to_string()),
            body: box Expression::NumberLiteral(0)
        }));
    assert_eq!(function("func main arg: Int { std.io.println@123 }"), Ok(
        Function{
            name: "main".to_string(),
            arg_name: "arg".to_string(),
            arg_type: box Type::Primary("Int".to_string()),
            body: box Expression::Apply(
                box Expression::Dot(
                    box Expression::Identifier("std".to_string()),
                    box Expression::Dot(
                        box Expression::Identifier("io".to_string()),
                        box Expression::Identifier("println".to_string())
                        )
                    ),
                box Expression::NumberLiteral(123)
                )
        }));
}

fn main() {
}

