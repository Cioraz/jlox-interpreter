mod scanner;
mod token;
use crate::scanner::*;

// Imports
use std::fs;
use std::{env, process::exit};

// To run a file
fn run_file(path: &str) -> Result<(), String> {
    match fs::read_to_string(path) {
        Ok(contents) => return run(&contents),
        Err(e) => Err(format!("Error reading file: {}", e)),
    }
}

// To run the source code of the jlox file
fn run(source: &str) -> Result<(), String> {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}

// To run the prompt
fn run_prompt() -> Result<(), String> {
    // To ensure we can use the history feature + edit line
    let mut rl = rustyline::DefaultEditor::new().expect("Error creating editor");
    loop {
        let readline = rl.readline("jlox> ");
        match readline {
            Ok(line) => {
                if line.trim() == "exit()" {
                    break;
                }
                rl.add_history_entry(line.as_str())
                    .expect("Error adding history entry.");
                println!("ECHO: {}", line);
                match run(&line) {
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("ERROR: {}", e);
                    }
                }
            }
            Err(_) => break,
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();

    if args_len > 2 {
        println!("Usage: jlox <script>");
        exit(64);
    } else if args_len == 2 {
        // Run the file
        match run_file(&args[1]) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("ERROR: {}", e);
                exit(1);
            }
        }
    } else {
        // Run the prompt
        match run_prompt() {
            Ok(_) => exit(0),
            Err(msg) => {
                eprintln!("ERROR: {}", msg);
                exit(1);
            }
        }
    }
}
