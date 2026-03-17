use std::process::exit;
use crate::commands::Command;

pub struct ExitCommand;
impl Command for ExitCommand {
    fn execute(&self, _tokens: Vec<String>) -> Result<i32, String> {
        exit(0);
    }
}