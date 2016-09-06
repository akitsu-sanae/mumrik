
use ast::Expression;

pub fn eval(expr: Expression) -> i64 {
    match expr {
        Expression::Number(num) => num,
        Expression::Add(box e1, box e2) => eval(e1) + eval(e2),
        Expression::Sub(box e1, box e2) => eval(e1) - eval(e2),
        Expression::Mult(box e1, box e2) => eval(e1) * eval(e2),
        Expression::Div(box e1, box e2) => eval(e1) / eval(e2),
        Expression::Var(_) => 42,
        Expression::Let(_, _, box e) => eval(e),
    }
}
