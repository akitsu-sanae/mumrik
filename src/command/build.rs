use ast;
use codegen;
use command::Command;
use config;
use parser;
use std::collections::VecDeque;
use typecheck;
use util;

pub struct BuildCommand {
    pub src: Option<String>,
    pub output: Option<String>,
}

impl BuildCommand {
    pub fn parse(program_name: String, mut args: VecDeque<String>) -> Box<dyn Command> {
        let mut src = None;
        let mut output = None;

        while let Some(arg) = args.pop_front() {
            if arg.as_str() == "--help" || arg.as_str() == "-h" {
                println!(
                    r#"mumrik-build : compile a local mumrik program and all of its dependencies
USAGE: {} build [options...] <filename>

options:
    --output, -o <output-filename>  write brinary code to <output-filename>, default: `a.out`
    --help, -h           print help information

filename: input mumrik program filename"#,
                    program_name
                );
                std::process::exit(0);
            } else if arg.as_str() == "--output" || arg.as_str() == "-o" {
                output = Some(args.pop_front().unwrap_or_else(|| {
                    panic!(
                        "{}: filename is required after `--output` or -o`",
                        util::alert("error")
                    )
                }));
            } else if arg.as_str().starts_with("--output=") {
                output = Some(arg[9..].to_string());
            } else if arg.as_str().starts_with("-o=") {
                output = Some(arg[3..].to_string());
            } else if src.is_some() {
                panic!(
                    "{}: too many command line argument `{}`",
                    util::alert("error"),
                    arg
                );
            } else {
                src = Some(arg);
            }
        }

        box BuildCommand {
            src: src,
            output: output,
        }
    }
}

impl Command for BuildCommand {
    fn work(self: Box<BuildCommand>) {
        let src = if let Some(src) = self.src {
            src
        } else {
            config::CONFIG.lock().unwrap().build.src.clone()
        };
        let output = if let Some(output) = self.output {
            output
        } else {
            config::CONFIG.lock().unwrap().build.output.clone()
        };
        let (expr, _) = read_file(&src);
        codegen::codegen(expr, &output);
    }
}

fn read_file(filename: &str) -> (ast::Expr, ast::Type) {
    use std::io::Read;
    let mut input_src = String::new();
    let f = std::fs::File::open(filename).and_then(|mut f| f.read_to_string(&mut input_src));
    if !f.is_ok() {
        eprintln!("can not read file: {}", filename);
        std::process::exit(-1);
    }

    let lines: Vec<_> = input_src.split('\n').collect();
    let program = match parser::program(&input_src) {
        Ok(program) => program,
        Err(err) => {
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
            std::process::exit(-1)
        }
    };

    let expr = program
        .imports
        .into_iter()
        .fold(program.expr, |acc, import| {
            let filename = format!("{}.mm", import);
            let (expr, _) = read_file(&filename);
            match expr {
                ast::Expr::Let(name, typ, box e, box ast::Expr::EmptyMark, pos) => {
                    ast::Expr::Let(name, typ, box e, box acc, pos)
                }
                ast::Expr::Func {
                    name,
                    param_name,
                    param_type,
                    ret_type,
                    box body,
                    left: box ast::Expr::EmptyMark,
                    pos,
                } => ast::Expr::Func {
                    name: name,
                    param_name: param_name,
                    param_type: param_type,
                    ret_type: ret_type,
                    body: box body,
                    left: box acc,
                    pos: pos,
                },
                _ => unreachable!(),
            }
        });

    match typecheck::check(expr) {
        Ok((expr, typ)) => (expr, typ),
        Err(err) => match err {
            typecheck::Error::RecursiveOccurrence { pos, var, typ } => {
                let start = util::pos_to_location(&input_src, pos.start);
                let end = util::pos_to_location(&input_src, pos.end);
                eprintln!(
                    "\u{001B}[31m[type error]\u{001B}[39m at ({}, {})-({}, {})",
                    start.0, start.1, end.0, end.1
                );
                let lines: Vec<_> = input_src.split('\n').collect();
                eprintln!("```");
                for line_i in start.0..end.0 {
                    eprintln!("{}", lines[line_i - 1]);
                }
                eprintln!("```");
                eprintln!("type variable {} occurs recursively in {}", var, typ);
                std::process::exit(-1)
            }
            typecheck::Error::UnmatchType {
                pos,
                expected,
                actual,
            } => {
                let start = util::pos_to_location(&input_src, pos.start);
                let end = util::pos_to_location(&input_src, pos.end);
                eprintln!(
                    "\u{001B}[31m[type error]\u{001B}[39m at ({}, {})-({}, {})",
                    start.0, start.1, end.0, end.1
                );
                let lines: Vec<_> = input_src.split('\n').collect();
                eprintln!("```");
                for line_i in start.0..end.0 {
                    eprintln!("{}", lines[line_i]);
                }
                eprintln!("```");
                eprintln!(
                    "expected type is {}, but actual type is {}",
                    expected, actual
                );
                std::process::exit(-1)
            }
            typecheck::Error::UnboundVariable { pos, name } => {
                let (line, column_start) = util::pos_to_location(&input_src, pos.start);
                let (_, column_end) = util::pos_to_location(&input_src, pos.end);
                eprintln!(
                    "\u{001B}[31m[type error]\u{001B}[39m at line {}, unbound variable: {}",
                    line, name
                );
                let lines: Vec<_> = input_src.split('\n').collect();
                eprintln!("> {}", lines[line]);
                eprintln!(
                    "  {}{}",
                    " ".repeat(column_start - 1),
                    "^".repeat(column_end - column_start)
                );
                std::process::exit(-1)
            }
        },
    }
}
