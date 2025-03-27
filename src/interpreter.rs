use nix::{
    sys::signal::{signal, SigHandler, Signal},
    sys::wait::{waitpid, WaitStatus},
    unistd::{fork, ForkResult, Pid, execvp, setpgid, tcsetpgrp, getpgrp},
};
use std::{
    env,
    process::exit,
    ffi::CString,
    fs::File,
};

pub(crate) fn interpret(tokens: Vec<String>) -> Result<i32, String> {
    println!("{:?}", tokens);

    if tokens.is_empty() {
        return Ok(0);
    }

    match tokens[0].as_str() {
        "exit" => exit(0),

        "pwd" => {
            println!("{}", env::current_dir().unwrap().display());
            Ok(0)
        }

        "cd" => match tokens.len() {
            1 => {
                let home = env::var("HOME").unwrap_or_else(|_| "/".to_string());
                env::set_current_dir(home).map(|_| 0).map_err(|e| e.to_string())
            }
            2 => env::set_current_dir(&tokens[1]).map(|_| 0).map_err(|e| e.to_string()),
            _ => Err("Invalid arguments!".to_string()),
        },

        _ => {
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
