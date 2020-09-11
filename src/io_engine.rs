use crate::file_remove_iterator::file_remove::FileRemove;
use std::io::*;
use std::path::Path;

pub fn remove_question(name: &str) -> Result<bool> {
    let mut buffer = String::new();

    println!("confirm remove: {}? [y/N]", name);
    std::io::stdin().read_line(&mut buffer)?;

    let lower = buffer.to_lowercase();
    let ans = lower.trim();

    if ans.len() > 0 && "yes".starts_with(ans) {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub struct InteractiveFileRemove {
    file_remove: Box<dyn FileRemove>,
}

impl InteractiveFileRemove {
    pub fn new(file_remove: Box<dyn FileRemove>) -> Self {
        Self { file_remove }
    }
}

impl FileRemove for InteractiveFileRemove {
    fn remove(&mut self, path: &Path) -> Result<bool> {
        if self.file_remove.remove(path)? {
            let name = if let Some(name) = path.to_str() {
                name
            } else {
                eprintln!("WARNING: CANNOT DISPLAY FILE NAME!");
                ""
            };
            remove_question(name)
        } else {
            Ok(false)
        }
    }
}
