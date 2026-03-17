use std::env;
use crate::commands::Command;

pub struct CdCommand;

impl Command for CdCommand {
    fn execute(&self, tokens: Vec<String>) -> Result<i32, String> {
        match tokens.len() {
            0 => {
                let home = env::var("HOME").unwrap_or_else(|_| "/".to_string());
                env::set_current_dir(home).map(|_| 0).map_err(|e| e.to_string())
            }
            1 => env::set_current_dir(&tokens[0]).map(|_| 0).map_err(|e| e.to_string()),
            _ => Err("Invalid arguments!".to_string()),
        }
    }
}