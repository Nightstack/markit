pub trait CommandRunner {
    fn run(&self, command: &str) -> Result<std::process::ExitStatus, std::io::Error>;
}

pub struct ShellCommandRunner;

impl CommandRunner for ShellCommandRunner {
    fn run(&self, command: &str) -> Result<std::process::ExitStatus, std::io::Error> {
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".into());
        std::process::Command::new(shell)
            .arg("-c")
            .arg(command)
            .status()
    }
}
