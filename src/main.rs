use std::{fs, env};

use std::path::{Path, PathBuf};
use std::process::Command;
fn main() {

    let path = get_path();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let sub_path = entry.path().join(".git");
                if sub_path.is_dir() {
                    // Run your function here
                    fetch_all(&sub_path);
                }
            }
        }
    }
}

fn fetch_all(directoy_path: &Path) {
    let status = Command::new("git")
        .current_dir(directoy_path)
        .arg("fetch")
        .arg("--all")
        .status();

    

    match status {
        Ok(exit_status) => {
            if exit_status.success() {
                println!(
                    "fetched {:?}",
                    directoy_path
                );
            } else {
                eprintln!("Something done messed up {:?}", directoy_path);
            }
        }
        Err(e) => eprintln!("Error running git fetch --all: {:?}", e),
    }
}

fn get_path() -> PathBuf {
    let path;
    if let Ok(current_dir) = env::current_dir() {
        path = current_dir;
        println!("Current directory: {}", path.display());
    } else {
        println!("Failed to get current directory, exiting");
        std::process::exit(0); // Exit the program if user enters 'exit'
    }

    return path;
}
