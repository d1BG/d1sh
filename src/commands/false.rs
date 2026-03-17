use std::process::ExitCode;
use crate::commands::Command;

pub struct FalseCommand;

impl Command for FalseCommand {
    fn execute(&self, _tokens: Vec<String>) -> Result<i32, String> {
        Ok(1)
    }
}