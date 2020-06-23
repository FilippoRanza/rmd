use std::collections::HashSet;
use std::io::Error;
use std::path::Path;

use super::file_remove::FileRemove;

pub struct ExtensionIgnore {
    extension: HashSet<String>,
    inner: Box<dyn FileRemove>,
}

impl ExtensionIgnore {
    pub fn new(extension: HashSet<String>, inner: Box<dyn FileRemove>) -> Self {
        ExtensionIgnore { extension, inner }
    }
}

impl FileRemove for ExtensionIgnore {
    fn remove(&mut self, path: &Path) -> Result<bool, Error> {
        let file_extension = path.extension();
        let action = if let Some(file_extension) = file_extension {
            let ext = file_extension.to_str().unwrap();
            if self.extension.contains(ext) {
                Action::Keep
            } else {
                Action::Pass
            }
        } else {
            Action::Pass
        };

        match action {
            Action::Keep => Ok(false),
            Action::Pass => self.inner.remove(path),
        }
    }
}

enum Action {
    Keep,
    Pass,
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs::File;
    use tempfile::TempDir;

    struct MockRemover {
        remove_file: bool,
    }

    impl FileRemove for MockRemover {
        fn remove(&mut self, _path: &Path) -> Result<bool, Error> {
            Ok(self.remove_file)
        }
    }

    #[test]
    fn test_ignore_extension() {
        let root = TempDir::new().unwrap();
        let files = [
            (true, "name.jpg"),
            (false, "file.txt"),
            (true, "video.mp4"),
            (true, "main.rs"),
            (false, "list.txt"),
            (false, "script.sh"),
        ];
        for (_, file) in &files {
            let path = root.path().join(file);
            File::create(path).unwrap();
        }

        let mock = MockRemover { remove_file: true };
        let extensions = extension_hash_set(&["sh", "txt"]);
        let mut remover = ExtensionIgnore::new(extensions, Box::new(mock));

        for (status, file) in &files {
            let path = root.path().join(file);
            assert_eq!(remover.remove(&path).unwrap(), *status);
        }
    }

    fn extension_hash_set(extensions: &[&str]) -> HashSet<String> {
        let mut output = HashSet::with_capacity(extensions.len());
        for ext in extensions {
            output.insert(ext.to_owned().to_owned());
        }
        output
    }
}
