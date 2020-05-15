use std::collections::VecDeque;
use util;

mod build;

pub trait Command {
    fn work(self);
}

pub fn parse_toplevel(mut args: VecDeque<String>) -> impl Command {
    let program_name = args.pop_front().unwrap();
    let subcommand_name = args.pop_front();
    let subcommand_name = subcommand_name.as_ref().map(String::as_str);
    match subcommand_name {
        Some("build") => build::BuildCommand::parse(&program_name, args),
        Some("--help") | Some("-h") | None => {
            println!(
                r#"mumrik : a programming language
USAGE: {} [options...] <subcommand>

options:
    --help, -h    prints help infomation

subcommand:
    build         compile a local mumrik program and all of its dependencies"#,
                program_name
            );
            std::process::exit(0);
        }
        Some(subcommand_name) => {
            eprintln!(
                "{}: no such subcommand `{}`",
                util::alert("error"),
                subcommand_name
            );
            std::process::exit(-1);
        }
    }
}
