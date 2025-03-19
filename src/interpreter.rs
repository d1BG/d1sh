use std::env;
use std::process::{exit, Command};

pub(crate) fn interpret(tokens: Vec<String>) {
    println!("{:?}", tokens);

    if tokens.is_empty() {
        return;
    }

    match tokens[0].as_str() {
        "exit" => exit(0),
        "pwd" => println!("{}", env::current_dir().unwrap().display()),
        "cd" => {
            match tokens.len() {
                1 => env::set_current_dir(env::var("HOME").unwrap()).unwrap(),
                2 => env::set_current_dir(&*tokens[1]).unwrap(),
                _ => println!("Invalid arguments!")
            }
        },
        _ => {
            let com = Command::new(&tokens[0])
                .args(tokens[1..].to_vec())
                .output()
                .expect(format!("Failed to execute command: {}", tokens[0]).as_str());

            if com.status.success() {
                println!("{}", String::from_utf8_lossy(&com.stdout));
            } else {
                println!("{}\n{}",com.status ,String::from_utf8_lossy(&com.stderr));
            }
        }
    }
}