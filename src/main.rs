mod tokenizer;
mod interpreter;
mod commands;
mod forker;
mod print_prompt;

use std::io;
use std::error::Error;
use std::io::Write;
use crate::interpreter::Interpreter;
use crate::tokenizer::tokenize;
fn main() -> Result<(), Box<dyn Error>> {
    loop {
        print_prompt::print_prompt();

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
