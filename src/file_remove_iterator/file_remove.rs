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
) -> Result<bool, Error> {
    let mut empty = true;
    for entry in read_dir(path)? {
        let entry = entry?;
        if entry.path().is_dir() {
            let dir_path = entry.path();
            let rm_dir = file_remover(dir_path.to_str().unwrap(), remove, clean)?;
            if rm_dir {
                if clean {
                    remove_dir(dir_path)?;
                }
            } else {
                empty = false;
            }
        } else if remove.remove(&entry.path())? {
            remove_file(entry.path())?;
        } else {
            empty = false;
        }
    }
    Ok(empty)
}
