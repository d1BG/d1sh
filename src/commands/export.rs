use std::env;
use crate::commands::Command;

pub struct ExportCommand;

impl Command for ExportCommand {
    fn execute(&self, _tokens: Vec<String>) -> Result<i32, String> {
        Ok(0)
    }
}