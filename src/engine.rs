use super::file_remove_iterator::*;
use super::io_engine;

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

pub fn automatic_remove(paths: &[&str], mode: Mode, command: Command) -> Result<()> {
    let mut controller = make_controller(command)?;
    for path in paths.iter() {
        run_remove(path, &mode, &mut controller)?;
    }

    Ok(())
}

pub fn remove(file_name: &[&str], mode: Mode, recursive: bool) -> Result<()> {
    for file in file_name {
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
                }
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
) -> Result<()> {
    match mode {
        Mode::Standard => {
            file_remove::file_remover(path, controller)?;
        }
        Mode::Force => {
            let _ = file_remove::file_remover(path, controller);
        }
        Mode::Interactive => {
            if io_engine::remove_question(path)? {
                file_remove::file_remover(path, controller)?;
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
    use std::path::PathBuf;
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
        automatic_remove(&paths, Mode::Standard, Command::Duplicates).unwrap();

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
        automatic_remove(&paths, Mode::Standard, Command::BySize((size_spec, false))).unwrap();
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
        automatic_remove(&paths, Mode::Standard, Command::BySize((size_spec, true))).unwrap();
        for f in non_remove_files.iter() {
            assert!(f.exists());
        }

        for f in remove_files.iter() {
            assert!(!f.exists());
        }
    }

    fn make_sized_files(base_dir: &TempDir, ext: &str, count: usize,  min_size: usize, max_size: usize) -> Vec<PathBuf>{
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

}
