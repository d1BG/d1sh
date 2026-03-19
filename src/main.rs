mod tokenizer;
mod interpreter;
mod commands;

use std::{env, io};
use std::error::Error;
use std::io::Write;
use whoami::{hostname, username};
use crate::interpreter::Interpreter;
use crate::tokenizer::tokenize;
fn main() -> Result<(), Box<dyn Error>> {
    let hostname = hostname().unwrap_or_else(|_| "hostname".to_string());
    let user = username().unwrap_or_else(|_| "username".to_string());

    loop {
        print!("[{user}@{hostname}] {} d1sh> ",
                env::current_dir()?.display());

        io::stdout().flush()?;
        let mut input = String::new();

        if let Ok(bytes_read) = io::stdin().read_line(&mut input) {
            if bytes_read == 0 {
                println!();
                break;
            }
        }
        else {
            eprintln!("Error reading input.");
            continue;
        }

        let input = input.trim(); // remove \n char
        let interpreter = Interpreter::new();
        let tokenized = tokenize(input);
        
        println!("{:?}", tokenized);

        match interpreter.run(tokenized){
            Ok(_result) => {}
            Err(err) => {
                eprintln!("{}", err);
            }
        }

    }
    Ok(())
}
