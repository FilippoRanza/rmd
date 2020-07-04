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
    .arg("--").arg("--ignore-extensions").arg("rs").arg("sh").arg("toml")
    .arg("--larger").arg(format!("{}b", large_size)).arg("--")
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


