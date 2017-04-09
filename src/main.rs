#![feature(plugin)]
#![plugin(peg_syntax_ext)]


#![feature(box_patterns)]
#![feature(box_syntax)]

mod expr;
mod context;
mod type_;

#[cfg(test)]
mod test;

use std::io;
use std::io::Read;
use std::io::Write;
use std::fs::File;
use context::Context;

fn main() {
    println!("\u{001B}[34m-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-\u{001B}[39m");
    println!("    Mumrik version 0.0.1 by akitsu-sanae");
    println!("\u{001B}[34m-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-\u{001B}[39m");
    println!("");

    loop {
        print!("# ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        match line.as_str().trim() {
            "quit" => return,
            "help" => print_help(),
            line if line.split_whitespace().collect::<Vec<_>>()[0] == "load" => {
                let filename = line.split_whitespace().collect::<Vec<_>>()[1];
                io::stdout().flush().unwrap();
                let mut src = String::new();
                let f = File::open(filename).and_then(|mut f| {
                    f.read_to_string(&mut src)
                });
                if f.is_ok() {
                    exec(&src)
                } else {
                    println!("can not load file: {}", filename)
                }
            },
            _ => exec(&line)
        }
    }
}

peg_file! parse("grammar.rustpeg");

fn exec(src: &String) {
    match parse::expr(src.as_str()) {
        Ok(expr) => {
            println!("expr: {:?}", expr);
            println!("type: {:?}", expr.type_of(&Context::new()));
            println!("value: {:?}", expr.eval(&Context::new()));
        },
        Err(err) => {
            println!("\u{001B}[31m{}^", " ".repeat(err.column+1));
            println!("expected: {:?}\u{001B}[39m", err.expected)
        }
    }
}

fn print_help() {
    println!("\u{001B}[32m-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
    println!("    quit ... quit this interpreter");
    println!("    help ... print help like this!");
    println!("    load ... load file, and execute it as mumrik code");
    println!("    otherwise ... execute line as mumrik code");
    println!("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=\u{001B}[39m");
}

