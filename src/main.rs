#![feature(box_patterns)]
#![feature(box_syntax)]

extern crate nf2llvmir as nf;
extern crate peg;

mod args;
mod ast;
mod codegen;
mod env;
mod eval;
mod ident;
mod parser;
mod typecheck;
mod util;

#[cfg(test)]
mod tests;

fn main() {
    let args = args::Args::new();

    match parser::program(&args.input_src) {
        Ok(expr) => match typecheck::check(expr) {
            Ok((expr, typ)) => {
                if let Some(output_filename) = args.output_filename {
                    if let Err(err) = codegen::codegen(expr, &output_filename) {
                        eprintln!("\u{001B}[31m[internal codegen error]\u{001B}[39m {}", err);
                        eprintln!(
                            "please report this issue to akitsu-sanae <akitsu.sanae@gmail.com>, the developer of mumrik language"
                        );
                        std::process::exit(-1);
                    }
                } else {
                    println!("{}: {}", eval::expr(expr), typ);
                }
            }
            Err(err) => match err {
                typecheck::Error::RecursiveOccurrence { pos, var, typ } => {
                    let start = util::pos_to_location(&args.input_src, pos.start);
                    let end = util::pos_to_location(&args.input_src, pos.end);
                    eprintln!(
                        "\u{001B}[31m[type error]\u{001B}[39m at ({}, {})-({}, {})",
                        start.0, start.1, end.0, end.1
                    );
                    let lines: Vec<_> = args.input_src.split('\n').collect();
                    eprintln!("```");
                    for line_i in start.0..end.0 {
                        eprintln!("{}", lines[line_i - 1]);
                    }
                    eprintln!("```");
                    eprintln!("type variable {} occurs recursively in {}", var, typ);
                    std::process::exit(-1);
                }
                typecheck::Error::UnmatchType {
                    pos,
                    expected,
                    actual,
                } => {
                    let start = util::pos_to_location(&args.input_src, pos.start);
                    let end = util::pos_to_location(&args.input_src, pos.end);
                    eprintln!(
                        "\u{001B}[31m[type error]\u{001B}[39m at ({}, {})-({}, {})",
                        start.0, start.1, end.0, end.1
                    );
                    let lines: Vec<_> = args.input_src.split('\n').collect();
                    eprintln!("```");
                    for line_i in start.0..end.0 {
                        eprintln!("{}", lines[line_i]);
                    }
                    eprintln!("```");
                    eprintln!(
                        "expected type is {}, but actual type is {}",
                        expected, actual
                    );
                    std::process::exit(-1);
                }
                typecheck::Error::UnboundVariable { pos, name } => {
                    let (line, column_start) = util::pos_to_location(&args.input_src, pos.start);
                    let (_, column_end) = util::pos_to_location(&args.input_src, pos.end);
                    eprintln!(
                        "\u{001B}[31m[type error]\u{001B}[39m at line {}, unbound variable: {}",
                        line, name
                    );
                    let lines: Vec<_> = args.input_src.split('\n').collect();
                    eprintln!("> {}", lines[line]);
                    eprintln!(
                        "  {}{}",
                        " ".repeat(column_start - 1),
                        "^".repeat(column_end - column_start)
                    );
                    std::process::exit(-1);
                }
            },
        },
        Err(err) => {
            let lines: Vec<_> = args.input_src.split('\n').collect();
            let msg = format!(
                "{}\n{}\u{001B}[31m^\u{001B}[39m\nexpected: {}",
                lines[err.location.line - 1],
                " ".repeat(err.location.column - 1),
                parser::Expected::from(err.expected),
            );
            eprintln!(
                "\u{001B}[31m[syntax error]\u{001B}[39m at ({}, {})\n{}",
                err.location.line, err.location.column, msg
            );
            std::process::exit(-1);
        }
    };
}
