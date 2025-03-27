mod tokenizer;
mod interpreter;

use std::{env, io};
use std::io::Write;
use gethostname::gethostname;

fn main() {
    let hostname = gethostname().to_str().unwrap().to_string();
    let user = env::var("USER") // Unix-like systems
        .or_else(|_| env::var("USERNAME")) // Windows
        .unwrap_or_else(|_| "d1sh".to_string());

    env::set_var("HOST", hostname.clone());

    loop {
        print!("[{user}@{hostname}] {} d1sh> ",
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
