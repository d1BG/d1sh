mod tokenizer;
mod interpreter;

use std::io;
use std::io::Write;

fn main() {
    loop {
        print!("d1sh> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Error reading input.");
            continue;
        }

        let input = input.trim(); // remove \n char

        interpreter::interpret(tokenizer::tokenize(input));
    }
}
