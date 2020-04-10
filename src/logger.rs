use std::io::Result;
use std::path::Path;

enum VerboseLevel {
    Low,
    High,
}

pub struct StatusLogger {
    total_size: u64,
    file_count: usize,
    dir_count: usize,
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
        }
    }

    pub fn log_file_remove<P: AsRef<Path>>(&mut self, file: P) -> Result<()> {
        self.inner_log_file_remove(file.as_ref())
    }

    fn inner_log_file_remove(&mut self, file: &Path) -> Result<()> {
        let size = self.update_stat(file)?;
        match self.level {
            VerboseLevel::Low => println!("{:?}", file),
            VerboseLevel::High => {
                if file.is_dir() {
                    println!("Remove Directory: {:?}", file);
                } else {
                    println!("Remove File: {:?} - freed {}", file, format_size(size));
                }
            }
        }
        Ok(())
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

    #[test]
    fn test_size_conveter() {
        let total_size = 1234;
        assert_eq!(format_size(total_size), "1.23 kb");

        let total_size = 1234567;
        assert_eq!(format_size(total_size), "1.23 Mb");
    }
}
