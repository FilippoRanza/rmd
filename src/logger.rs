use std::io::Result;
use std::path::Path;
use std::fmt::Write;

enum VerboseLevel {
    Low,
    High,
}

pub struct StatusLogger {
    total_size: u64,
    file_count: usize,
    dir_count: usize,
    cache_log: String,
    level: VerboseLevel,
}

impl StatusLogger {
    pub fn new(level: u64) -> Self {
        let level = if level == 1 {
            VerboseLevel::Low
        } else {
            VerboseLevel::High
        };
        StatusLogger {
            total_size: 0,
            file_count: 0,
            dir_count: 0,
            level,
            cache_log: String::new()
        }
    }

    pub fn log_file_remove<P: AsRef<Path>>(&mut self, file: P) -> Result<()> {
        self.inner_log_file_remove(file.as_ref())
    }

    fn inner_log_file_remove(&mut self, file: &Path) -> Result<()> {
        let size = self.update_stat(file)?;
        let result = match self.level {
            VerboseLevel::Low => writeln!(&mut self.cache_log, "{:?}", file),
            VerboseLevel::High => {
                if file.is_dir() {
                    writeln!(&mut self.cache_log, "Remove Directory: {:?}", file)
                } else {
                    writeln!(&mut self.cache_log, "Remove File: {:?} - freed {}", file, format_size(size))
                }
            }
        };
        result.expect("unable to format log message");
        Ok(())
    }

    pub fn output_log(&mut self) {
        print!("{}", self.cache_log);
        self.cache_log.clear();
    }

    pub fn log_statistics(&mut self) {
        if let VerboseLevel::High = self.level {
            println!("Final job statistics:");
            println!(
                "{} director{} removed",
                self.dir_count,
                if self.dir_count < 2 { "y" } else { "ies" }
            );
            println!(
                "{} file{} removed",
                self.file_count,
                if self.file_count < 2 { "" } else { "s" }
            );
            let tmp = format_size(self.total_size);
            println!("{} freed", tmp);
        }
    }

    fn update_stat(&mut self, file: &Path) -> Result<u64> {
        let output = if file.is_dir() {
            self.dir_count += 1;
            0
        } else {
            self.file_count += 1;
            let meta = file.metadata()?;
            let size = meta.len();
            self.total_size += size;
            size
        };
        Ok(output)
    }
}

pub fn add_file_remove_log<P: AsRef<Path>>(log: &mut Option<StatusLogger>, path: P) -> Result<()> {
    if let Some(log) = log {
        log.log_file_remove(path)
    } else {
        Ok(())
    }
}

pub fn output_file_remove_log(log: &mut Option<StatusLogger>) {
    if let Some(log) = log {
        log.output_log();
    }
}


fn format_size(size: u64) -> String {
    let sizes = ["", "k", "M", "G", "T", "P", "E", "Z"];
    let mut size: f64 = size as f64;
    let mut count = 0;
    while count < sizes.len() - 1 && size > 1000.0 {
        size /= 1000.0;
        count += 1;
    }

    format!("{:.2} {}b", size, sizes[count])
}

#[cfg(test)]
mod test {

    use super::*;
    use tempfile::TempDir;
    use std::io::Write;
    use std::fs::File;
    use std::fs::create_dir;

    #[test]
    fn test_size_conveter() {
        let total_size = 1234;
        assert_eq!(format_size(total_size), "1.23 kb");

        let total_size = 1234567;
        assert_eq!(format_size(total_size), "1.23 Mb");
    }

    #[test]
    fn test_log_formatter() {
        let base_dir = TempDir::new().unwrap();
        

        let dir_path = base_dir.path().join("some_dir");
        create_dir(&dir_path).unwrap();
        let file_path = base_dir.path().join("file.dat");
        let mut large_file = File::create(&file_path).unwrap();

        for _ in 0..1000 {
            large_file.write(&[1, 2, 3, 4, 5]).unwrap();
        }
        
        let mut log = StatusLogger::new(1);

        log.log_file_remove(&file_path).unwrap();
        assert_eq!(log.cache_log, format!("{:?}\n", file_path));
        
        log.output_log();
        assert_eq!(log.cache_log, "");

        log.log_file_remove(&dir_path).unwrap();
        assert_eq!(log.cache_log, format!("{:?}\n", dir_path));

        log.output_log();
        assert_eq!(log.cache_log, "");

        let mut log = StatusLogger::new(2);

        log.log_file_remove(&file_path).unwrap();
        assert_eq!(log.cache_log, format!("Remove File: {:?} - freed 5.00 kb\n", file_path));
        
        log.output_log();
        assert_eq!(log.cache_log, "");

        log.log_file_remove(&dir_path).unwrap();
        assert_eq!(log.cache_log, format!("Remove Directory: {:?}\n", dir_path));

        log.output_log();
        assert_eq!(log.cache_log, "");

    }

}
