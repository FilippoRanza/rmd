extern crate data_encoding;
extern crate ring;

use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;

use data_encoding::HEXUPPER;
use ring::digest::{Context, SHA256};

use crate::file_remove_iterator::file_remove::FileRemove;

pub struct FileIndex {
    store: HashSet<String>,
}

impl FileIndex {
    pub fn new() -> FileIndex {
        FileIndex {
            store: HashSet::new(),
        }
    }
}

impl FileRemove for FileIndex {
    fn remove(&mut self, path: &Path) -> Result<bool, Error> {
        let hash = hash_file(path)?;
        if self.store.contains(&hash) {
            return Ok(true);
        }
        self.store.insert(hash);
        Ok(false)
    }
}

fn hash_file(path: &Path) -> Result<String, Error> {
    let mut input = File::open(path)?;
    let mut buff = [0; 1024];
    let mut context = Context::new(&SHA256);

    loop {
        let count = input.read(&mut buff)?;
        if count == 0 {
            break;
        }
        context.update(&buff[..count]);
    }

    let digest = context.finish();
    let hash = HEXUPPER.encode(digest.as_ref());
    Ok(hash)
}

#[cfg(test)]
mod test {

    extern crate tempfile;

    use super::*;
    use std::io::prelude::Write;
    use tempfile::tempdir;

    #[test]
    fn test_file_hash() {
        let dir = tempdir().unwrap();
        let msg = "A RANDOM MESSAGE\n";
        let path = dir.path().to_owned();
        let path = path.join("RANDOM_NAME");
        let mut file = File::create(&path).unwrap();
        file.write(msg.as_bytes()).unwrap();

        let ans = "2FF711FDB1CB48EA4B1BBD34C5CE5817921AC0FC852B34DAEB250D1293DE8B63";
        assert_eq!(ans, hash_file(&path).unwrap());
    }

    #[test]
    fn test_file_index() {
        let names = vec!["unique", "equal_1", "equal_2"];
        let dir = tempdir().unwrap();

        let eq_data = "EQUAL FILES";
        let uniq_data = "UNIQUE FILE";

        let base_dir = dir.path().to_owned();

        let file_path = base_dir.join(names[0]);
        let mut unique_file = File::create(&file_path).unwrap();
        unique_file.write(uniq_data.as_bytes()).unwrap();

        for name in names.iter().skip(1) {
            let mut eq_file = File::create(dir.path().join(name)).unwrap();
            eq_file.write(eq_data.as_bytes()).unwrap();
        }

        let ans = vec![true, true, false];

        let mut index = FileIndex::new();
        for (n, c) in names.iter().zip(ans.iter()) {
            let a = !index.remove(dir.path().join(n).as_path()).unwrap();
            assert_eq!(a, *c);
        }
    }
}
