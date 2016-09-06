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
        Expression::Apply(box e1, box e2) => {
            let arg = Expression::Number(eval(e2, env));
            match e1 {
                Expression::Closure(name, body) => {
                    let mut new_env = env.clone();
                    new_env.push((name, box arg));
                    eval(*body, &new_env)
                },
                _ => {
                    println!("error: apply to non closure expression {:?}", e1);
                    exit(-1);
                }
            }
        },
        Expression::Var(name) => {
            if let Some(e) = env.iter().find(|&e| e.0 == name) {
                eval(*e.1.clone(), env)
            } else {
                println!("no such variable: {}", name);
                exit(-1);
            }
        },
        Expression::Let(name, box init, box e) => {
            let mut new_env = env.clone();
            new_env.push((name, box Expression::Number(eval(init, env))));
            eval(e, &new_env)
        },
        Expression::Closure(_, _) => {
            println!("not implemented: eval closure {:?}", expr);
            42
        }
    }
}
