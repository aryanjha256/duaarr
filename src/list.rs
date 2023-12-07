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
                            println!(
                                "Directory: {} ({} bytes)",
                                entry.file_name().to_string_lossy(),
                                metadata.len()
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
