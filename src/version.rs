use std::{fs, path::PathBuf};

fn list_all_changelogs() -> Vec<PathBuf> {
    let mut changelogs: Vec<PathBuf> = vec![];
    match fs::read_dir(".workspyce/") {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let pt = entry.path();
                        let ext = pt.extension().expect("should be able to get an extension for the file");
                        if ext == "md" {
                            changelogs.push(pt);
                        }
                    },
                    Err(e) => eprintln!("Error while reading the `.workspyce` directory: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error while reading the `.workspyce` directory:: {}", e),
    }
    changelogs
}

// Should take a look at hashmaps for this: https://doc.rust-lang.org/rust-by-example/std/hash.html
fn get_all_version_bumps() {}