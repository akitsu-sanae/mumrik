use std::collections::VecDeque;

pub struct Args {
    pub is_interp: bool,
    pub input_filenames: Vec<String>,
    pub output_filename: String,
}

fn print_usage(program_name: &str) {
    println!(
        r#"mumrik : a programming language
USAGE: {} <filename...> [option...]
options are:
    --output, -o <file>  Write output tp <file>, default: `a.out`
    --interp, -i         Run mumrik interpreter, `--output` flag will be ignored
    --help, -h           Display this"#,
        program_name
    );
}

impl Args {
    pub fn new() -> Args {
        let mut args: VecDeque<_> = std::env::args().collect();
        let program_name = args.pop_front().unwrap();

        let mut is_interp = false;
        let mut input_filenames = vec![];
        let mut output_filename = "./a.out".to_string();

        while let Some(arg) = args.pop_front() {
            if arg.as_str() == "--help" || arg.as_str() == "-h" {
                print_usage(program_name.as_str());
                std::process::exit(0);
            } else if arg.as_str() == "--interp" || arg.as_str() == "-i" {
                is_interp = true;
            } else if arg.as_str() == "--output" || arg.as_str() == "-o" {
                output_filename = args
                    .pop_front()
                    .expect("filename is required after `--output` or -o`");
            } else if arg.as_str().starts_with("--output=") {
                output_filename = arg[9..].to_string();
            } else if arg.as_str().starts_with("-o=") {
                output_filename = arg[3..].to_string();
            } else {
                input_filenames.push(arg);
            }
        }

        if input_filenames.is_empty() {
            eprintln!("input filenames are required");
            std::process::exit(-1);
        }

        Args {
            is_interp: is_interp,
            input_filenames: input_filenames,
            output_filename: output_filename,
        }
    }
}
