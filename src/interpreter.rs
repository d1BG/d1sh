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
        "cd" => {
            if tokens.len() == 2 {
                env::set_current_dir(&*tokens[1]).unwrap();
            } else if tokens.len() == 1 {
                env::set_current_dir("/").unwrap();
            } else if tokens.len() > 2 {
                println!("Invalid arguments!");
            }
        },
        _ => {}
    }
}