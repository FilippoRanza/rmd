use std::collections::HashSet;
use std::path::Path;

pub struct DirectoryCheck {
    names: HashSet<String>,
}

impl DirectoryCheck {
    pub fn new(names: &[&str]) -> Self {
        let names: HashSet<String> = names.iter().map(|n| n.clone().to_owned()).collect();
        Self { names }
    }

    pub fn process_directory(&self, path: &Path) -> bool {
        let dir_name = path.file_name();
        if let Some(dir_name) = dir_name {
            let name = dir_name.to_str().unwrap();
            !self.names.contains(name)
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_directory_protection() {
        let protect = &[".git", ".svn", "important_stuff"];
        let dir_check = DirectoryCheck::new(protect);
        let paths = [
            (Path::new("test").join("directory"), true),
            (Path::new("test").join("dir").join(".git"), false),
            (
                Path::new("root").join("stuff").join("important_stuff"),
                false,
            ),
            (
                Path::new("root")
                    .join("stuff")
                    .join("important_stuff")
                    .join("less_important_stuff"),
                true,
            ),
            (
                Path::new("root")
                    .join("stuff")
                    .join("important_stuff")
                    .join("less_important_stuff")
                    .join(".svn"),
                false,
            ),
        ];

        for (path, stat) in &paths {
            assert_eq!(dir_check.process_directory(path), *stat);
        }
    }
}
