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
            "load" => {
                print!("filename: ");
                io::stdout().flush().unwrap();
                let mut filename = String::new();
                io::stdin().read_line(&mut filename).unwrap();
                let mut src = String::new();
                File::open(filename.as_str().trim()).and_then(|mut f| {
                    f.read_to_string(&mut src)
                }).expect("no such file");
                exec(&src)
            },
            _ => exec(&line),
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
            println!("\u{001B}[31mparsing fail : {:?}\u{001B}[39m", err)
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

