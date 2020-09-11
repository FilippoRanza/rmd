use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use tempfile::TempDir;

#[test]
fn test_all_positive_answer() {
    run_test(1000, 1, |i| match i % 6 {
        0 => "yes\n",
        1 => "Yes\n",
        2 => "Y\n",
        3 => "ye\n",
        4 => "YE\n",
        5 => "y\n",
        _ => unreachable!(),
    });
}

#[test]
fn test_all_negative_answer() {
    run_test(1000, 1000, |i| match i % 10 {
        0 => "n\n",
        1 => "no\n",
        2 => "No\n",
        3 => "nO\n",
        4 => "N\n",
        5 => "NO\n",
        6 => "YESS\n",
        7 => "yres\n",
        8 => "qwe\n",
        9 => "333\n",
        _ => unreachable!()
    });
}


#[test]
fn test_half_positive_half_negative() {
    run_test(1000, 500, |i| if i % 2 == 0 {"y\n"} else {"no\n"});
}



fn run_test<F>(file_count: usize, expected_count: usize, callback: F)
where
    F: Fn(usize) -> &'static str,
{
    let temp_root = TempDir::new().unwrap();
    let files = create_duplicates(temp_root.path(), file_count);

    let ans: Vec<u8> = (0..file_count)
        .map(callback)
        .map(|s| s.as_bytes().to_owned())
        .flatten()
        .collect();

    let output = run_remove_duplicates(temp_root.path(), &ans);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(output.status.success(), "{}\n{}", stdout, String::from_utf8(output.stderr).unwrap());
    

    let count = files.iter().filter(|f| f.exists()).count();
    assert_eq!(count, expected_count);

    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), file_count - 1);

}

fn run_remove_duplicates(dir: &Path, input: &[u8]) -> Output {
    let mut process = Command::new("cargo")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg("run")
        .arg("--")
        .arg("--duplicates")
        .arg("-i")
        .arg(dir)
        .spawn()
        .unwrap();

    let process_stdin = process.stdin.as_mut().unwrap();
    process_stdin.write_all(input).unwrap();

    process.wait_with_output().unwrap()
}

fn create_duplicates(root: &Path, count: usize) -> Vec<PathBuf> {
    let mut output = Vec::with_capacity(count);
    for i in 0..count {
        let path = create_and_write(root, i);
        output.push(path)
    }

    output
}

fn create_and_write(root: &Path, id: usize) -> PathBuf {
    let name = format!("file-{}.txt", id);
    let path = root.join(name);
    let mut file = File::create(&path).unwrap();
    file.write(&[0, 1, 2, 3, 4]).unwrap();
    path
}
