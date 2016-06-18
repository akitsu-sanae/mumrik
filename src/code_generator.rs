/*============================================================================
  Copyright (C) 2015-2016 akitsu sanae
  https://github.com/akitsu-sanae/mumrik
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

/*
 * まず初めの段階としてC++に変換した結果を出力する
 */

use ast::*;

pub fn code_gen_expr(expr: &Expression) -> String {
    match expr {
        &Expression::NumberLiteral(ref n) => n.to_string(),
        &Expression::Identifier(ref id) => id.clone(),
        &Expression::Lambda(ref arg, _, ref e) => format!("[](auto {}) {{{}}}", arg, code_gen_expr(&*e)),
        &Expression::Range(_, _) => "umimplemented!!".to_string(),

        &Expression::Sequence(ref e1, ref e2) => {
            match *e2 {
                box Expression::Sequence(_, _) => format!("{}; {}", code_gen_expr(&*e1), code_gen_expr(&*e2)),
                _ => format!("{}; return {};", code_gen_expr(&*e1), code_gen_expr(&*e2)),
            }
        },
        &Expression::Let(ref id, _, ref e) => format!("auto {} = {}", id, code_gen_expr(&*e)),
        &Expression::Add(ref e1, ref e2) => format!("{} + {}", code_gen_expr(&*e1), code_gen_expr(&*e2)),
        &Expression::Sub(ref e1, ref e2) => format!("{} - {}", code_gen_expr(&*e1), code_gen_expr(&*e2)),
        &Expression::Mult(ref e1, ref e2) => format!("{} * {}", code_gen_expr(&*e1), code_gen_expr(&*e2)),
        &Expression::Apply(ref e1, ref e2) => format!("{}({})", code_gen_expr(&*e1), code_gen_expr(&*e2)),
        &Expression::Dot(ref e1, ref e2) => format!("{}.{}", code_gen_expr(&*e1), code_gen_expr(&*e2)),
    }
}

pub fn code_gen_type(t: &Type) -> String {
    match t {
        &Type::Primary(ref s) => s.clone(),
        &Type::Tuple(ref t1, ref t2) => format!("std::tuple<{}, {}>", code_gen_type(&*t1), code_gen_type(&*t2)),
        &Type::Dependent(ref s, ref t) => {
            match s.as_ref() {
                "List" => format!("std::vector<{}>", code_gen_type(&*t)),
                _ => "no such a dependent".to_string(),
            }
        },
        _ => "not implemented type".to_string()
    }
}

pub fn code_gen_func(func: &Function) -> String {
    // int main(int a) { return a + 2; }
    format!("auto {}({} {}) {{{}}}", func.name, code_gen_type(&*func.arg_type), func.arg_name, code_gen_expr(&*func.body))
}




