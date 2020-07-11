use ast;
use codegen;
use command::Command;
use config;
use parser;
use std::collections::VecDeque;
use std::path::PathBuf;
use typecheck;
use util;

pub struct BuildCommand {
    pub src: Option<PathBuf>,
    pub output: Option<PathBuf>,
}

fn print_help(program_name: &str) {
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
}
impl BuildCommand {
    pub fn parse(program_name: String, mut args: VecDeque<String>) -> Box<dyn Command> {
        let mut src = None;
        let mut output = None;

        while let Some(arg) = args.pop_front() {
            if arg.as_str() == "--help" || arg.as_str() == "-h" {
                print_help(&program_name);
            } else if arg.as_str() == "--output" || arg.as_str() == "-o" {
                output = Some(PathBuf::from(args.pop_front().unwrap_or_else(|| {
                    panic!(
                        "{}: filename is required after `--output` or -o`",
                        util::alert("error")
                    )
                })));
            } else if arg.as_str().starts_with("--output=") {
                output = Some(PathBuf::from(arg[9..].to_string()));
            } else if arg.as_str().starts_with("-o=") {
                output = Some(PathBuf::from(arg[3..].to_string()));
            } else if src.is_some() {
                panic!(
                    "{}: too many command line argument `{}`",
                    util::alert("error"),
                    arg
                );
            } else {
                src = Some(PathBuf::from(arg));
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
        let mut entry_modules = vec![];
        if let Some(src_dir) = src.as_path().parent() {
            entry_modules.push(src_dir.to_path_buf());
        }
        entry_modules.push(config::CONFIG.lock().unwrap().build.dep.clone());
        let (expr, _) = read_file(&src, &entry_modules);
        codegen::codegen(expr, &output);
    }
}

fn append_import_path(path: PathBuf, import: ast::Import) -> PathBuf {
    let mut path = import.dirs.into_iter().fold(path, |mut acc, dir| {
        acc.push(format!("{}", dir));
        acc
    });
    path.push(format!("{}.mm", import.module_name));
    path
}

fn imported_filepath(entry_modules: &Vec<PathBuf>, import: ast::Import) -> Option<PathBuf> {
    for entry_module in entry_modules.iter() {
        let module_file_path = append_import_path(entry_module.clone(), import.clone());
        if module_file_path.is_file() {
            eprintln!("import: {}", module_file_path.to_str().unwrap());
            return Some(module_file_path);
        }
    }
    eprintln!("unknown import: {:?}", import); // TODO
    panic!();
}

fn read_file(input_path: &PathBuf, entry_modules: &Vec<PathBuf>) -> (ast::Expr, ast::Type) {
    use std::io::Read;
    let mut input_src = String::new();
    let f = std::fs::File::open(input_path).and_then(|mut f| f.read_to_string(&mut input_src));
    if !f.is_ok() {
        eprintln!("can not read file: {}", input_path.to_str().unwrap());
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
            if let Some(file_pathbuf) = imported_filepath(entry_modules, import) {
                let (expr, _) = read_file(&file_pathbuf, entry_modules);
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
            } else {
                acc
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
