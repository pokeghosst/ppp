mod ast;
mod interpreter;
mod parser;

use crate::interpreter::Interpreter;
use crate::parser::parse_statements;
use std::env;
use std::fs;
use std::process;

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the file argument is passed
    if args.len() < 2 {
        eprintln!("Usage: {} <source_file>", args[0]);
        process::exit(1);
    }

    // Get the filename from the arguments
    let filename = &args[1];

    // Read the file content
    let source_code = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file {}: {}", filename, err);
        process::exit(1);
    });

    // Parse the source code
    match parse_statements(&source_code) {
        Ok((_remaining, statements)) => {
            let mut interpreter = Interpreter::new();
            interpreter.eval_statements(statements);
        }
        Err(e) => {
            eprintln!("Error parsing input: {:?}", e);
            process::exit(1);
        }
    }
}
