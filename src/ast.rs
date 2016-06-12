/*============================================================================
  Copyright (C) 2015-2016 akitsu sanae
  https://github.com/akitsu-sanae/mumrik
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Function {
    pub name: String,
    pub arg_name: String,
    pub arg_type: Box<Type>,
    pub return_type: Box<Type>,
    pub body: Box<Expression>,
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
    Tuple(Box<Type>, Box<Type>),
    Dependent(String, Box<Type>)
}


