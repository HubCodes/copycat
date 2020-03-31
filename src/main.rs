use std::env;
use std::fs;
use std::io::{stdin, stdout, Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let args: &[String] = &args[1..];
    match args {
        [] => from_stdin(),
        _ => from_args(args),
    }
}

fn from_args(args: &[String]) {
    for arg in args {
        let contents = fs::read(&arg)
            .expect(format!("An error occurred while reading filename {}", &arg).as_str());
        stdout()
            .write(&contents)
            .expect(format!("An error occurred while writing filename {}", &arg).as_str());
    }
}

fn from_stdin() {
    let mut contents: Vec<u8> = Vec::new();
    stdin().read(&mut contents).expect("Cannot read from stdin");
    stdout().write(&contents).expect("Cannot write to stdout");
}
