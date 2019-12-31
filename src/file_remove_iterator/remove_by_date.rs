use std::char;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::time::{Duration, SystemTime};

use crate::file_remove_iterator::file_remove::{file_remover, FileRemove};

const SECONDS_IN_MINUTE: u64 = 60;
const SECONDS_IN_HOUR: u64 = 60 * SECONDS_IN_MINUTE;
const SECONDS_IN_DAY: u64 = 24 * SECONDS_IN_HOUR;
const SECONDS_IN_WEEK: u64 = 7 * SECONDS_IN_DAY;
const SECONDS_IN_MONTH: u64 = 30 * SECONDS_IN_DAY;
const SECONDS_IN_YEAR: u64 = 365 * SECONDS_IN_DAY;

pub fn remove_older_then(path: &str, time: &str) -> Result<(), Error> {
    remove_by_date(path, time, true)
}

pub fn remove_newer_then(path: &str, time: &str) -> Result<(), Error> {
    remove_by_date(path, time, false)
}

fn remove_by_date(path: &str, time: &str, older: bool) -> Result<(), Error> {
    let time_remove = TimeRemove::new(time, older);
    match time_remove {
        Ok(mut time_remove) => file_remover(path, &mut time_remove),
        Err(ch) => Err(Error::new(
            ErrorKind::InvalidInput,
            format!("unknow token '{}' in time specifier '{}'", ch, time),
        )),
    }
}

struct TimeRemove {
    time: Duration,
    older: bool,
    now: SystemTime,
}

impl TimeRemove {
    fn new(time: &str, older: bool) -> Result<TimeRemove, char> {
        let tmp = TimeRemove::parse_time_str(time)?;
        let out = TimeRemove {
            time: tmp,
            older: older,
            now: SystemTime::now(),
        };
        Ok(out)
    }

    fn convert_time(time_id: char, time_val: u64) -> Result<u64, char> {
        let out = match time_id {
            'y' => time_val * SECONDS_IN_YEAR,
            'M' => time_val * SECONDS_IN_MONTH,
            'w' => time_val * SECONDS_IN_WEEK,
            'd' => time_val * SECONDS_IN_DAY,
            'h' => time_val * SECONDS_IN_HOUR,
            'm' => time_val * SECONDS_IN_MINUTE,
            's' => time_val,
            _ => return Err(time_id),
        };
        Ok(out)
    }

    fn parse_time_str(time: &str) -> Result<Duration, char> {
        let parser = Parser::new(time);
        let mut seconds = 0;
        for token in parser {
            let (val, ch) = token?;
            seconds += TimeRemove::convert_time(ch, val)?;
        }
        Ok(Duration::new(seconds, 0))
    }

    fn get_time_diff(&self, path: &Path) -> Result<Duration, Error> {
        let metadata = path.metadata()?;
        let access = metadata.accessed()?;
        let diff = self.now.duration_since(access).unwrap();
        Ok(diff)
    }
}

