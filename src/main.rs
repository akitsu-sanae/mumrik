#![feature(plugin)]
#![plugin(peg_syntax_ext)]

mod syntax;


use std::collections::HashMap;

type Environment = HashMap<String, i64>;

enum RunResult {
    Success,
    TypeError
}

fn eval_expression (e: syntax::Expression, env: Environment) ->i64 {
    match e {
        syntax::Expression::Number(n) => n,
        syntax::Expression::Add(l ,r) => eval_expression (*l, env.clone()) + eval_expression (*r, env.clone()),
        syntax::Expression::Sub(l, r) => eval_expression (*l, env.clone()) - (eval_expression (*r, env.clone())),
        syntax::Expression::Mult(l, r) => eval_expression (*l, env.clone()) * eval_expression (*r, env.clone()),
        syntax::Expression::Div(l, r) => eval_expression (*l, env.clone()) / eval_expression (*r, env.clone()),
        syntax::Expression::Identifier(name) => env[&name],
        syntax::Expression::Let(_, _, e, body) => eval_expression(*body, env)
    }
}

fn main() {
    let env = Environment::new();
    println!("{}", eval_expression(
            syntax::parse_expr("let a : int = 12 let b : int = 23 a * b"),
            env
            ));
}
