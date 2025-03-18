use std::env;
use std::process::exit;

pub(crate) fn interpret(tokens: Vec<String>) {
    println!("{:?}", tokens);

    if tokens.is_empty() {
        return;
    }

    match tokens[0].as_str() {
        "exit" => exit(0),
        "pwd" => println!("{}", env::current_dir().unwrap().display()),
        _ => {}
    }
}