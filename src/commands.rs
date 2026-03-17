pub mod cd;
pub mod pwd;
pub mod exit;

pub trait Command {
    fn execute(&self, tokens: Vec<String>) -> Result<i32, String>;
}