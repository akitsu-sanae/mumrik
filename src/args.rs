use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;

pub struct Args {
    pub input: String,
    pub output: Option<String>,
}

impl Args {
    pub fn new() -> Args {
        let mut args: VecDeque<_> = std::env::args().collect();
        args.pop_front();

        let mut input = None;
        let mut output = None;

        while let Some(arg) = args.pop_front() {
            if arg.as_str() == "-o" {
                output = Some(args.pop_front().expect("filename is required after `-o`"));
            } else if arg.as_str().starts_with("-o=") {
                output = Some(arg[3..].to_string());
            } else {
                let mut src = String::new();
                let filename = arg;
                let f = File::open(filename.clone()).and_then(|mut f| f.read_to_string(&mut src));
                if f.is_ok() {
                    input = Some(src);
                } else {
                    eprintln!("can not load file: {}", filename);
                    std::process::abort();
                }
            }
        }

        Args {
            input: input.expect("input filename is required"),
            output: output,
        }
    }
}
