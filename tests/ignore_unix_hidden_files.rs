use std::fs::{create_dir, File};
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_ignore_unix_hidden_files() {
    let temp_root = TempDir::new().unwrap();
    let (hidden, visible) = make_dir_tree(temp_root.path());

    let _ = Command::new("cargo")
        .arg("new")
        .arg("--")
        .arg("--smaller")
        .arg("100")
        .arg("--ignore-unix-hidden")
        .arg(temp_root.path().as_os_str())
        .output();

    for file in &visible {
        assert!(!file.exists());
    }

    for file in &hidden {
        assert!(file.exists());
    }

}

fn make_dir_tree(root: &Path) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let dir_name = ["dir_a", "dir_b", "dir_c", "dir_d"];
    let file_name = ["file_a", "file_b", "file_c", "file_d"];

    let mut index = 0;

    let mut output_hidden = Vec::new();
    let mut output_visible = Vec::new();

    for dir in &dir_name {
        let curr_root = root.join(dir);
        create_dir(&curr_root).unwrap();
        for file in &file_name {
            if index % 3 == 0 {
                let name = format!(".{}", file);
                let path = curr_root.join(name);
                File::create(&path).unwrap();
                output_hidden.push(path);
            } else {
                let path = curr_root.join(file);
                File::create(&path).unwrap();
                output_visible.push(path);
            }
        }
    }

    (output_hidden, output_visible)
}
