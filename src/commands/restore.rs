use crate::storage::{get_backup_files, restore_backup};
use crate::ui::select_backup;

pub fn restore_command() {
    let backups = get_backup_files();
    if backups.is_empty() {
        println!("📭 No backups found.");
        return;
    }

    let display_names: Vec<String> = backups
        .iter()
        .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
        .collect();

    let selected_index = match select_backup(&display_names) {
        Some(i) => i,
        None => return,
    };

    let full_path = backups.get(selected_index).unwrap().clone();
    restore_backup(full_path);
}
