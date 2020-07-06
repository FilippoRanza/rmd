use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::TempDir;

/**
 * This test create some files
 * with varios extensions.
 * Some of this files will be larger then
 * a certein value and will be remover by rmd
 * except for those files with a specific extension
 */

#[test]
fn preserve_by_extension() {
    let temp_root = TempDir::new().unwrap();
    let names = ["test", "control", "file", "name", "photo", "data"];
    let extensions = ["txt", "jpg", "mp4", "exe", "csv"];
    let large_size = 100;
    let small_size = 10;
    let save_files = make_files(temp_root.path(), &names, &extensions, small_size);
    let remove_files = make_files(temp_root.path(), &names, &extensions, large_size);
    let large_save_files = make_files(temp_root.path(), &names, &["rs", "sh", "toml"], large_size);

    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--ignore-extensions")
        .arg("rs")
        .arg("sh")
        .arg("toml")
        .arg("--larger")
        .arg(format!("{}b", large_size))
        .arg("--")
        .arg(temp_root.path().as_os_str())
        .output();

    println!("{:?}", output);

    for file in &save_files {
        assert!(file.exists());
    }

    for file in &remove_files {
        assert!(!file.exists());
    }

    for file in &large_save_files {
        assert!(file.exists());
    }
}

#[test]
fn preserve_by_extension_in_sub_directories() {
    let temp_root = TempDir::new().unwrap();
    let sub_dirs = make_sub_dirs(&temp_root.path(), 5, 3);

    let names = ["test", "control", "file", "name", "photo", "data"];

    let extensions = ["txt", "jpg", "mp4", "exe", "csv"];
    let save_extensions = ["rs", "sh", "toml"];

    let large = 100;
    let small = 10;

    let mut non_existing_files = Vec::new();
    let mut existing_files = Vec::new();

    for sub in &sub_dirs {
        let mut tmp = make_files(sub, &names, &extensions, small);
        existing_files.append(&mut tmp);

        let mut tmp = make_files(sub, &names, &save_extensions, large);
        existing_files.append(&mut tmp);

        let mut tmp = make_files(sub, &names, &extensions, large);
        non_existing_files.append(&mut tmp);
    }

    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--ignore-extensions")
        .arg("rs")
        .arg("sh")
        .arg("toml")
        .arg("--larger")
        .arg(format!("{}b", large))
        .arg("--")
        .arg(temp_root.path().as_os_str())
        .output();

    println!("{:?}", output);

    for sub in &sub_dirs {
        assert!(sub.exists());
    }

    for file in &non_existing_files {
        assert!(!file.exists());
    }

    for file in &existing_files {
        assert!(file.exists());
    }
}




fn make_sub_dirs(root: &Path, count: i32, depth: i32) -> Vec<PathBuf> {
    let mut output = Vec::new();
    for i in 0..count {
        let mut curr_path = root.to_path_buf();
        for d in 0..depth {
            curr_path = curr_path.join(&format!("dir_{}-{}", i, d));
        }
        create_dir_all(&curr_path).unwrap();
        output.push(curr_path);
    }

    output
}

fn make_files(root: &Path, names: &[&str], extensions: &[&str], size: usize) -> Vec<PathBuf> {
    let data: Vec<u8> = (0..size).map(|_| 0).collect();
    let mut output = Vec::with_capacity(names.len() * extensions.len());
    for ext in extensions {
        for name in names {
            let file_name = format!("{}-{}.{}", name, size, ext);
            let file_path = root.join(file_name);
            let mut file = File::create(&file_path).unwrap();
            file.write(&data).unwrap();
            output.push(file_path);
        }
    }
    output
}

#[test]
fn test_skip_backup_files() {
    let root = TempDir::new().unwrap();
    let names = ["test", "file", "important", "stuff", "data"];
    let files = make_duplicates(root.path(), &names);

    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--ignore-extensions")
        .arg("bak")
        .arg("--duplicates")
        .arg(root.path().as_os_str())
        .output();

    println!("{:?}", output);

    for file in &files {
        assert!(file.exists());
    }

}


fn make_duplicates(root: &Path, names: &[&str]) -> Vec<PathBuf> {
    let mut output = Vec::new();

    for name in names {
        let tmp = make_file(root, format!("{}.txt", name));
        output.push(tmp);

        let tmp = make_file(root, format!("{}.txt.bak", name));
        output.push(tmp);
    }

    output
}

fn make_file(root: &Path, name: String) -> PathBuf {
    let file_path = root.join(&name);
    let mut file = File::create(&file_path).unwrap();
    file.write(name.as_bytes()).unwrap();
    file_path
}

