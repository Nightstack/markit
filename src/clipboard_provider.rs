use std::process::Command;

pub trait ClipboardProvider {
    fn set_text(&mut self, text: &str) -> Result<(), String>;
}

pub struct SmartClipboard {
    native: Option<arboard::Clipboard>,
}

impl SmartClipboard {
    pub fn new() -> Self {
        let native = arboard::Clipboard::new().ok();
        Self { native }
    }

    fn is_command_available(&self, cmd: &str) -> bool {
        Command::new("which")
            .arg(cmd)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    fn run_copy_command(&self, cmd: &str, text: &str, args: &[&str]) -> Result<(), String> {
        let mut child = Command::new(cmd)
            .args(args)
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn {}: {}", cmd, e))?;

        if let Some(stdin) = &mut child.stdin {
            use std::io::Write;
            stdin
                .write_all(text.as_bytes())
                .map_err(|e| format!("Failed to write to {} stdin: {}", cmd, e))?;
        }

        let status = child
            .wait()
            .map_err(|e| format!("Failed to wait for {}: {}", cmd, e))?;
        if status.success() {
            Ok(())
        } else {
            Err(format!("{} exited with status {}", cmd, status))
        }
    }

    fn fallback_copy(&self, text: &str) -> Result<(), String> {
        if self.is_command_available("wl-copy") {
            return self.run_copy_command("wl-copy", text, &[]);
        }

        if self.is_command_available("xclip") {
            return self.run_copy_command("xclip", text, &["-selection", "clipboard"]);
        }

        if self.is_command_available("xsel") {
            return self.run_copy_command("xsel", text, &["--clipboard", "--input"]);
        }

        Err(
            "No clipboard provider available (arboard failed and no wl-copy, xclip, or xsel)"
                .into(),
        )
    }
}

impl ClipboardProvider for SmartClipboard {
    fn set_text(&mut self, text: &str) -> Result<(), String> {
        if let Some(clipboard) = &mut self.native {
            if let Err(e) = clipboard.set_text(text.to_string()) {
                eprintln!("⚠️ arboard failed, falling back: {}", e);
                return self.fallback_copy(text);
            } else {
                return Ok(());
            }
        }

        self.fallback_copy(text)
    }
}
