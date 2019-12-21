
use std::io::Error;
use std::fs::remove_dir_all;
use std::fs::remove_file;

use super::file_index;
use super::io_engine;

pub enum Mode {
    Interactive, 
    Force,
    Standard
}


pub fn remove(names: &Vec<&str>, recursive: bool, mode: Mode) -> Result<(), Error> {
    match mode {
        Mode::Force => force_remove_files(names, recursive),
        Mode::Interactive => interactive_remove_files(names, recursive),
        Mode::Standard => std_remove_files(names, recursive),
    }
}

fn std_remove_files(names: &Vec<&str>, rec: bool) -> Result<(), Error> {
    for name in names.iter() {
        remove_wrap(name, rec)?;
    }
    Ok(())
}

fn force_remove_files(names: &Vec<&str>, rec: bool) -> Result<(), Error> {
    for name in names.iter() {
        let _ = remove_wrap(name, rec);
    }
    Ok(())
}

fn interactive_remove_files(names: &Vec<&str>, rec: bool) -> Result<(), Error> {
    for name in names.iter() {
        if io_engine::remove_question(name)? {
            remove_wrap(name, rec)?;
        }
    }
    Ok(())
}


fn remove_wrap(name: &str, rec: bool) -> Result<(), Error> {
    if rec {
        remove_dir_all(name)
    }
    else {
        remove_file(name)
    }
}


pub fn remove_duplicates(names: &Vec<&str>, mode: Mode) -> Result<(), Error> {
    match mode {
        Mode::Standard => std_remove_duplicates(names),
        Mode::Force => force_remove_duplicates(names),
        Mode::Interactive => interactive_remove_duplicates(names)
    }
}

fn std_remove_duplicates(names: &Vec<&str>) -> Result<(), Error> {
    for name in names.iter() {
        file_index::remove_duplicates(name)?;
    }
    Ok(())
}


fn force_remove_duplicates(names: &Vec<&str>) -> Result<(), Error> {
    for name in names.iter() {
        let _ = file_index::remove_duplicates(name);
    }
    Ok(())
}

fn interactive_remove_duplicates(names: &Vec<&str>) -> Result<(), Error> {
    for name in names.iter() {
        if io_engine::remove_question(name)? {
            file_index::remove_duplicates(name)?;
        }
        
    }
    Ok(())
}


#[cfg(test)]
mod test {

    use tempfile::TempDir;
    use std::fs::{File, create_dir};
    use super::*;


    #[test]
    fn test_force_remove() {
        run_remove(Mode::Force);
    }

    #[test]
    fn test_force_remove_dir() {
        run_remove_dir(Mode::Force);
    }

    #[test]
    #[should_panic(expected = "`Result::unwrap()` on an `Err` value: Os")]
    fn test_std_remove() {
        run_remove(Mode::Standard);
    }

    #[test]
    #[should_panic(expected = "`Result::unwrap()` on an `Err` value: Os")]
    fn test_std_remove_dir() {
        run_remove_dir(Mode::Standard);
    }

    fn run_remove(mode: Mode) {
        let dir = TempDir::new().unwrap();
        let existing_file = dir.path().join("EXIST");
        let _ = File::create(&existing_file).unwrap();
        let not_existing_file = dir.path().join("NON_EXIST");
        let files = vec![existing_file.to_str().unwrap(), not_existing_file.to_str().unwrap()];
        remove(&files, false, mode).unwrap();
    }

    fn run_remove_dir(mode: Mode) {
        let dir = TempDir::new().unwrap();
        let existing_dir = dir.path().join("DIR_A");
        create_dir(&existing_dir).unwrap();
        for letter in ["a", "b", "c", "d"].iter() {
            let tmp = existing_dir.join(letter);
            File::create(tmp).unwrap();
        }
        
        let non_existing_dir = dir.path().join("DIR_B");

        let files = vec![existing_dir.to_str().unwrap(), non_existing_dir.to_str().unwrap()];
        remove(&files, true, mode).unwrap();
    }

}
