use std::collections::VecDeque;

pub struct Args {
    pub input_filename: String,
    pub output_filename: String,
}

fn print_usage(program_name: &str) {
    println!(
        r#"mumrik : a programming language
USAGE: {} <filename...> [option...]
options are:
    --output, -o <file>  Write output tp <file>, default: `a.out`
    --help, -h           Display this"#,
        program_name
    );
}

impl Args {
    pub fn new() -> Args {
        let mut args: VecDeque<_> = std::env::args().collect();
        let program_name = args.pop_front().unwrap();

        let mut input_filename = None;
        let mut output_filename = "./a.out".to_string();

        while let Some(arg) = args.pop_front() {
            if arg.as_str() == "--help" || arg.as_str() == "-h" {
                print_usage(program_name.as_str());
                std::process::exit(0);
            } else if arg.as_str() == "--output" || arg.as_str() == "-o" {
                output_filename = args
                    .pop_front()
                    .expect("filename is required after `--output` or -o`");
            } else if arg.as_str().starts_with("--output=") {
                output_filename = arg[9..].to_string();
            } else if arg.as_str().starts_with("-o=") {
                output_filename = arg[3..].to_string();
            } else if input_filename.is_some() {
                print_usage(program_name.as_str());
                std::process::exit(-1);
            } else {
                input_filename = Some(arg);
            }
        }

        let input_filename = input_filename.unwrap_or("main.mm".to_string());
        Args {
            input_filename: input_filename,
            output_filename: output_filename,
        }
    }
}
