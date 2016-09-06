
use parser::expression;
use eval::eval;

#[test]
fn parsing_test() {
    println!("{:?}", expression(b"1"));
    println!("{:?}", expression(b"1+2"));
    println!("{:?}", expression(b"1*2"));
    println!("{:?}", expression(b"4+1*2"));
    println!("{:?}", expression(b"5*4+1-4"));
    println!("{:?}", expression(b"hoge"));
    println!("{:?}", expression(b"hoge+1"));
    println!("{:?}", expression(b"let a = 1+2; 12"));
    println!("{:?}", expression(b"let a = 1+2; let b = 2+5; a*b"));
}

#[test]
fn eval_test() {
    println!("{:?}", eval(expression(b"123"), &vec![]));
    println!("{:?}", eval(expression(b"123+1"), &vec![]));
    println!("{:?}", eval(expression(b"123*2"), &vec![]));
    println!("{:?}", eval(expression(b"let a = 1+2; 12"), &vec![]));
    println!("{:?}", eval(expression(b"let b = 4*3; b+5"), &vec![]));
    println!("{:?}", eval(expression(b"let a = 4*3; let b = 3; a+b"), &vec![]));
}

