use std::env::{ current_dir, home_dir };
use whoami::{ hostname, username };

pub fn print_prompt() {
    let hostname = hostname().unwrap_or_else(|_| "hostname".to_string());
    let user = username().unwrap_or_else(|_| "username".to_string());
    let current_dir = current_dir().unwrap();
    let home_dir = home_dir().unwrap();
    let mut display_dir: String = current_dir.display().to_string();

    if current_dir.starts_with(home_dir.clone()) {
        display_dir = current_dir.to_str()
            .unwrap()
            .replace(home_dir.to_str().unwrap(), "~")
    }

    print!("[{user}@{hostname}] {} d1sh> ", display_dir);
}