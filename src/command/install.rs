use command::Command;
use config;
use std::collections::VecDeque;
use util;

pub struct InstallCommand {
    pub repo_name: String,
}

fn print_help(program_name: &str) {
    println!(
        r#"mumrik-install : download git repository from github for the local mumrik project
USAGE: {} install <repository-name>

repository-name: target github repository name in the form of `user-name/project-name`"#,
        program_name
    );
    std::process::exit(0);
}

impl InstallCommand {
    pub fn parse(program_name: String, mut args: VecDeque<String>) -> Box<dyn Command> {
        let mut repo_name = None;
        while let Some(arg) = args.pop_front() {
            if arg.as_str() == "--help" || arg.as_str() == "-h" {
                print_help(&program_name);
            } else if repo_name.is_some() {
                panic!(
                    "{}: too many command line argument {}",
                    util::alert("error"),
                    arg
                );
            } else {
                repo_name = Some(arg);
            }
        }
        box InstallCommand {
            repo_name: repo_name
                .unwrap_or_else(|| panic!("{}: repository name is required", util::alert("error"))),
        }
    }
}

impl Command for InstallCommand {
    fn work(self: Box<InstallCommand>) {
        let mut dep_dir = config::config_path()
            .unwrap_or_else(|| panic!("{}: local mumrik project not found", util::alert("error")));
        dep_dir.pop();
        dep_dir.push(config::CONFIG.lock().unwrap().build.dep.clone());
        println!("{}", dep_dir.as_path().to_str().unwrap());
        std::env::set_current_dir(dep_dir.as_path()).unwrap();

        let result = std::process::Command::new("git")
            .arg("clone")
            .arg(format!("https://github.com/{}", self.repo_name))
            .output()
            .unwrap_or_else(|_| panic!("failed to execute `git clone`"));

        if !result.status.success() {
            std::process::exit(-1);
        }
    }
}
