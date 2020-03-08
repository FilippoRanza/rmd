
use std::io::{Result, Error, ErrorKind};
use std::path::Path;
use std::time::{Duration, SystemTime};

use crate::file_remove_iterator::file_remove::FileRemove;
use super::parser::spec_string_parser;

const SECONDS_IN_MINUTE: u64 = 60;
const SECONDS_IN_HOUR: u64 = 60 * SECONDS_IN_MINUTE;
const SECONDS_IN_DAY: u64 = 24 * SECONDS_IN_HOUR;
const SECONDS_IN_WEEK: u64 = 7 * SECONDS_IN_DAY;
const SECONDS_IN_MONTH: u64 = 30 * SECONDS_IN_DAY;
const SECONDS_IN_YEAR: u64 = 365 * SECONDS_IN_DAY;

pub struct TimeRemove {
    time: Duration,
    older: bool,
    now: SystemTime,
}

impl TimeRemove {
    pub fn new(time: &str, older: bool) -> Result<Self> {
        let ans = spec_string_parser(time, |s| {
            match s {
                "s"|"second" => Ok(1),
                "m"|"minute" => Ok(SECONDS_IN_MINUTE),
                "h"|"hour" => Ok(SECONDS_IN_HOUR),
                "d"|"day" => Ok(SECONDS_IN_DAY),
                "w"|"week" => Ok(SECONDS_IN_WEEK),
                "M"|"month" => Ok(SECONDS_IN_MONTH),
                "y"|"year" => Ok(SECONDS_IN_YEAR),
                _ => {
                    let err_msg = format!("unknown time specifier {}", s);
                    Err(err_msg)
                }
            }
        });
        match ans {
            Ok(time) => Ok(Self::factory(time, older)),
            Err(msg) => Err(Self::error_factory(msg))
        }
    }

    fn factory(time: u64, older: bool) -> Self {
        let time = Duration::new(time, 0);
        TimeRemove {
            time,
            older,
            now: SystemTime::now()
        }
    }

    fn error_factory(msg: String) -> Error {
        Error::new(ErrorKind::Other, msg)
    }

    fn get_time_diff(&self, path: &Path) -> Result<Duration> {
        let metadata = path.metadata()?;
        let access = metadata.accessed()?;
        let diff = self.now.duration_since(access).unwrap();
        Ok(diff)
    }

}

impl FileRemove for TimeRemove {
    fn remove(&mut self, path: &Path) -> Result<bool> {
        let time_since_access = self.get_time_diff(path)?;
        if self.older {
            if time_since_access > self.time {
                return Ok(true);
            }
        } else {
            if time_since_access < self.time {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
