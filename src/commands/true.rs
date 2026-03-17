use crate::commands::Command;

pub struct TrueCommand;

impl Command for TrueCommand {
    fn execute(&self, _tokens: Vec<String>) -> Result<i32, String> {
        Ok(0)
    }
}