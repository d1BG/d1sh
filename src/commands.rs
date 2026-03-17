pub mod cd;
pub mod pwd;
pub mod exit;
pub mod export;
pub mod r#false;
pub mod r#true;

pub trait Command {
    fn execute(&self, tokens: Vec<String>) -> Result<i32, String>;
}