use nix::{
    sys::signal::{signal, SigHandler, Signal},
    sys::wait::{waitpid, WaitStatus},
    unistd::{fork, ForkResult, Pid, execvp, setpgid, tcsetpgrp, getpgrp},
};

use std::{
    process::exit,
    ffi::CString,
    fs::File,
};

use std::collections::HashMap;
use crate::commands::Command;
use crate::commands::cd::CdCommand;
use crate::commands::exit::ExitCommand;
use crate::commands::pwd::PwdCommand;

pub struct Interpreter {
    commands: HashMap<String, Box<dyn Command>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();

        commands.insert(String::from("cd"), Box::new(CdCommand));
        commands.insert(String::from("exit"), Box::new(ExitCommand));
        commands.insert(String::from("pwd"), Box::new(PwdCommand));

        Self { commands }
    }

    pub fn run(&self, tokens: Vec<String>) -> Result<i32, String> {
        if tokens.is_empty() {
            return Ok(0);
        }

        let cmd_name = &tokens[0];
        let args = tokens[1..].to_vec();

        match self.commands.get(cmd_name) {
            Some(command) => {
                command.execute(args).map_err(|err| err.to_string())
            }
            None => {
                match unsafe { fork() } {
                    Ok(ForkResult::Child) => {
                        let _ = setpgid(Pid::this(), Pid::this()); // Set process group

                        // Ignore SIGTTOU so we don't get stopped
                        unsafe {
                            signal(Signal::SIGTTOU, SigHandler::SigIgn).unwrap();
                        }

                        // Move child process to foreground
                        if let Ok(tty) = File::open("/dev/tty") {
                            let _ = tcsetpgrp(&tty, Pid::this());
                        }

                        let c_cmd = CString::new(tokens[0].clone()).unwrap();
                        let c_args: Vec<CString> = tokens.iter().map(|s| CString::new(s.as_str()).unwrap()).collect();

                        execvp(&c_cmd, &c_args).unwrap_or_else(|e| {
                            eprintln!("exec failed: {}", e);
                            exit(1);
                        });
                        Ok(0)
                    }
                    Ok(ForkResult::Parent { child }) => {
                        let _ = setpgid(child, child); // Ensure child is in its own process group

                        // **Ensure the shell ignores SIGTTOU too**
                        unsafe {
                            signal(Signal::SIGTTOU, SigHandler::SigIgn).unwrap();
                        }

                        // Move child process to foreground
                        if let Ok(tty) = File::open("/dev/tty") {
                            let _ = tcsetpgrp(&tty, child);
                        }

                        // Wait for process to exit
                        loop {
                            match waitpid(child, None) {
                                Ok(WaitStatus::Exited(_, _)) | Ok(WaitStatus::Signaled(_, _, _)) => break,
                                Ok(WaitStatus::Stopped(_, _)) => {
                                    eprintln!("Process {} stopped", child);
                                    break;
                                }
                                Ok(_) => continue,
                                Err(e) => {
                                    eprintln!("Error waiting for process {}: {}", child, e);
                                    break;
                                }
                            }
                        }

                        // Restore shell as foreground process
                        if let Ok(tty) = File::open("/dev/tty") {
                            let _ = tcsetpgrp(&tty, getpgrp());
                        }

                        Ok(0)
                    }
                    Err(_) => Err("Failed to fork".to_string()),
                }
            }
        }
    }
}