use crate::storage::Storage;
use crate::ui::SelectionUI;

pub fn restore_command(storage: &dyn Storage, selection_ui: &dyn SelectionUI) {
    let backups = match storage.get_backups() {
        Ok(s) => s,
        Err(_) => {
            println!("📭 No backups created yet.");
            return;
        }
    };

    if backups.is_empty() {
        println!("📭 No backups found.");
        return;
    }

    let display_names: Vec<String> = backups
        .iter()
        .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
        .collect();

    let selected_index = match selection_ui.with_backup_list(&display_names) {
        Some(i) => i,
        None => return,
    };

    let full_path = backups.get(selected_index).unwrap().clone();

    match storage.restore_backup(&full_path) {
        Ok(_) => println!("✅ Backup restored successfully."),
        Err(e) => eprintln!("⛔ Failed to restore backup: {}", e),
    };
}
