use std::fs::metadata;
use std::io::{Error, ErrorKind};
use std::path::Path;

use super::file_remove::FileRemove;
use super::parser::spec_string_parser;

const BYTE: u64 = 1;

//standard SI sizes
const KILO_BYTE: u64 = 1000 * BYTE;
const MEGA_BYTE: u64 = 1000 * KILO_BYTE;
const GIGA_BYTE: u64 = 1000 * MEGA_BYTE;
const TERA_BYTE: u64 = 1000 * GIGA_BYTE;
const PETA_BYTE: u64 = 1000 * TERA_BYTE;

//binary sizes
const KIBI_BYTE: u64 = 1024 * BYTE;
const MEBI_BYTE: u64 = 1024 * KIBI_BYTE;
const GIBI_BYTE: u64 = 1024 * MEBI_BYTE;
const TEBI_BYTE: u64 = 1024 * GIBI_BYTE;
const PEBI_BYTE: u64 = 1024 * TEBI_BYTE;

pub struct SizeRemove {
    size: u64,
    smaller: bool,
}

impl SizeRemove {
    pub fn new(size_spec: &str, smaller: bool) -> std::io::Result<Self> {
        let tmp = Self::size_converter(size_spec);
        match tmp {
            Ok(size) => Ok(Self::factory(size, smaller)),
            Err(msg) => Err(Self::error_factory(msg)),
        }
    }

    fn size_converter(size_spec: &str) -> Result<u64, String> {
        spec_string_parser(size_spec, |s| {
            match s {
                "b" => Ok(BYTE),
                // standard SI
                "kb" | "kilo" => Ok(KILO_BYTE),
                "mb" | "mega" => Ok(MEGA_BYTE),
                "gb" | "giga" => Ok(GIGA_BYTE),
                "tb" | "tera" => Ok(TERA_BYTE),
                "pb" | "peta" => Ok(PETA_BYTE),
                // binary
                "kib" | "kibi" => Ok(KIBI_BYTE),
                "mib" | "mebi" => Ok(MEBI_BYTE),
                "gib" | "gibi" => Ok(GIBI_BYTE),
                "tib" | "tebi" => Ok(TEBI_BYTE),
                "pib" | "pebi" => Ok(PEBI_BYTE),
                _ => Err(format!("unknown size specifier {}", s)),
            }
        })
    }

    fn factory(size: u64, smaller: bool) -> Self {
        SizeRemove { size, smaller }
    }

    fn error_factory(msg: String) -> Error {
        Error::new(ErrorKind::Other, msg)
    }
}

impl FileRemove for SizeRemove {
    fn remove(&mut self, path: &Path) -> Result<bool, Error> {
        let meta = metadata(path)?;
        let size = meta.len();
        let output = if self.smaller {
            size <= self.size
        } else {
            size >= self.size
        };
        Ok(output)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_size_converter() {
        // these values has been randomly generated
        run_test("1kb 4mib", 4195304);
        run_test("4gb 2kibi 13b", 4000002061);
        run_test("1kibi 1tera ", 1000000001024);
        run_test("6tebi 5gibi 5tb ", 11602438475776);
        run_test("5mebi 1gibi 1tb 0mb 2gb ", 1003078984704);
        run_test("1kilo 3tebi 5pebi ", 5632798069097448);
        run_test("5kib 4mb 6pebi 4kb ", 6755399445064864);
    }

    fn run_test(spec: &str, size: u64) {
        let tmp = SizeRemove::size_converter(spec).unwrap();
        assert_eq!(tmp, size);
    }
}
