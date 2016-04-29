#![feature(plugin)]
#![plugin(peg_syntax_ext)]

#![feature(box_syntax)]

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expression {
    NumberLiteral(i32),
    Identifier(String),
    Lambda(String, Box<Expression>),
    Range(Box<Expression>, Box<Expression>),

    Sequence(Box<Expression>, Box<Expression>),
    Let(String, Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Mult(Box<Expression>, Box<Expression>),
    Apply(Box<Expression>, Box<Expression>),
}

use syntax::*;
peg_file! syntax("syntax_rule");

fn main() {
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
    assert_eq!(expression("let x = 12+42; 42+12*3"), Ok(
            Expression::Sequence(
                box Expression::Let("x".to_string(),
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


    assert_eq!(statement("let x = 12+42; 42+12*3;;"), Ok(
            Expression::Sequence(
                box Expression::Let("x".to_string(),
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



}
