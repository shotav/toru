use std::process::{Command, ExitStatus};

pub fn execute(command: String) -> ExitStatus {
    return Command::new("sh").arg("-c").arg(command).status().expect("");
}