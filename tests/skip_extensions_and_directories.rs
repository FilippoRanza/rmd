use std::fs::{create_dir, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_skip_extensions_and_directories() {
    let root = TempDir::new().unwrap();

    let mut preserve_files = make_copies(root.path(), "important_file", "bak", 10);
    let skip_dir = root.path().join(".git");
    create_dir(&skip_dir).unwrap();

    let mut tmp = make_copies(&skip_dir, "data", "txt", 10);
    preserve_files.append(&mut tmp);

    let remove_file_in_root = make_copies(root.path(), "less_important_file", "txt", 10);
    let not_skip_dir = root.path().join("sub_dir");
    create_dir(&not_skip_dir).unwrap();

    let remove_file_in_sub_dir = make_copies(&not_skip_dir, "not_important_file", "dat", 10);

    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--ignore-extensions")
        .arg("bak")
        .arg("--ignore-directories")
        .arg(".git")
        .arg("--duplicates")
        .arg(root.path().as_os_str())
        .output();

    println!("{:?}", output);

    for file in &preserve_files {
        assert!(file.exists());
    }

    only_one_file_exists(&remove_file_in_root);
    only_one_file_exists(&remove_file_in_sub_dir);
}

fn only_one_file_exists(files: &[PathBuf]) {
    let count = files
        .iter()
        .filter(|f| f.exists())
        .fold(0, |count, _| count + 1);
    assert_eq!(count, 1, "files {:?}", files);
}

fn make_copies(root: &Path, name: &str, ext: &str, count: usize) -> Vec<PathBuf> {
    let mut output = Vec::new();

    for i in 0..count {
        let file_name = format!("{}-{}.{}", name, i, ext);
        let path = root.join(file_name);
        let mut file = File::create(&path).unwrap();
        file.write(name.as_bytes()).unwrap();
        output.push(path);
    }

    output
}
