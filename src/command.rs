use std::collections::VecDeque;
use util;

mod build;
mod install;
mod new_;
mod run;

pub trait Command {
    fn work(self: Box<Self>);
}

#[derive(Clone)]
enum ParamInfo {
    Subcommand {
        name: &'static str,
        desc: &'static str,
        parse: fn(String, VecDeque<String>) -> Box<dyn Command>,
    },
    Option {
        name: &'static str,
        short_name: char,
        desc: &'static str,
        task: fn(String, Vec<ParamInfo>) -> !,
    },
}

fn param_name_max_len(param_infos: &Vec<ParamInfo>) -> (usize, usize) {
    param_infos.iter().fold(
        (0, 0),
        |(acc_subcommand, acc_option), param_info| match param_info {
            ParamInfo::Subcommand { name, .. } => {
                (std::cmp::max(name.len(), acc_subcommand), acc_option)
            }
            ParamInfo::Option { name, .. } => {
                (acc_subcommand, std::cmp::max(name.len(), acc_option))
            }
        },
    )
}

fn show_help(program_name: String, param_infos: Vec<ParamInfo>) -> ! {
    let (subcommand_max_len, option_max_len) = param_name_max_len(&param_infos);
    let (subcommand_usage, option_usage) = param_infos.iter().fold(
        (String::new(), String::new()),
        |(acc_subcommand, acc_option), param_info| match param_info {
            ParamInfo::Subcommand { name, desc, .. } => (
                format!(
                    "{}\n    {:width$}    {}",
                    acc_subcommand,
                    name,
                    desc,
                    width = subcommand_max_len
                ),
                acc_option,
            ),
            ParamInfo::Option {
                name,
                short_name,
                desc,
                ..
            } => (
                acc_subcommand,
                format!(
                    "{}\n    -{}, --{:width$}    {}",
                    acc_option,
                    short_name,
                    name,
                    desc,
                    width = option_max_len
                ),
            ),
        },
    );
    println!(
        r#"mumrik : a programming language
USAGE: {} [options...] <subcommand>

options:{}

subcommand:{}"#,
        program_name, option_usage, subcommand_usage,
    );
    std::process::exit(0)
}

pub fn parse_toplevel(mut args: VecDeque<String>) -> Box<dyn Command> {
    let param_infos = vec![
        ParamInfo::Option {
            name: "help",
            short_name: 'h',
            desc: "prints help infomation",
            task: show_help,
        },
        ParamInfo::Subcommand {
            name: "new",
            desc: "create new mumrik project with default config file",
            parse: new_::NewCommand::parse,
        },
        ParamInfo::Subcommand {
            name: "build",
            desc: "compile the local mumrik program and all of its dependencies",
            parse: build::BuildCommand::parse,
        },
        ParamInfo::Subcommand {
            name: "run",
            desc: "compile and run the local mumrik program",
            parse: run::RunCommand::parse,
        },
        ParamInfo::Subcommand {
            name: "install",
            desc: "download git repository from github for the local mumrik project",
            parse: install::InstallCommand::parse,
        },
    ];

    let program_name = args.pop_front().unwrap();
    let arg = if let Some(arg) = args.pop_front() {
        arg
    } else {
        show_help(program_name, param_infos)
    };

    let param_info = param_infos
        .iter()
        .find(|param_info| match param_info {
            ParamInfo::Subcommand { name, .. } => name == &arg.as_str(),
            ParamInfo::Option {
                name, short_name, ..
            } => arg == format!("--{}", name) || arg == format!("-{}", short_name),
        })
        .clone();

    match param_info {
        Some(ParamInfo::Subcommand { parse, .. }) => parse(program_name, args),
        Some(ParamInfo::Option { task, .. }) => task(program_name, param_infos),
        None => {
            eprintln!("{}: no such subcommand `{}`", util::alert("error"), arg);
            std::process::exit(-1)
        }
    }
}
