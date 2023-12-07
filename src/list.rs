use crate::libs::dir_size;
use std::fs;

pub fn list_files() {
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries {
            if let Ok(entry) = entry {
                let metadata = entry.metadata();
                match metadata {
                    Ok(metadata) => {
                        if metadata.is_file() {
                            println!(
                                "File: {} ({} bytes)",
                                entry.file_name().to_string_lossy(),
                                metadata.len(),
                            );
                        } else if metadata.is_dir() {
                            let dir_size = dir_size::calculate_directory_size(&entry.path());

                            println!(
                                "Directory: {} ({} bytes)",
                                entry.file_name().to_string_lossy(),
                                dir_size
                            );
                        } else {
                            println!(
                                "Unknown: {} ({} bytes)",
                                entry.file_name().to_string_lossy(),
                                metadata.len()
                            );
                        }
                    }
                    Err(_) => {
                        println!(
                            "Error getting metadata for: {}",
                            entry.file_name().to_string_lossy()
                        );
                    }
                }
            }
        }
    } else {
        eprintln!("Error reading directory");
    }
}
