use std::path::PathBuf;
use search_path::SearchPath;
use crate::commands::Command;

pub struct WhichCommand;

impl Command for WhichCommand {
    fn execute(&self, tokens: Vec<String>) -> Result<i32, String> {
        let search_path = SearchPath::new("PATH")
            .expect("How do you live with no $PATH?");

        for i in 0..tokens.len() {
            match search_path.find_file(&PathBuf::from(tokens[i].to_owned())) {
                Some(path) => {
                    println!("{}", path.display());
                }
                None => {
                    eprintln!("{} not found", tokens[i]);
                }
            }
        }
        Ok(0)
    }
}