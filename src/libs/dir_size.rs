use std::{fs, path::Path};

pub fn calculate_directory_size(path: &Path) -> u64 {
    let mut total_size = 0;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.metadata().map(|m| m.is_file()).unwrap_or(false) {
                    total_size += entry.metadata().map(|m| m.len()).unwrap_or(0);
                } else if entry.metadata().map(|m| m.is_dir()).unwrap_or(false) {
                    total_size += calculate_directory_size(&entry.path());
                }
            }
        }
    }
    total_size
}
