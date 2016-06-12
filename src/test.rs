/*============================================================================
  Copyright (C) 2015-2016 akitsu sanae
  https://github.com/akitsu-sanae/mumrik
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use ast::*;
use syntax::*;

#[test]
fn expression_test() {
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
    assert_eq!(expression("let x: Int = 12+42; 42+12*3"), Ok(
            Expression::Sequence(
                box Expression::Let("x".to_string(),
                    box Type::Primary("Int".to_string()),
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

    assert_eq!(expression("fizzbuzz@12*23"), Ok(
            Expression::Mult(
                box Expression::Apply(
                    box Expression::Identifier("fizzbuzz".to_string()),
                    box Expression::NumberLiteral(12)
                    ),
                box Expression::NumberLiteral(23),
                )));

    assert_eq!(expression("fib@(n+-1) + fib@(n+-2)"), Ok(
            Expression::Add(
                box Expression::Apply(
                    box Expression::Identifier("fib".to_string()),
                    box Expression::Add(
                        box Expression::Identifier("n".to_string()),
                        box Expression::NumberLiteral(-1)
                        )
                    ),
                box Expression::Apply(
                    box Expression::Identifier("fib".to_string()),
                    box Expression::Add(
                        box Expression::Identifier("n".to_string()),
                        box Expression::NumberLiteral(-2)
                        )
                    )
            )));


}

#[test]
fn function_test() {
    assert_eq!(function("func main (args: List[String]) -> Int { 0 }"), Ok(
        Function{
            name: "main".to_string(),
            arg_name: "args".to_string(),
            arg_type: box Type::Dependent(
                "List".to_string(),
                box Type::Primary("String".to_string())),
            return_type: box Type::Primary("Int".to_string()),
            body: box Expression::NumberLiteral(0)
        }));
    assert_eq!(function("
                        func fib (n: Int) -> Int {
                            fib@(n-1) + fib@(n-2)
                        }"), Ok(
        Function{
            name: "fib".to_string(),
            arg_name: "n".to_string(),
            arg_type: box Type::Primary("Int".to_string()),
            return_type: box Type::Primary("Int".to_string()),
            body: box Expression::Add(
                box Expression::Apply(
                    box Expression::Identifier("fib".to_string()),
                    box Expression::Sub(
                        box Expression::Identifier("n".to_string()),
                        box Expression::NumberLiteral(1)
                        )
                    ),
                box Expression::Apply(
                    box Expression::Identifier("fib".to_string()),
                    box Expression::Sub(
                        box Expression::Identifier("n".to_string()),
                        box Expression::NumberLiteral(2)
                        )
                    )
                )
        }));
    assert_eq!(function("
                        func main (args: List[String * Int]) -> Int {
                            std.io.println@123
                        }"), Ok(
        Function{
            name: "main".to_string(),
            arg_name: "args".to_string(),
            arg_type: box Type::Dependent("List".to_string(), box Type::Tuple(box Type::Primary("String".to_string()), box Type::Primary("Int".to_string()))),
            return_type: box Type::Primary("Int".to_string()),
            body: box Expression::Apply(
                box Expression::Dot(
                    box Expression::Identifier("std".to_string()),
                    box Expression::Dot(
                        box Expression::Identifier("io".to_string()),
                        box Expression::Identifier("println".to_string())
                        )
                    ),
                box Expression::NumberLiteral(123)
                )
        }));
}


