
extern crate  walkdir;

use std::path::Path;
use std::io::Error;

use std::fs::remove_file;
use walkdir::WalkDir;


pub trait FileRemove {
    fn remove(&mut self, path: &Path) -> Result<bool, Error>;
}


pub fn file_remover(path: &str, remove: &mut dyn FileRemove) -> Result<(), Error> {
    for entry in WalkDir::new(path) {
        let entry = entry?;
        if entry.path().is_dir() {
            continue;
        }
        if !remove.remove(entry.path())? {
            remove_file(entry.path())?;
        } 
    }

    Ok(())
} 

