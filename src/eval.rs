use ast::Expression;

type Env = Vec<(String, Box<Expression>)>;

pub fn eval(expr: Expression, env: &Env) -> Expression {
    match expr {
        Expression::Number(num) => Expression::Number(num),
        Expression::Bool(_) => expr,
        Expression::Add(box e1, box e2) => match (eval(e1, env), eval(e2, env)) {
            (Expression::Number(a), Expression::Number(b)) => Expression::Number(a+b),
            _ => panic!("add non number expression"),
        },
        Expression::Sub(box e1, box e2) => match (eval(e1, env), eval(e2, env)) {
            (Expression::Number(a), Expression::Number(b)) => Expression::Number(a-b),
            _ => panic!("add non number expression"),
        },
        Expression::Mult(box e1, box e2) => match (eval(e1, env), eval(e2, env)) {
            (Expression::Number(a), Expression::Number(b)) => Expression::Number(a*b),
            _ => panic!("add non number expression"),
        },
        Expression::Div(box e1, box e2) => match (eval(e1, env), eval(e2, env)) {
            (Expression::Number(a), Expression::Number(b)) => Expression::Number(a/b),
            _ => panic!("add non number expression"),
        },
        Expression::Apply(box e1, box e2) => {
            let f = eval(e1, env);
            let arg = eval(e2, env);
            match f {
                Expression::Closure(name, body) => {
                    let mut new_env = env.clone();
                    new_env.insert(0, (name, box arg));
                    eval(*body, &new_env)
                },
                _ => Expression::Error(format!("apply to non closure expression: {:?}", f))
            }
        },
        Expression::Var(name) => {
            if let Some(e) = env.iter().find(|&e| e.0 == name) {
                eval(*e.1.clone(), env)
            } else {
                Expression::Error(format!("no such variable: {}", name))
            }
        },
        Expression::Let(name, box init, box e) => {
            let mut new_env = env.clone();
            new_env.insert(0, (name, box eval(init, env)));
            eval(e, &new_env)
        },
        Expression::Closure(name, box body) => Expression::Closure(name, box body),

        Expression::Error(_) => expr,
    }
}
