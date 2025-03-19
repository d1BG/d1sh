mod tokenizer;
mod interpreter;

use std::{env, io};
use std::io::Write;
use gethostname::gethostname;

fn main() {
    loop {
        env::set_var("HOST", gethostname().to_str().unwrap());
        print!("[{}@{}] {} d1sh> ",
               env::var("USER").unwrap(),
               env::var("HOST").unwrap(),
               env::current_dir().unwrap().display());

        io::stdout().flush().unwrap();
        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Error reading input.");
            continue;
        }

        let input = input.trim(); // remove \n char

        match interpreter::interpret(tokenizer::tokenize(input)){ _ => {} }
    }
}
