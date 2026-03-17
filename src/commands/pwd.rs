use std::env;
use crate::commands::Command;

pub struct PwdCommand;
impl Command for PwdCommand {
    fn execute(&self, _tokens: Vec<String>) -> Result<i32, String> {
        println!("{}", env::current_dir().unwrap().display());
        Ok(0)
    }
}