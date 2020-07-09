use std::collections::HashSet;
use std::ffi::OsStr;
use std::path::Path;

pub struct FileFilter {
    ignore_dirs: Option<HashSet<String>>,
    ignore_exts: Option<HashSet<String>>,
    ignore_hiddens: bool,
}

impl FileFilter {
    pub fn new(exts: Option<&[&str]>, dirs: Option<&[&str]>) -> Self {
        let ignore_dirs = collect_string_slice(dirs);
        let ignore_exts = collect_string_slice(exts);

        Self {
            ignore_dirs,
            ignore_exts,
            ignore_hiddens: false,
        }
    }

    pub fn ingnore_hidden(mut self) -> Self {
        self.ignore_hiddens = true;
        self
    }

    pub fn process_path(&self, path: &Path) -> bool {
        if self.ignore_hiddens && is_hidden(path) {
            true
        } else if path.is_file() {
            check_path(&self.ignore_exts, path.extension())
        } else {
            check_path(&self.ignore_dirs, path.file_name())
        }
    }
}

fn check_path(set: &Option<HashSet<String>>, os_str: Option<&OsStr>) -> bool {
    if let Some(ref set) = set {
        !contains(set, os_str)
    } else {
        true
    }
}

fn contains(set: &HashSet<String>, os_str: Option<&OsStr>) -> bool {
    if let Some(os_str) = os_str {
        if let Some(std_str) = os_str.to_str() {
            set.contains(std_str)
        } else {
            false
        }
    } else {
        false
    }
}

fn collect_string_slice(slice: Option<&[&str]>) -> Option<HashSet<String>> {
    if let Some(slice) = slice {
        let tmp: HashSet<String> = slice.iter().map(|s| s.clone().to_owned()).collect();
        Some(tmp)
    } else {
        None
    }
}

fn is_hidden(path: &Path) -> bool {
    if let Some(name) = path.file_name() {
        if let Some(name) = name.to_str() {
            name.starts_with(".")
        } else {
            false
        }
    } else {
        false
    }
}

#[cfg(test)]
mod tests {

    use std::fs::{create_dir_all, File};
    use std::path::PathBuf;

    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_preserve_files() {
        let root = TempDir::new().unwrap();

        let names = ["test", "main", "photo", "video", "stuff", "file", "program"];

        let preserve_extensions = ["txt", "jpg"];
        let remove_extensions = ["rs", "png", "tiff", "exe", "py", "c", "php"];

        let preserve_files = create_files(
            root.path(),
            &["path", "to", "dir_a"],
            &names,
            &preserve_extensions,
        );
        let remove_files = create_files(
            root.path(),
            &["path", "to", "dir_b"],
            &names,
            &remove_extensions,
        );

        let filter = FileFilter::new(Some(&preserve_extensions), None);
        for file in &preserve_files {
            assert!(!filter.process_path(file));
        }

        for file in &remove_files {
            assert!(filter.process_path(file));
        }
    }

    #[test]
    fn test_preserve_dirs() {
        let root = TempDir::new().unwrap();
        let path_tokens = ["path", "to", "dirs"];

        let preserve_names = [".git", "important_files", "homework"];
        let remove_names = ["remove_dir", "useless_files", "test"];

        let preserve_dirs = create_complex_tree(root.path(), &path_tokens, &preserve_names);
        let remove_dirs = create_complex_tree(root.path(), &path_tokens, &remove_names);

        let filter = FileFilter::new(None, Some(&preserve_names));

        for dir in &preserve_dirs {
            assert!(!filter.process_path(dir));
        }

        for dir in &remove_dirs {
            assert!(filter.process_path(dir));
        }
    }

    #[test]
    fn test_preserve_file_and_dirs() {
        let root = TempDir::new().unwrap();
        let path_tokens = ["path", "to", "dirs"];

        let names = ["test", "main", "photo", "video", "stuff", "file", "program"];

        let preserve_extensions = ["txt", "jpg"];
        let remove_extensions = ["rs", "png", "tiff", "exe", "py", "c", "php"];

        let preserve_files = create_files(root.path(), &path_tokens, &names, &preserve_extensions);
        let remove_files = create_files(root.path(), &path_tokens, &names, &remove_extensions);

        let preserve_names = [".git", "important_files", "homework"];
        let remove_names = ["remove_dir", "useless_files", "test"];

        let preserve_dirs = create_complex_tree(root.path(), &path_tokens, &preserve_names);
        let remove_dirs = create_complex_tree(root.path(), &path_tokens, &remove_names);

        let filter = FileFilter::new(Some(&preserve_extensions), Some(&preserve_names));
        for file in &preserve_files {
            assert!(!filter.process_path(file));
        }

        for file in &remove_files {
            assert!(filter.process_path(file));
        }

        for dir in &preserve_dirs {
            assert!(!filter.process_path(dir));
        }

        for dir in &remove_dirs {
            assert!(filter.process_path(dir));
        }
    }

    fn create_files(
        root: &Path,
        dir_path: &[&str],
        names: &[&str],
        extensions: &[&str],
    ) -> Vec<PathBuf> {
        let mut output = Vec::new();
        let base_dir = create_test_dir(root, dir_path);
        for name in names {
            for ext in extensions {
                let name = format!("{}.{}", name, ext);
                let path = base_dir.join(name);
                File::create(&path).unwrap();
                output.push(path);
            }
        }
        output
    }

    fn create_complex_tree(root: &Path, path_tokens: &[&str], names: &[&str]) -> Vec<PathBuf> {
        let mut output = Vec::new();

        for i in 1..path_tokens.len() {
            let base_dir = create_test_dir(root, &path_tokens[0..i]);
            for name in names {
                let dir_name = base_dir.join(name);
                create_dir_all(&dir_name).unwrap();
                output.push(dir_name)
            }
        }

        output
    }

    fn create_test_dir(root: &Path, dir_path: &[&str]) -> PathBuf {
        create_test_path(root, dir_path, true)
    }

    fn create_test_path(root: &Path, path_tokens: &[&str], dir: bool) -> PathBuf {
        let len = path_tokens.len();
        let path = path_tokens[0..len - 1]
            .iter()
            .fold(root.to_path_buf(), |acc, tok| acc.join(tok));
        create_dir_all(&path).unwrap();
        let output = path.join(path_tokens[len - 1]);
        if dir {
            create_dir_all(&output).unwrap();
        } else {
            File::create(&output).unwrap();
        }
        output
    }
}
