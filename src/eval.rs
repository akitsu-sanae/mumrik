use ast::Expression;

type Env = Vec<(String, Box<Expression>)>;

pub fn eval(expr: Expression, env: &Env) -> Expression {
    match expr {
        Expression::Number(_) | Expression::Bool(_) |
        Expression::Closure(_, _, _) | Expression::Error(_) => expr,

        Expression::Add(box e1, box e2) => match (eval(e1.clone(), env), eval(e2.clone(), env)) {
            (Expression::Number(a), Expression::Number(b)) => Expression::Number(a+b),
            _ => Expression::Error(format!("non number expression cen not be added: {:?} + {:?}", e1, e2)),
        },
        Expression::Sub(box e1, box e2) => match (eval(e1.clone(), env), eval(e2.clone(), env)) {
            (Expression::Number(a), Expression::Number(b)) => Expression::Number(a-b),
            _ => Expression::Error(format!("non number expression cen not be subed: {:?} + {:?}", e1, e2)),
        },
        Expression::Mult(box e1, box e2) => match (eval(e1.clone(), env), eval(e2.clone(), env)) {
            (Expression::Number(a), Expression::Number(b)) => Expression::Number(a*b),
            _ => Expression::Error(format!("non number expression cen not be multed: {:?} + {:?}", e1, e2)),
        },
        Expression::Div(box e1, box e2) => match (eval(e1.clone(), env), eval(e2.clone(), env)) {
            (Expression::Number(a), Expression::Number(b)) => Expression::Number(a/b),
            _ => Expression::Error(format!("non number expression cen not be dived: {:?} + {:?}", e1, e2)),
        },
        Expression::GreaterThan(box lhs, box rhs) => {
            match (eval(lhs.clone(), env), eval(rhs.clone(), env)) {
                (Expression::Number(a), Expression::Number(b)) => Expression::Bool(a > b),
                _ => Expression::Error(format!("can not compare: {:?} and {:?}", lhs, rhs)),
            }
        },
        Expression::LessThan(box lhs, box rhs) => {
            match (eval(lhs.clone(), env), eval(rhs.clone(), env)) {
                (Expression::Number(a), Expression::Number(b)) => Expression::Bool(a < b),
                _ => Expression::Error(format!("can not compare: {:?} and {:?}", lhs, rhs)),
            }
        },
        Expression::Equal(box lhs, box rhs) => {
            match (eval(lhs.clone(), env), eval(rhs.clone(), env)) {
                (Expression::Number(a), Expression::Number(b)) => Expression::Bool(a == b),
                (Expression::Bool(a) , Expression::Bool(b)) => Expression::Bool(a == b),
                _ => Expression::Error(format!("can not compare: {:?} and {:?}", lhs, rhs)),
            }
        },
        Expression::NotEqual(box lhs, box rhs) => {
            match (eval(lhs.clone(), env), eval(rhs.clone(), env)) {
                (Expression::Number(a), Expression::Number(b)) => Expression::Bool(a != b),
                (Expression::Bool(a), Expression::Bool(b)) => Expression::Bool(a != b),
                _ => Expression::Error(format!("can not compare: {:?} and {:?}", lhs, rhs)),
            }
        },
        Expression::Apply(box e1, box e2) => {
            let f = eval(e1, env);
            let arg = eval(e2, env);
            match f {
                Expression::Closure(name, _, body) => {
                    let mut new_env = env.clone();
                    new_env.insert(0, (name, box arg));
                    eval(*body, &new_env)
                },
                _ => Expression::Error(format!("apply to non closure expression: {:?}", f))
            }
        },
        Expression::If(box cond, box true_expr, box false_expr) => {
            let cond = eval(cond, env);
            match cond {
                Expression::Bool(true) => eval(true_expr, env),
                Expression::Bool(false) => eval(false_expr, env),
                _ => Expression::Error(format!("can not implicitly cast {:?} to bool", cond)),
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
    }
}
