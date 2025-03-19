use std::env;
use std::process::{exit, Command};

pub(crate) fn interpret(tokens: Vec<String>) -> Result<i32, String> {
    println!("{:?}", tokens);

    if tokens.is_empty() {
        return Ok(0);
    }

    match tokens[0].as_str() {
        "exit" => exit(0),
        "pwd" => {
            println!("{}", env::current_dir().unwrap().display());
            Ok(0)
        },
        "cd" => {
            match tokens.len() {
                1 => {
                    let home = env::var("HOME").unwrap_or_else(|_| "how did u get here".to_string());
                    match env::set_current_dir(home) {
                        Ok(_) => Ok(0),
                        Err(e) => Err(format!("{}", e))
                    }
                },
                2 => {
                   match env::set_current_dir(&*tokens[1]) {
                       Ok(_) => Ok(0),
                       Err(e) => Err(format!("{}", e))
                   }
                },
                _ => {
                    Err("Invalid arguments!".to_string())
                }
            }
        },
        _ => {
            let mut cmd = Command::new(&tokens[0])
                .args(tokens[1..].to_vec()).output();
            match cmd {
                Ok(output) => {cmd = Ok(output)}
                Err(e) => {
                    println!("{}", e);
                    return Err(format!("{}", e));
                }
            }

            print!("{}", String::from_utf8_lossy(cmd.unwrap().stdout.as_slice()));
            Ok(0)
        }
    }
}