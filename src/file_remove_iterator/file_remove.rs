use crate::logger;
use std::io::Error;
use std::path::Path;


use std::fs::{read_dir, remove_dir, remove_file};

/// This trait's implementation
/// can be passed as argument to file_remover.
/// This allows to implement tailored file removes
/// without redefine a file iterator
pub trait FileRemove {
    fn remove(&mut self, path: &Path) -> Result<bool, Error>;
}

/// This function iterates though the file
/// tree starting from path. Each encountered files is
/// passed as argument to the  remove.remove
/// if this method returns true the file is removed.
/// the file is left untouched otherwise
pub fn file_remover(
    path: &str,
    remove: &mut Box<dyn FileRemove>,
    clean: bool,
    log: &mut Option<logger::StatusLogger>,
) -> Result<bool, Error> { 
    let mut empty = true;
    let mut directory_queue = Vec::new(); 
    for entry in read_dir(path)? {
        let entry = entry?;
        if entry.path().is_dir() {
            let dir_path = entry.path();
            directory_queue.push(dir_path);
        } else if remove.remove(&entry.path())? {
            logger::add_file_remove_log(log, &entry.path())?;
            remove_file(entry.path())?;
            logger::output_file_remove_log(log);
        } else {
            empty = false;
        }
    }

    for sub_dir in directory_queue {
        let rm_dir = file_remover(sub_dir.to_str().unwrap(), remove, clean, log)?;
        if rm_dir {
            if clean {
                logger::add_file_remove_log(log, &sub_dir)?;
                remove_dir(&sub_dir)?;
                logger::output_file_remove_log(log);
            }
        } else {
            empty = false;
        }
    }

    Ok(empty)
}
