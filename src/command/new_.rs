use command::Command;
use std::collections::VecDeque;
use util;

pub struct NewCommand {
    pub project_name: String,
}

impl NewCommand {
    pub fn parse(program_name: String, mut args: VecDeque<String>) -> Box<dyn Command> {
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

fn create_dir(path: &str, label: &str) {
    std::fs::create_dir(path).unwrap_or_else(|err| {
        eprintln!(
            "{}: {}",
            util::alert(&format!("failed to create {} directory", label)),
            err
        );
        std::process::exit(-1)
    });
}

fn create_file(path: &str, filename: &str, content: &[u8]) {
    use std::io::Write;
    let mut file = std::fs::File::create(path).unwrap_or_else(|err| {
        eprintln!(
            "{}: {}",
            util::alert(&format!("failed to create `{}`", filename)),
            err
        );
        std::process::exit(-1)
    });
    file.write_all(content).unwrap_or_else(|err| {
        eprintln!(
            "{}: {}",
            util::alert(&format!(
                "failed to write boilerplate code of `{}`",
                filename
            ),),
            err
        );
        std::process::exit(-1)
    });
}

const BOILERPLACE_MAIN_MM: &[u8] = br#"
rec func fib x: Int :Int {
    if x < 2 { 1 }
    else { fib (x-1) + fib (x-2) }
}

fib 8"#;

const BOILERPLACE_MUMRIK_CONF_TOML: &[u8] = br#"
[build]
src = "./src/main.mm"
output = "./build/output"
dep = "./dep/"
"#;

impl Command for NewCommand {
    fn work(self: Box<NewCommand>) {
        create_dir(&format!("./{}", self.project_name), "project");
        create_dir(&format!("./{}/src", self.project_name), "source");
        create_dir(&format!("./{}/build", self.project_name), "build");
        create_dir(&format!("./{}/dep", self.project_name), "build");
        create_file(
            &format!("./{}/src/main.mm", self.project_name),
            "main.mm",
            BOILERPLACE_MAIN_MM,
        );
        create_file(
            &format!("./{}/mumrik-conf.toml", self.project_name),
            "mumrik-conf.toml",
            BOILERPLACE_MUMRIK_CONF_TOML,
        );
    }
}
