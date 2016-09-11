use ast::Expression;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Primitive(String),
    Function(Box<Type>, Box<Type>),
    Variant(Box<Type>, Box<Type>),
    Tuple(Box<Type>, Box<Type>),

    Error(String),
}

type Env = Vec<(String, Box<Type>)>;

pub fn check(expr: &Expression, env: &Env) -> Type {
    match expr {
        &Expression::Number(_) => Type::Primitive("int".to_string()),
        &Expression::Bool(_) => Type::Primitive("bool".to_string()),
        &Expression::Closure(ref name, box ref ty, box ref body) => {
            let mut new_env = env.clone();
            new_env.insert(0, (name.clone(), box ty.clone()));
            Type::Function(box ty.clone(), box check(body, &new_env))
        },
        &Expression::Add(box ref lhs, box ref rhs) | &Expression::Sub(box ref lhs, box ref rhs) |
        &Expression::Mult(box ref lhs, box ref rhs) | &Expression::Div(box ref lhs, box ref rhs) => {
            let left_type = check(lhs, env);
            let right_type = check(rhs, env);
            if left_type == right_type {
                left_type
            } else {
                Type::Error(format!("'+' no much type: {:?} and {:?}", left_type, right_type))
            }
        },
        &Expression::GreaterThan(box ref lhs, box ref rhs) | &Expression::LessThan(box ref lhs, box ref rhs) |
        &Expression::Equal(box ref lhs, box ref rhs) | &Expression::NotEqual(box ref lhs, box ref rhs) => {
            let left_type = check(lhs, env);
            let right_type = check(rhs, env);
            if left_type == right_type {
                Type::Primitive("bool".to_string())
            } else {
                Type::Error(format!("'+' no much type: {:?} and {:?}", left_type, right_type))
            }
        },
        &Expression::Apply(box ref f, box ref arg) => {
            let arg_type = check(arg, env);
            let f_type = check(f, env);
            match f_type {
                Type::Function(box f, box body) => if f == arg_type { body }
                else { Type::Error(format!("no much type: {:?} and {:?}", f, arg_type)) },
                _ => Type::Error(format!("can not apply to non function: {:?}", f_type))
            }
        },
        &Expression::If(box ref cond, box ref t, box ref f) => {
            match check(cond, env) {
                Type::Primitive(ref name) if name == &"bool".to_string() => {
                    Type::Error(format!("condition in if expression must be boolean: {:?}", cond))
                },
                _ => {
                    let true_type = check(t, env);
                    let false_type = check(f, env);
                    if true_type == false_type {
                        true_type
                    } else {
                        Type::Error(format!("type not much: {:?} and {:?}", true_type, false_type))
                    }
                }
            }
        },
        &Expression::Var(ref name) => {
            if let Some(e) = env.iter().find(|&e| e.0 == name.clone()) {
                *e.1.clone()
            } else {
                Type::Error(format!("no such variable: {}", name))
            }
        },
        &Expression::Let(ref name, box ref init, box ref e) => {
            let mut new_env = env.clone();
            new_env.insert(0, (name.clone(), box check(init, env)));
            check(e, &new_env)
        },
        &Expression::Error(_) => panic!("invalid expression")
    }
}

