use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tempfile::TempDir;

#[test]
fn test_interactive_mode() {
    let temp_root = TempDir::new().unwrap();
    let (small, large) = build_files(temp_root.path(), "file", 102, 10, 150);

    let mut process = Command::new("cargo")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg("run")
        .arg("--")
        .arg("--smaller")
        .arg("100b")
        .arg("-i")
        .arg(temp_root.path().as_os_str())
        .spawn()
        .unwrap();

    let process_stdin = process.stdin.as_mut().unwrap();
    process_stdin
        .write_all(&remove_answer(small.len(), "n\n"))
        .unwrap();

    let status = process.wait_with_output().unwrap();

    assert!(
        status.status.success(),
        "{}\n{}",
        String::from_utf8(status.stdout).unwrap(),
        String::from_utf8(status.stderr).unwrap()
    );

    for file in &large {
        assert!(file.exists());
    }

    for file in &small {
        assert!(file.exists());
    }

    let output = String::from_utf8(status.stdout).unwrap();
    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines.len(), small.len());
}

fn remove_answer(count: usize, ans: &str) -> Vec<u8> {
    (0..count)
        .map(|_| ans)
        .map(|s| s.as_bytes().to_owned())
        .flatten()
        .collect()
}

fn build_files(
    root: &Path,
    prefix: &str,
    count: usize,
    small_size: usize,
    large_size: usize,
) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let mut small = Vec::with_capacity(count * (1 / 3));
    let mut large = Vec::with_capacity(count * (2 / 3));

    for i in 0..count {
        let name = format!("{}-{}.dat", prefix, i);
        let path = root.join(name);
        if i % 3 == 0 {
            small.push(create_file(path, small_size));
        } else {
            large.push(create_file(path, large_size));
        }
    }

    (small, large)
}

fn create_file(path: PathBuf, size: usize) -> PathBuf {
    let mut file = File::create(&path).unwrap();
    for _ in 0..size {
        file.write(&[0]).unwrap();
    }
    path
}
