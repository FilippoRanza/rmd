use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::TempDir;
/*
    Create a directory and a sub directory.
    Those directories will contain identical files.
    rmd should consider duplicates the file in the deeper
    directory.
*/
#[test]
fn remove_deeper_duplicates() {
    let temp_dir = TempDir::new().expect("cannot create a temp dir");
    let file_to_keep = fill_directory(temp_dir.path());
    let sub_dir = temp_dir.path().join("a").join("b");
    let file_to_remove = fill_directory(&sub_dir);
    let _ = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("-drcv")
        .arg(temp_dir.path().as_os_str())
        .output();    
        
    for file in file_to_keep {
        assert!(file.exists());
    }

    for file in file_to_remove {
        assert!(!file.exists(), "{}", file.to_str().unwrap());
    }

    assert!(!sub_dir.exists());

}

fn fill_directory(path: &Path) -> Vec<PathBuf> {
    if !path.exists() {
        create_dir_all(path).expect("cannot create directory");
    }
    let mut output = Vec::with_capacity(100);
    for i in 1..100 {
        let name = format!("file_{}.txt", i);
        let content = format!("clone_{}", i);
        let file_name = path.join(name);
        let mut file = File::create(&file_name).expect("cannot create temp file");
        writeln!(file, "{}", content).expect("cannot write to file");
        output.push(file_name);
    }
    output
}
