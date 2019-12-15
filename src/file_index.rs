
extern crate ring;
extern crate data_encoding;
extern crate  walkdir;

use std::io::prelude::*;
use std::io::Error;
use std::collections::HashSet;
use std::fs::{File, remove_file};
use std::path::Path;

use ring::digest::{Context,  SHA256};
use data_encoding::HEXUPPER;


use walkdir::WalkDir;


pub fn remove_duplicates(path: &str) -> Result<(), Error> {

    let mut index = FileIndex::new();

    for entry in WalkDir::new(path) {
        let entry = entry?;
        if entry.path().is_dir() {
            continue;
        }
        if !index.unique(entry.path())? {
            remove_file(entry.path())?;
        } 
    }

    Ok(())
}


struct FileIndex {
    store: HashSet<String>
}

impl FileIndex {
    fn new() -> FileIndex {
        FileIndex {
            store: HashSet::new()
        }
    }

    fn unique(&mut self, path: &Path) -> Result<bool, Error> {
        let hash = hash_file(path)?;
        if self.store.contains(&hash) {
            return Ok(false);
        }
        self.store.insert(hash);
        Ok(true)
    }

}


fn hash_file(path: &Path) -> Result<String, Error> {
    let mut input = File::open(path)?;
    let mut buff = [0; 1024];
    let mut context = Context::new(&SHA256);
    
    loop{
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
    use tempfile::{tempdir, TempDir};
    use std::io::prelude::Write;
    use std::path::PathBuf;
    use std::fs::create_dir;
    use std::collections::HashMap;


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
            let a = index.unique(dir.path().join(n).as_path()).unwrap();
            assert_eq!(a, *c);
        }

    }


    #[test]
    fn test_remove_duplicates() {
        let temp_dir = tempdir().unwrap();
        let unique = build_unique_file_tree(&temp_dir);
        let duplicates = build_duplicates_file_tree(&temp_dir);
        remove_duplicates(temp_dir.path().to_str().unwrap()).unwrap();


        for path in unique.iter() {
            let path = path.as_path();
            assert!(path.exists());
        }

        assert!(temp_dir.path().exists());
        for (key, files) in duplicates.iter() {
            let mut count = 0;
            for file in files.iter() {
                if file.exists() {
                    count += 1;
                }
            }
            //let count = files.iter().fold(0, |_, x| {if x.exists() {1} else {0}});
            println!("Current {} {}", key, count);
            assert_eq!(count, 1);
        }


    }


    fn build_unique_file_tree(dir: &TempDir) -> Vec<PathBuf> {
        let mut output = Vec::new();
        let unique_names = vec!["unique_a", "unique_b", "unique_c", "unique_d"];
        for name in unique_names.iter() {
            let path = dir.path().join(name);
            let mut file = File::create(&path).unwrap();
            file.write(name.as_bytes()).unwrap();
            output.push(path);
        }


        let unique_dirs = vec!["dir_a", "dir_b", "dir_c", "dir_d"];
        for unique_dir in unique_dirs.iter() {
            let root = dir.path().join(unique_dir);
            create_dir(&root).unwrap();
            for name in unique_names.iter() {
                let path = root.join(name);
                let mut file = File::create(path).unwrap();
                file.write(unique_dir.as_bytes()).unwrap();
                file.write(name.as_bytes()).unwrap();
            }
        }

        output 
    }


    fn build_duplicates_file_tree(dir: &TempDir) -> HashMap<String, Vec<PathBuf>> {
        let mut output = HashMap::new();

        let duplicates = vec!["dup_a", "dup_b", "dup_c", "dup_d"];
        let mut tmp = Vec::new();
        for dup in duplicates.iter() {
            let path = dir.path().join(dup);
            let mut file = File::create(&path).unwrap();
            file.write("data".as_bytes()).unwrap();
            tmp.push(path);
        }

        output.insert(String::new(), tmp);


        let dirs = vec!["dir_a", "dir_b", "dir_c", "dir_d"];
        
        for name in duplicates.iter() {
            let mut tmp = Vec::new();
            for d in dirs.iter() {
                let path = dir.path().join(d).join(name);
                let mut file = File::create(&path).unwrap();
                file.write(name.as_bytes()).unwrap();
                file.write("second".as_bytes()).unwrap();
                tmp.push(path);
            }
            output.insert(String::from(*name), tmp);
        }
        
        output
    }



}
