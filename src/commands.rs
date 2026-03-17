pub mod cd_command;
pub mod pwd_command;
pub mod exit_command;

pub trait Command {
    fn execute(&self, tokens: Vec<String>) -> Result<i32, String>;
}