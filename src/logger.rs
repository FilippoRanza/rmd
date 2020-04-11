use std::fmt::Write;
use std::io::Result;
use std::path::Path;

use log;
use log::info;
use syslog;

pub enum VerboseLevel {
    Low,
    High,
}

pub fn get_levle_from_int(level: u64) -> VerboseLevel {
    if level < 2 {
        VerboseLevel::Low
    } else {
        VerboseLevel::High
    }
}

enum Kind {
    Verbose,
    Log
}

pub struct StatusLogger {
    verbose: Option<LogBuilder>,
    logger : Option<LogBuilder>
}

impl StatusLogger {
    pub fn new() -> Self {
        StatusLogger {
            verbose: None,
            logger: None
        }
    }

    pub fn add_verbose(&mut self, level: VerboseLevel) {
        let tmp = LogBuilder::new(level, Kind::Verbose);
        self.verbose = Some(tmp);
    }

    pub fn add_logger(&mut self, level: VerboseLevel) {
        let tmp = LogBuilder::new(level, Kind::Log);
        self.logger = Some(tmp);
    }

    pub fn is_used(&mut self) -> bool {
        if let Some(_) = self.logger {
            true
        } else if let Some(_) = self.verbose {
            true
        } else {
            false
        }
    } 

    pub fn log_file_remove<P: AsRef<Path>>(&mut self, file: P) -> Result<()> {
        if let Some(ref mut verb) = self.verbose {
            verb.log_file_remove(&file)?;
        }
        if let Some(ref mut log) = self.logger {
            log.log_file_remove(&file)?;
        }
        Ok(())
    }

    pub fn output_log(&mut self) {
        if let Some(ref mut verb) = self.verbose {
            verb.output_log();
        }
        if let Some(ref mut log) = self.logger {
            log.output_log();
        }
    }

    pub fn log_statistics(&mut self) {
        if let Some(ref mut verb) = self.verbose {
            verb.log_statistics();
            verb.output_log();
        }
        if let Some(ref mut log) = self.logger {
            log.log_statistics();
            log.output_log();
        }

    }

}


 struct LogBuilder {
    total_size: u64,
    file_count: usize,
    dir_count: usize,
    curr_size: u64,
    is_dir: bool,
    cache_log: String,
    level: VerboseLevel,
    kind: Kind,
}

impl LogBuilder {
     fn new(level: VerboseLevel, kind: Kind) -> Self {

        if let Kind::Log = kind {
            LogBuilder::init_logger();
        }

        LogBuilder {
            total_size: 0,
            file_count: 0,
            dir_count: 0,

            curr_size: 0,
            is_dir: false,

            level,
            cache_log: String::new(),
            kind,
        }
    }

     fn log_file_remove<P: AsRef<Path>>(&mut self, file: P) -> Result<()> {
        self.inner_log_file_remove(file.as_ref())
    }

    fn inner_log_file_remove(&mut self, file: &Path) -> Result<()> {
        self.cache_log.clear();
        let size = self.update_stat(file)?;
        let result = match self.level {
            VerboseLevel::Low => writeln!(&mut self.cache_log, "{:?}", file),
            VerboseLevel::High => {
                if file.is_dir() {
                    writeln!(&mut self.cache_log, "Remove Directory: {:?}", file)
                } else {
                    writeln!(
                        &mut self.cache_log,
                        "Remove File: {:?} - freed {}",
                        file,
                        format_size(size)
                    )
                }
            }
        };
        result.expect("unable to format log message");
        Ok(())
    }

     fn output_log(&mut self) {

        if self.is_dir {
            self.dir_count += 1;
        } else {
            self.file_count += 1;
            self.total_size += self.curr_size;
        }


        match self.kind {
            Kind::Verbose => print!("{}", self.cache_log),
            Kind::Log => info!("{}", self.cache_log)
        }
        self.cache_log.clear();
        self.curr_size = 0;
    }

     fn log_statistics(&mut self) {
        if let VerboseLevel::High = self.level {
            writeln!(&mut self.cache_log, "Final job statistics:")
                .expect("unable to format log message");
            writeln!(
                &mut self.cache_log,
                "{} director{} removed",
                self.dir_count,
                if self.dir_count < 2 { "y" } else { "ies" }
            )
            .expect("unable to format log message");
            writeln!(
                &mut self.cache_log,
                "{} file{} removed",
                self.file_count,
                if self.file_count < 2 { "" } else { "s" }
            )
            .expect("unable to format log message");
            let tmp = format_size(self.total_size);
            writeln!(&mut self.cache_log, "{} freed", tmp).expect("unable to format log message");
        }
    }

    fn update_stat(&mut self, file: &Path) -> Result<u64> {
        if file.is_dir() {
            self.curr_size = 0;
            self.is_dir = true;
        } else {
            let meta = file.metadata()?;
            let size = meta.len();
            self.curr_size += size;
            self.is_dir = false;
        }
        Ok(self.curr_size)
    }

    fn init_logger() -> bool {
        let res = syslog::init(syslog::Facility::LOG_USER, log::LevelFilter::Info, None);
        match res {
            Ok(()) => true,
            Err(_) => false,
        }
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
    use std::fs::create_dir;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

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

        let mut log = LogBuilder::new(VerboseLevel::Low, Kind::Verbose);

        log.log_file_remove(&file_path).unwrap();
        assert_eq!(log.cache_log, format!("{:?}\n", file_path));

        log.output_log();
        assert_eq!(log.cache_log, "");

        log.log_file_remove(&dir_path).unwrap();
        assert_eq!(log.cache_log, format!("{:?}\n", dir_path));

        log.output_log();
        assert_eq!(log.cache_log, "");

        let mut log = LogBuilder::new(VerboseLevel::High, Kind::Verbose);

        log.log_file_remove(&file_path).unwrap();
        assert_eq!(
            log.cache_log,
            format!("Remove File: {:?} - freed 5.00 kb\n", file_path)
        );

        assert_eq!(log.curr_size, 5000);
        assert_eq!(log.total_size, 0);

        log.output_log();
        assert_eq!(log.cache_log, "");

        assert_eq!(log.total_size, 5000);
        assert_eq!(log.curr_size, 0);

        log.log_file_remove(&dir_path).unwrap();
        assert_eq!(log.cache_log, format!("Remove Directory: {:?}\n", dir_path));

        log.output_log();
        assert_eq!(log.cache_log, "");
    }
}