impl FileRemove for TimeRemove {
    fn remove(&mut self, path: &Path) -> Result<bool, Error> {
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

enum Status {
    Init,
    Number,
    Letter,
    Error,
}

struct Parser<'a> {
    line: &'a str,
    curr: usize,
}

impl<'a> Parser<'a> {
    fn new(line: &'a str) -> Parser<'a> {
        Parser { line, curr: 0 }
    }

    fn get_status(token: char, status: Status) -> Status {
        match status {
            Status::Init => {
                if token.is_digit(10) {
                    Status::Number
                } else {
                    Status::Error
                }
            }
            Status::Number => {
                if token.is_digit(10) {
                    Status::Number
                } else if token.is_alphabetic() {
                    Status::Letter
                } else {
                    Status::Error
                }
            }
            _ => Status::Error,
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<(u64, char), char>;
    fn next(&mut self) -> Option<Result<(u64, char), char>> {
        if self.curr >= self.line.len() {
            return None;
        }

        let iter = &self.line[self.curr..];
        let mut val: u64 = 0;
        let mut letter = '\0';
        let mut stat = Status::Init;
        for ch in iter.chars() {
            self.curr += 1;
            stat = Parser::get_status(ch, stat);
            match stat {
                Status::Init => (),
                Status::Number => {
                    val *= 10;
                    let tmp: u64 = ch as u64 - '0' as u64;
                    val += tmp;
                }
                Status::Letter => {
                    letter = ch;
                    break;
                }
                Status::Error => return Some(Err(ch)),
            }
        }
        Some(Ok((val, letter)))
    }
}

#[cfg(test)]
mod test {

    extern crate tempfile;

    use super::*;
    use std::error;
    use std::fs::File;
    use std::thread;
    use tempfile::TempDir;

    #[test]
    fn test_time_parse() {
        // 1 year + 1 month + 1 week
        assert_eq!(
            TimeRemove::parse_time_str("1y1M1w"),
            Ok(Duration::new(34732800, 0))
        );
        // Error
        assert_eq!(TimeRemove::parse_time_str("1yy"), Err('y'));
        // 67 day + 1 year -> the order does not metter
        assert_eq!(
            TimeRemove::parse_time_str("67d1y"),
            Ok(Duration::new(37324800, 0))
        );
        // 1 year + 1 month + 1 week + 1 day + 1 hour + 1 minute + 1 second
        assert_eq!(
            TimeRemove::parse_time_str("1y1M1w1d1h1m1s"),
            Ok(Duration::new(34822861, 0))
        );
    }

    #[test]
    fn test_time_string_parser() {
        let mut parser = Parser::new("23a45r2w67y");
        assert_eq!(parser.next(), Some(Ok((23, 'a'))));
        assert_eq!(parser.curr, 3);
        assert_eq!(parser.next(), Some(Ok((45, 'r'))));
        assert_eq!(parser.curr, 6);
        assert_eq!(parser.next(), Some(Ok((2, 'w'))));
        assert_eq!(parser.curr, 8);
        assert_eq!(parser.next(), Some(Ok((67, 'y'))));
        assert_eq!(parser.curr, 11);
        assert_eq!(parser.next(), None);

        let mut parser = Parser::new("rrr");
        assert_eq!(parser.next(), Some(Err('r')));
        assert_eq!(parser.curr, 1);

        let mut parser = Parser::new("34r4&");
        assert_eq!(parser.next(), Some(Ok((34, 'r'))));
        assert_eq!(parser.next(), Some(Err('&')));

        let mut parser = Parser::new("1234");
        assert_eq!(parser.next(), Some(Ok((1234, '\0'))));
        assert_eq!(parser.next(), None);
    }

    #[test]
    fn test_remove_wrong_time_format() {
        let stat = remove_older_then("", "3w4e");
        match stat {
            Ok(_) => assert!(false),
            Err(err) => {
                assert_eq!(err.kind(), ErrorKind::InvalidInput);
                assert_eq!(
                    error::Error::description(&err),
                    "unknow token 'e' in time specifier '3w4e'"
                )
            }
        }
    }

    #[test]
    fn test_remove_old() {
        let temp_dir = TempDir::new().unwrap();

        let file_to_remove = temp_dir.path().join("a");
        File::create(&file_to_remove).unwrap();

        thread::sleep(Duration::new(3, 0));

        let file_to_keep = temp_dir.path().join("b");
        File::create(&file_to_keep).unwrap();

        remove_older_then(temp_dir.path().to_str().unwrap(), "2s").unwrap();

        assert!(file_to_keep.exists());
        assert!(!file_to_remove.exists());
    }

    #[test]
    fn test_remove_newer() {
        let temp_dir = TempDir::new().unwrap();

        let file_to_keep = temp_dir.path().join("a");
        File::create(&file_to_keep).unwrap();

        thread::sleep(Duration::new(3, 0));

        let file_to_remove = temp_dir.path().join("b");
        File::create(&file_to_remove).unwrap();

        remove_newer_then(temp_dir.path().to_str().unwrap(), "2s").unwrap();

        assert!(!file_to_remove.exists());
        assert!(file_to_keep.exists());
    }
}
