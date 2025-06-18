pub trait ClipboardProvider {
    fn set_text(&mut self, text: String) -> Result<(), String>;
}

impl ClipboardProvider for arboard::Clipboard {
    fn set_text(&mut self, text: String) -> Result<(), String> {
        arboard::Clipboard::set_text(self, text).map_err(|e| e.to_string())
    }
}
