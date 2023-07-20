use std::{
    env::args,
    io::{self, BufRead},
};

use error::*;
use scanner::*;

mod error;
mod scanner;
mod token;
mod token_type;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        println!("Usage: lox-ast [script]");
        std::process::exit(64);
    } else if args.len() == 1 {
        run_file(&args[1]).expect("Could not run file");
    } else {
        run_prompt();
    }
}

fn run_file(path: &String) -> io::Result<()> {
    let buf = std::fs::read_to_string(path).expect("cannot read this file");
    match run(buf) {
        Ok(_) => {}
        Err(m) => {
            m.report("".to_string());
            std::process::exit(65)
        }
    };
    Ok(())
}

fn run_prompt() {
    let stdin = io::stdin();
    print!("> ");
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            match run(line) {
                Ok(_) => {}
                Err(m) => m.report("".to_string()),
            };
        } else {
            break;
        }
    }
}

fn run(source: String) -> Result<(), LoxError> {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}
