use std::fs::{create_dir, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_preserve_single_directory() {
    let root = TempDir::new().unwrap();

    let small = 10;
    let large = 100;

    let remove_dir_name = ["test", "dir", "useless_stuff"];
    let preserve_dir_name = [".git", "important_stuff"];
    let empty_dirs_remove_name = ["empty_delete", "empty_remove"];
    let empty_dirs_preserve_name = ["empty_save", "empty_preserve"];

    let files = ["file", "test", "stuff", "data", "photo"];

    let mut remove_files = Vec::new();
    let mut preserve_files = Vec::new();

    for rm in &remove_dir_name {
        let mut tmp = make_sub_dir(root.path(), rm, &files, small);
        remove_files.append(&mut tmp);

        let mut tmp = make_sub_dir(root.path(), rm, &files, large);
        preserve_files.append(&mut tmp);
    }

    for pr in &preserve_dir_name {
        let mut tmp = make_sub_dir(root.path(), pr, &files, small);
        preserve_files.append(&mut tmp);

        let mut tmp = make_sub_dir(root.path(), pr, &files, large);
        preserve_files.append(&mut tmp)
    }

    create_empty_dirs(root.path(), &empty_dirs_preserve_name);
    create_empty_dirs(root.path(), &empty_dirs_remove_name);

    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("-c")
        .arg("-v")
        .arg("--ignore-directories")
        .arg("important_stuff")
        .arg(".git")
        .arg("empty_save")
        .arg("empty_preserve")
        .arg("--smaller")
        .arg(format!("{}b", small))
        .arg("--")
        .arg(root.path().as_os_str())
        .output();

    println!("{:?}", output);

    exist_dirs(root.path(), &remove_dir_name, true);
    exist_dirs(root.path(), &preserve_dir_name, true);
    exist_dirs(root.path(), &empty_dirs_remove_name, false);
    exist_dirs(root.path(), &empty_dirs_preserve_name, true);

    for file in &remove_files {
        assert!(!file.exists());
    }

    for file in &preserve_files {
        assert!(file.exists());
    }

    for dir in &preserve_dir_name {
        let path = root.path().join(dir);
        assert!(path.exists());
    }
}

fn make_sub_dir(
    root: &Path,
    dir_name: &str,
    file_names: &[&str],
    file_size: usize,
) -> Vec<PathBuf> {
    let mut output = Vec::new();
    let data: Vec<u8> = (0..file_size).map(|_| 0).collect();
    let dir_path = root.join(dir_name);
    if !dir_path.is_dir() {
        create_dir(&dir_path).unwrap();
    }

    for file in file_names {
        let file = format!("{}-{}", file, file_size);
        let path = dir_path.join(file);
        let mut file = File::create(&path).unwrap();
        file.write(&data).unwrap();
        output.push(path);
    }

    output
}

fn create_empty_dirs(root: &Path, names: &[&str]) {
    for name in names {
        let path = root.join(name);
        create_dir(path).unwrap();
    }
}

fn exist_dirs(root: &Path, names: &[&str], exist: bool) {
    for name in names {
        let path = root.join(name);
        assert_eq!(path.is_dir(), exist);
    }
}
