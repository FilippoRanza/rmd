
use std::io::Error;
use std::fs::remove_dir_all;
use std::fs::remove_file;

use super::file_index;

pub enum Mode {
    Interactive, 
    Force,
    Standard
}


pub fn remove(names: &Vec<&str>, recursive: bool, mode: Mode) -> Result<(), Error> {
    match mode {
        Mode::Force => force_remove_files(names, recursive),
        Mode::Interactive => Ok(()),
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
        Mode::Interactive => Ok(())
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