// Imports
use std::{env, process::exit};
use std::fs;
use std::io::{self, Write};

// To run a file
fn run_file(path: &str) -> Result<(), String>{
    match fs::read_to_string(path){
        Ok(contents) => return run(&contents),
        Err(e) => {
            Err(format!("Error reading file: {}", e))
        }
    }
}

fn run(_source: &str) -> Result<(), String>{
    Err("Not implemented".to_string())
}

fn run_prompt() -> Result<(), std::io::Error>{
    loop{
        print!("> ");
        match io::stdout().flush(){
            Ok(_) => (),
            Err(e) => return Err(e)
        }
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;
        if buffer.trim() == "exit()"{
            break;
        }
        println!("You Wrote: {}",buffer);
    }
    Ok(())
    
}

fn main() {
    let args : Vec<String> = env::args().collect();
    let args_len = args.len();

    if args_len > 2{
        println!("Usage: jlox <script>");
        exit(64);
    } else if args_len == 2 {
        match run_file(&args[1]){
            Ok(_) => (),
            Err(e) => {
                eprintln!("{}", e);
                exit(64);
            }
        }
    } else {
        match run_prompt(){
            Ok(_) => exit(0),
            Err(msg) => {
                eprintln!("Error: {}", msg);
                exit(64);
            }
        }
    }

}
