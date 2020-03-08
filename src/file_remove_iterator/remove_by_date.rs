use std::io::{Error, ErrorKind};
use std::path::Path;
use std::time::{Duration, SystemTime};

use super::parser::spec_string_parser;
use crate::file_remove_iterator::file_remove::FileRemove;

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
    pub fn new(time: &str, older: bool) -> std::io::Result<Self> {
        let ans = Self::convert_time_spec(time);
        match ans {
            Ok(time) => Ok(Self::factory(time, older)),
            Err(msg) => Err(Self::error_factory(msg)),
        }
    }

    fn factory(time: u64, older: bool) -> Self {
        let time = Duration::new(time, 0);
        TimeRemove {
            time,
            older,
            now: SystemTime::now(),
        }
    }

    fn error_factory(msg: String) -> Error {
        Error::new(ErrorKind::Other, msg)
    }

    fn convert_time_spec(time: &str) -> Result<u64, String> {
        spec_string_parser(time, |s| match s {
            "s" | "second" => Ok(1),
            "m" | "minute" => Ok(SECONDS_IN_MINUTE),
            "h" | "hour" => Ok(SECONDS_IN_HOUR),
            "d" | "day" => Ok(SECONDS_IN_DAY),
            "w" | "week" => Ok(SECONDS_IN_WEEK),
            "M" | "month" => Ok(SECONDS_IN_MONTH),
            "y" | "year" => Ok(SECONDS_IN_YEAR),
            _ => {
                let err_msg = format!("unknown time specifier {}", s);
                Err(err_msg)
            }
        })
    }

    fn get_time_diff(&self, path: &Path) -> std::io::Result<Duration> {
        let metadata = path.metadata()?;
        let access = metadata.accessed()?;
        let diff = self.now.duration_since(access).unwrap();
        Ok(diff)
    }
}

impl FileRemove for TimeRemove {
    fn remove(&mut self, path: &Path) -> std::io::Result<bool> {
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

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_time_convertion() {
        assert_eq!(TimeRemove::convert_time_spec("1y1M1w"), Ok(34732800));
        // 67 day + 1 year -> the order does not metter
        assert_eq!(TimeRemove::convert_time_spec("67d1y"), Ok(37324800));
        // 1 year + 1 month + 1 week + 1 day + 1 hour + 1 minute + 1 second
        assert_eq!(
            TimeRemove::convert_time_spec("1y1M1w1d1h1m1s"),
            Ok(34822861)
        );
    }

    #[test]
    fn test_long_time_convertion() {
        //accept both long and short time specifier
        assert_eq!(
            TimeRemove::convert_time_spec("1year+1month+1w"),
            Ok(34732800)
        );

        // another test to ensure that non alphanumeric characters are ignored
        assert_eq!(
            TimeRemove::convert_time_spec(
                "1 year + 1 month + 1 week + 1 day + 1 hour + 1 minute + 1 second"
            ),
            Ok(34822861)
        )
    }
}
