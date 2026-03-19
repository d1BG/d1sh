use std::ffi::CString;
use std::fs::File;
use std::process::exit;
use nix::sys::signal::{signal, SigHandler, Signal};
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{execvp, fork, getpgrp, setpgid, tcsetpgrp, ForkResult, Pid};

pub fn run_command(tokens: Vec<String>) -> Result<i32, String> {
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

            let c_cmd = CString::new(tokens[0].to_string()).unwrap();
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