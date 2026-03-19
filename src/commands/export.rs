use std::env;
use crate::commands::Command;

pub struct ExportCommand;

impl Command for ExportCommand {
    fn execute(&self, tokens: Vec<String>) -> Result<i32, String> {
        match tokens.len() {
            0 => {
                for (key, value) in env::vars() {
                    println!("{key}={value}");
                }
                Ok(0)
            }
            1.. => {
                for i in 0..tokens.len() {
                    let arg = parse_argument(tokens[i].to_owned());
                    match arg {
                        Err(e) => return Err(e),
                        Ok((key, value)) => {
                            env::set_var(key, value)
                        }
                    }
                }
                Ok(0)
            }
        }

    }


}

fn parse_argument(arg: String) -> Result<(String, String), String> {
    let key_value = arg.split("=").map(|s| s.to_string()).collect::<Vec<String>>();
    let key = key_value.get(0).map(|s| s.as_str()).unwrap_or("").to_string();
    let value = key_value.get(1).map(|s| s.as_str()).unwrap_or("").to_string();

    if (key.is_empty() || value.is_empty() || key.starts_with(|c: char| c.is_ascii_digit())) {
        return Err("invalid argument".to_string());
    }


    for c in key.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '_' => continue,
            _ => return Err("Invalid character used".to_string())
        }
    }

    Ok((key, value))
}