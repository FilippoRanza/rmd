use super::file_remove_iterator::*;
use super::io_engine;
use super::logger::StatusLogger;

use std::fs::{remove_dir_all, remove_file};
use std::io::Result;

pub enum Command<'a> {
    BySize((&'a str, bool)),
    ByDate((&'a str, bool)),
    Duplicates,
}

pub enum Mode {
    Standard,
    Force,
    Interactive,
}

pub fn automatic_remove(
    paths: &[&str],
    mode: Mode,
    command: Command,
    clean: bool,
    log: &mut Option<StatusLogger>,
) -> Result<()> {
    let mut controller = make_controller(command)?;
    for path in paths.iter() {
        run_remove(path, &mode, &mut controller, clean, log)?;
    }

    Ok(())
}

pub fn remove(
    file_name: &[&str],
    mode: Mode,
    recursive: bool,
    log: &mut Option<StatusLogger>,
) -> Result<()> {
    for file in file_name {
        let mut done = true;
        match mode {
            Mode::Standard => {
                remove_wrap(file, recursive)?;
            }
            Mode::Force => {
                let _ = remove_wrap(file, recursive);
            }
            Mode::Interactive => {
                if io_engine::remove_question(file)? {
                    remove_wrap(file, recursive)?;
                } else {
                    done = false;
                }
            }
        }
        if done {
            if let Some(log) = log {
                log.log_file_remove(file)?;
            }
        }
    }
    Ok(())
}

fn remove_wrap(name: &str, rec: bool) -> Result<()> {
    if rec {
        remove_dir_all(name)
    } else {
        remove_file(name)
    }
}

fn run_remove(
    path: &str,
    mode: &Mode,
    controller: &mut Box<dyn file_remove::FileRemove>,
    clean: bool,
    log: &mut Option<StatusLogger>,
) -> Result<()> {
    match mode {
        Mode::Standard => {
            file_remove::file_remover(path, controller, clean, log)?;
        }
        Mode::Force => {
            let _ = file_remove::file_remover(path, controller, clean, log);
        }
        Mode::Interactive => {
            if io_engine::remove_question(path)? {
                file_remove::file_remover(path, controller, clean, log)?;
            }
        }
    }

    Ok(())
}

fn make_controller(command: Command) -> Result<Box<dyn file_remove::FileRemove>> {
    match command {
        Command::BySize((size, smaller)) => {
            let val = remove_by_size::SizeRemove::new(&size, smaller)?;
            Ok(Box::new(val))
        }
        Command::ByDate((time, older)) => {
            let val = remove_by_date::TimeRemove::new(&time, older)?;
            Ok(Box::new(val))
        }
        Command::Duplicates => Ok(Box::new(remove_duplicates::FileIndex::new())),
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use std::collections::HashMap;
    use std::fs::{create_dir, File};
    use std::io::prelude::Write;
    use std::path::{Path, PathBuf};
    use std::thread;
    use std::time::Duration;
    use tempfile::tempdir;
    use tempfile::TempDir;

    #[test]
    fn test_remove_duplicates() {
        let temp_dir = tempdir().unwrap();
        let unique = build_unique_file_tree(&temp_dir);
        let duplicates = build_duplicates_file_tree(&temp_dir);
        let paths = [temp_dir.path().to_str().unwrap()];
        automatic_remove(
            &paths,
            Mode::Standard,
            Command::Duplicates,
            false,
            &mut None,
        )
        .unwrap();

        for path in unique.iter() {
            let path = path.as_path();
            assert!(path.exists());
        }

        assert!(temp_dir.path().exists());
        for (_, files) in duplicates.iter() {
            let mut count = 0;
            for file in files.iter() {
                if file.exists() {
                    count += 1;
                }
            }
            assert_eq!(count, 1);
        }
    }

    fn build_unique_file_tree(dir: &TempDir) -> Vec<PathBuf> {
        let mut output = Vec::new();
        let unique_names = vec!["unique_a", "unique_b", "unique_c", "unique_d"];
        for name in unique_names.iter() {
            let path = dir.path().join(name);
            let mut file = File::create(&path).unwrap();
            file.write(name.as_bytes()).unwrap();
            output.push(path);
        }

        let unique_dirs = vec!["dir_a", "dir_b", "dir_c", "dir_d"];
        for unique_dir in unique_dirs.iter() {
            let root = dir.path().join(unique_dir);
            create_dir(&root).unwrap();
            for name in unique_names.iter() {
                let path = root.join(name);
                let mut file = File::create(path).unwrap();
                file.write(unique_dir.as_bytes()).unwrap();
                file.write(name.as_bytes()).unwrap();
            }
        }

        output
    }

    fn build_duplicates_file_tree(dir: &TempDir) -> HashMap<String, Vec<PathBuf>> {
        let mut output = HashMap::new();

        let duplicates = vec!["dup_a", "dup_b", "dup_c", "dup_d"];
        let mut tmp = Vec::new();
        for dup in duplicates.iter() {
            let path = dir.path().join(dup);
            let mut file = File::create(&path).unwrap();
            file.write("data".as_bytes()).unwrap();
            tmp.push(path);
        }

        output.insert(String::new(), tmp);

        let dirs = vec!["dir_a", "dir_b", "dir_c", "dir_d"];

        for name in duplicates.iter() {
            let mut tmp = Vec::new();
            for d in dirs.iter() {
                let path = dir.path().join(d).join(name);
                let mut file = File::create(&path).unwrap();
                file.write(name.as_bytes()).unwrap();
                file.write("second".as_bytes()).unwrap();
                tmp.push(path);
            }
            output.insert(String::from(*name), tmp);
        }

        output
    }

    #[test]
    fn test_remove_old() {
        let temp_dir = TempDir::new().unwrap();

        let file_to_remove = temp_dir.path().join("a");
        File::create(&file_to_remove).unwrap();

        thread::sleep(Duration::new(3, 0));

        let file_to_keep = temp_dir.path().join("b");
        File::create(&file_to_keep).unwrap();
        let paths = [temp_dir.path().to_str().unwrap()];
        automatic_remove(
            &paths,
            Mode::Standard,
            Command::ByDate(("2s", true)),
            false,
            &mut None,
        )
        .unwrap();

        assert!(file_to_keep.exists());
        assert!(!file_to_remove.exists());
    }

    #[test]
    fn test_remove_newer() {
        let temp_dir = TempDir::new().unwrap();

        let file_to_keep = temp_dir.path().join("a");
        File::create(&file_to_keep).unwrap();

        thread::sleep(Duration::new(3, 0));

        let file_to_remove = temp_dir.path().join("b");
        File::create(&file_to_remove).unwrap();
        let paths = [temp_dir.path().to_str().unwrap()];
        automatic_remove(
            &paths,
            Mode::Standard,
            Command::ByDate(("2s", false)),
            false,
            &mut None,
        )
        .unwrap();

        assert!(!file_to_remove.exists());
        assert!(file_to_keep.exists());
    }

    #[test]
    fn test_remove_larger() {
        let size_spec = "4kb+140b";
        let base_dir = tempdir().unwrap();
        let non_remove_files = make_sized_files(&base_dir, "a", 10, 1, 4130);
        let remove_files = make_sized_files(&base_dir, "b", 10, 4140, 10000);
        let paths = [base_dir.path().to_str().unwrap()];
        automatic_remove(
            &paths,
            Mode::Standard,
            Command::BySize((size_spec, false)),
            false,
            &mut None,
        )
        .unwrap();
        for f in non_remove_files.iter() {
            assert!(f.exists());
        }

        for f in remove_files.iter() {
            assert!(!f.exists());
        }
    }
    #[test]
    fn test_remove_smaller() {
        let size_spec = "4kb+140b";
        let base_dir = tempdir().unwrap();
        let remove_files = make_sized_files(&base_dir, "a", 10, 1, 4140);
        let non_remove_files = make_sized_files(&base_dir, "b", 10, 4150, 10000);
        let paths = [base_dir.path().to_str().unwrap()];
        automatic_remove(
            &paths,
            Mode::Standard,
            Command::BySize((size_spec, true)),
            false,
            &mut None,
        )
        .unwrap();
        for f in non_remove_files.iter() {
            assert!(f.exists());
        }

        for f in remove_files.iter() {
            assert!(!f.exists());
        }
    }

    fn make_sized_files(
        base_dir: &TempDir,
        ext: &str,
        count: usize,
        min_size: usize,
        max_size: usize,
    ) -> Vec<PathBuf> {
        let mut output = Vec::with_capacity(count);
        let size_step = (max_size - min_size) / count;
        let mut base_size = min_size;
        let buff: [u8; 1] = [0];
        for i in 0..count {
            let name = format!("size_temp_file_{}_size{}.{}", i, base_size, ext);
            let full_name = base_dir.path().join(name);
            let mut tmp = File::create(&full_name).unwrap();
            for _ in 0..base_size {
                tmp.write(&buff).unwrap();
            }
            base_size += size_step;
            output.push(full_name);
        }
        output
    }

    #[test]
    fn test_remove_duplicates_and_clean() {
        let temp_dir = TempDir::new().unwrap();

        let empty_dirs = ["a", "b", "c", "d"];
        let empty_dirs_after_removal = ["e", "f", "g", "h"];
        let non_empty_dirs = ["i", "l", "m", "n"];

        //this directories will be ignored as the are mean to be empty
        let _ = create_empty_sub_dirs(temp_dir.path(), &empty_dirs);

        // this directories will contain the same files, all but one will be removed
        let dup_dir_paths = create_empty_sub_dirs(temp_dir.path(), &empty_dirs_after_removal);
        create_files(&dup_dir_paths, false);

        // this directories will contain unique files
        let uni_dir_path = create_empty_sub_dirs(temp_dir.path(), &non_empty_dirs);
        create_files(&uni_dir_path, true);

        let full_path_empty = temp_dir.path().join("A").join("B");
        std::fs::create_dir_all(&full_path_empty).unwrap();

        let existing_dir = temp_dir.path().join("C").join("B");
        std::fs::create_dir_all(&existing_dir).unwrap();
        let name = existing_dir.join("file");
        let mut file = File::create(&name).unwrap();
        file.write(name.to_str().unwrap().as_bytes()).unwrap();

        let empty_sub_dir = existing_dir.join("E");
        create_dir(&empty_sub_dir).unwrap();

        let empty_after_remove_sub_dir = existing_dir.join("F");
        create_dir(&empty_after_remove_sub_dir).unwrap();
        let name = empty_after_remove_sub_dir.join("file");
        let mut file = File::create(&name).unwrap();
        file.write(&[0, 1, 2, 3, 4]).unwrap();

        automatic_remove(
            &[temp_dir.path().to_str().unwrap()],
            Mode::Standard,
            Command::Duplicates,
            true,
            &mut None,
        )
        .unwrap();

        assert!(!full_path_empty.exists());
        assert!(existing_dir.exists());
        assert!(!empty_sub_dir.exists());

        for dir in &empty_dirs {
            let tmp = temp_dir.path().join(dir);
            assert!(!tmp.exists());
        }

        let mut count = 0;
        for dir in &empty_dirs_after_removal {
            let tmp = temp_dir.path().join(dir);
            if tmp.exists() {
                count += 1;
            }
        }

        if empty_after_remove_sub_dir.exists() {
            count += 1;
        }

        assert_eq!(count, 1);

        for dir in &non_empty_dirs {
            let tmp = temp_dir.path().join(dir);
            assert!(tmp.exists());
        }
    }

    fn create_files(dirs: &[PathBuf], unique: bool) -> Vec<PathBuf> {
        let mut output = Vec::with_capacity(dirs.len());
        for dir in dirs {
            let tmp = dir.join("File");
            let mut file = File::create(&tmp).unwrap();
            if unique {
                file.write(tmp.to_str().unwrap().as_bytes()).unwrap();
            } else {
                file.write(&[0, 1, 2, 3, 4]).unwrap();
            }
            output.push(tmp);
        }
        output
    }

    fn create_empty_sub_dirs(base_path: &Path, names: &[&str]) -> Vec<PathBuf> {
        let mut output = Vec::with_capacity(names.len());
        for name in names {
            let dir_name = base_path.join(name);
            create_dir(&dir_name).unwrap();
            output.push(dir_name);
        }
        output
    }
}
