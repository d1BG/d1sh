use std::collections::HashMap;
use crate::commands::Command;
use crate::commands::cd::CdCommand;
use crate::commands::exit::ExitCommand;
use crate::commands::pwd::PwdCommand;
use crate::commands::r#false::FalseCommand;
use crate::commands::r#true::TrueCommand;
use crate::commands::which::WhichCommand;
use crate::forker;

pub struct Interpreter {
    commands: HashMap<String, Box<dyn Command>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();

        commands.insert(String::from("cd"), Box::new(CdCommand));
        commands.insert(String::from("exit"), Box::new(ExitCommand));
        commands.insert(String::from("pwd"), Box::new(PwdCommand));
        commands.insert(String::from("false"), Box::new(FalseCommand));
        commands.insert(String::from("true"), Box::new(TrueCommand));
        commands.insert(String::from("which"), Box::new(WhichCommand));

        Self { commands }
    }

    pub fn run(&self, tokens: Vec<String>) -> Result<i32, String> {
        if tokens.is_empty() {
            return Ok(0);
        }

        match self.commands.get(tokens[0].as_str()) {
            Some(command) => {
                // internal command
                command.execute(tokens[1..].to_owned()).map_err(|err| err.to_string())
            }
            None => {
                // search path and execute
                forker::run_command(tokens)
            }
        }
    }
}