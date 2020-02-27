extern crate walkdir;

use std::io::Error;
use std::path::Path;

use std::fs::remove_file;
use walkdir::WalkDir;

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
pub fn file_remover(path: &str, remove: &mut Box<dyn FileRemove>) -> Result<(), Error> {
    for entry in WalkDir::new(path) {
        let entry = entry?;
        if entry.path().is_dir() {
            continue;
        }
        if remove.remove(entry.path())? {
            remove_file(entry.path())?;
        }
    }

    Ok(())
}
