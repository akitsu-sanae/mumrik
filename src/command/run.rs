use command::{build::BuildCommand, Command};
use config;
use std::collections::VecDeque;
use std::path::PathBuf;
use util;

pub struct RunCommand {
    pub src: Option<PathBuf>,
    pub output: Option<PathBuf>,
}

impl RunCommand {
    pub fn parse(program_name: String, mut args: VecDeque<String>) -> Box<dyn Command> {
        let mut src = None;
        let mut output = None;

        while let Some(arg) = args.pop_front() {
            if arg.as_str() == "--help" || arg.as_str() == "-h" {
                println!(
                    r#"mumrik-run : compile and run the local mumrik program
USAGE: {} run [options...] <filename>

options:
    --output, -o <output-filename>  write brinary code to <output-filename>, default: `a.out`
    --help, -h           print help information

filename: input mumrik program filename"#,
                    program_name
                );
                std::process::exit(0);
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

        box RunCommand {
            src: src,
            output: output,
        }
    }
}

impl Command for RunCommand {
    fn work(self: Box<RunCommand>) {
        let build_command = box BuildCommand {
            src: self.src,
            output: self.output.clone(),
        };
        build_command.work();
        let output = if let Some(output) = self.output {
            output
        } else {
            config::CONFIG.lock().unwrap().build.output.clone()
        };
        let result = std::process::Command::new(output)
            .output()
            .expect("internal error: failed to execute output binary");

        eprintln!("exit status: {}", result.status.code().unwrap());
        let stdout = std::str::from_utf8(&result.stdout)
            .expect("unrecognized output")
            .trim();
        if !stdout.is_empty() {
            println!("-=-=-= stdout =-=-=-");
            println!("```");
            println!("{}", stdout);
            println!("```");
        }
        let stderr = std::str::from_utf8(&result.stderr)
            .expect("unrecognized output")
            .trim();
        if !stderr.is_empty() {
            eprintln!("-=-=-= stderr =-=-=-");
            eprintln!("```");
            eprintln!("{}", stderr);
            eprintln!("```");
        }
    }
}
