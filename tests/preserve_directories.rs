

use std::io::Write;
use std::fs::{create_dir, File};
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

    let output = Command::new("cargo")
    .arg("run")
    .arg("--")
    .arg("--ignore-directories")
    .arg("important_stuff")
    .arg(".git")
    .arg("--smaller")
    .arg(format!("{}b", small))
    .arg("--")
    .arg(root.path().as_os_str())
    .output();


    for file in &remove_files {
        assert!(!file.exists());
    }

    for file in &preserve_files {
        assert!(file.exists());
    }



}

fn make_sub_dir(root: &Path, dir_name: &str, file_names: &[&str], file_size: usize) -> Vec<PathBuf> {
    let mut output = Vec::new();
    let data: Vec<u8> = (0..file_size).map(|x| 0).collect();
    let dir_path = root.join(dir_name);
    if !dir_path.is_dir() {
        create_dir(&dir_path).unwrap();
    }
    
    for file in file_names{
        let file = format!("{}-{}", file, file_size);
        let path = dir_path.join(file);
        let mut file = File::create(&path).unwrap();
        file.write(&data).unwrap();
        output.push(path);
    }

    output
}

