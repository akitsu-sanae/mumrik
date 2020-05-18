use command::Command;
use std::collections::VecDeque;
use util;

pub struct NewCommand {
    pub project_name: String,
}

impl NewCommand {
    pub fn parse(program_name: &str, mut args: VecDeque<String>) -> Box<dyn Command> {
        let mut project_name = None;
        while let Some(arg) = args.pop_front() {
            if arg.as_str() == "--help" || arg.as_str() == "-h" {
                println!(
                    r#"mumrik-new : create a new mumrik project directory
USAGE: {} new <project-name>"#,
                    program_name
                );
                std::process::exit(0);
            } else if project_name.is_some() {
                panic!(
                    "{}: too many command line argument `{}`",
                    util::alert("error"),
                    arg
                );
            } else {
                project_name = Some(arg);
            }
        }

        let project_name = project_name.unwrap_or_else(|| {
            panic!(
                "{}: new subcommand needs project-name as argument",
                util::alert("error")
            )
        });
        box NewCommand {
            project_name: project_name,
        }
    }
}

impl Command for NewCommand {
    fn work(self: Box<NewCommand>) {
        use std::fs::{self, File};
        use std::io::Write;
        fs::create_dir(format!("./{}", self.project_name)).unwrap_or_else(|err| {
            eprintln!(
                "{}: {}",
                util::alert("failed to create project directory"),
                err
            );
            std::process::exit(-1)
        });
        fs::create_dir(format!("./{}/src", self.project_name)).unwrap_or_else(|err| {
            eprintln!(
                "{}: {}",
                util::alert("failed to create source directory"),
                err
            );
            std::process::exit(-1)
        });
        fs::create_dir(format!("./{}/build", self.project_name)).unwrap_or_else(|err| {
            eprintln!(
                "{}: {}",
                util::alert("failed to create build directory"),
                err
            );
            std::process::exit(-1)
        });
        let mut main_src = File::create(format!("./{}/src/main.mm", self.project_name))
            .unwrap_or_else(|err| {
                eprintln!("{}: {}", util::alert("failed to create `main.mm`"), err);
                std::process::exit(-1)
            });
        main_src
            .write_all(
                br#"
rec func fib x: Int :Int {
    if x < 2 { 1 }
    else { fib (x-1) + fib (x-2) }
}

fib 8"#,
            )
            .unwrap_or_else(|err| {
                eprintln!(
                    "{}: {}",
                    util::alert("failed to write boilerplate code"),
                    err
                );
                std::process::exit(-1)
            });

        let mut conf = File::create(format!("./{}/mumrik-conf.toml", self.project_name))
            .unwrap_or_else(|err| {
                eprintln!(
                    "{}: {}",
                    util::alert("failed to create `mumrik-conf.toml`"),
                    err
                );
                std::process::exit(-1)
            });
        conf.write_all(
            br#"
[build]
src = "./src/main.mm"
output = "./build/output""#,
        )
        .unwrap_or_else(|err| {
            eprintln!(
                "{}: {}",
                util::alert("failed to write bilerplate code"),
                err
            );
            std::process::exit(-1)
        });
    }
}
