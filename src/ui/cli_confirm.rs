use dialoguer::Confirm;

use crate::ui::ConfirmPrompt;

pub struct DialoguerConfirm;

impl ConfirmPrompt for DialoguerConfirm {
    fn confirm(&self, message: &str) -> bool {
        Confirm::new()
            .with_prompt(message)
            .default(false)
            .interact()
            .unwrap_or(false)
    }
}
