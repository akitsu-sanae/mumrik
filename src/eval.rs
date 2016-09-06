use std::process::exit;
use ast::Expression;

type Env = Vec<(String, Box<Expression>)>;

pub fn eval(expr: Expression, env: &Env) -> i64 {
    match expr {
        Expression::Number(num) => num,
        Expression::Add(box e1, box e2) => eval(e1, env) + eval(e2, env),
        Expression::Sub(box e1, box e2) => eval(e1, env) - eval(e2, env),
        Expression::Mult(box e1, box e2) => eval(e1, env) * eval(e2, env),
        Expression::Div(box e1, box e2) => eval(e1, env) / eval(e2, env),
        Expression::Var(name) => {
            if let Some(e) = env.iter().find(|&e| e.0 == name) {
                eval(*e.1.clone(), env)
            } else {
                println!("no such variable: {}", name);
                exit(-1);
            }
        },
        Expression::Let(name, box init, box e) => {
            let mut env = env.clone();
            env.push((name, box init));
            eval(e, &env)
        },
    }
}
