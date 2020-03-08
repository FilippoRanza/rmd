use super::specification_string_tokenizer::{SpecToken, SpecTokenizer};

pub fn spec_string_parser<F>(spec: &str, f: F) -> Result<u64, String>
where
    F: Fn(&str) -> Result<u64, String>,
{
    let mut previous: Option<SpecToken> = None;
    let mut total: u64 = 0;
    let mut run_once = false;
    let tokenizer = SpecTokenizer::new(spec);
    for token in tokenizer {
        if let Some(prev) = previous {
            match (prev, token) {
                (SpecToken::Number(n), SpecToken::Text(t)) => {
                    let tmp = f(t)?;
                    total += n * tmp;
                    previous = None;
                    run_once = true;
                }
                _ => {
                    let err_msg = format!("Cannot parse `{}`", spec);
                    return Err(err_msg);
                }
            }
        } else {
            previous = Some(token);
        }
    }
    if run_once {
        Ok(total)
    } else {
        let err_msg = format!("`{}` does not contain any specification", spec);
        Err(err_msg)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_specification_string_parser() {
        /*
            a = 10;
            b = 100;
            c = 1000;
            spec -> 5440;
        */
        let spec = "4-a,4-b,5-c";
        let ans = spec_string_parser(spec, |s| match s {
            "a" => Ok(10),
            "b" => Ok(100),
            "c" => Ok(1000),
            err => Err(format!("unknown specifier {}", err)),
        })
        .unwrap();
        assert_eq!(ans, 5440);
    }

    #[test]
    fn test_with_minimal_string() {
        let spec = "4k";
        let ans = run_weight_test(spec).unwrap();
        assert_eq!(ans, 4000);
    }

    #[test]
    #[should_panic(expected = "`` does not contain any specification")]
    fn test_with_empty_string() {
        let spec = "";
        let ans = run_weight_test(spec).unwrap();
        assert_eq!(ans, 0);
    }

    #[test]
    fn test_with_long_correct_string() {
        let spec = "3kilo4gram2ton2t45k";
        let ans = run_weight_test(spec).unwrap();
        assert_eq!(ans, 4048004);
    }

    #[test]
    #[should_panic(expected = "unknown specifier tn")]
    fn test_with_malformatted_specifier() {
        let spec = "3kilo4gram2tn2t45k";
        let ans = run_weight_test(spec).unwrap();
        assert_eq!(ans, 0);
    }

    #[test]
    #[should_panic(expected = "Cannot parse `ton43`")]
    fn test_with_wrong_specifier() {
        let spec = "ton43";
        let ans = run_weight_test(spec).unwrap();
        assert_eq!(ans, 0);
    }

    #[test]
    #[should_panic(expected = "`....` does not contain any specification")]
    fn test_with_empty_specifier() {
        let spec = "....";
        let ans = run_weight_test(spec).unwrap();
        assert_eq!(ans, 0);
    }

    fn run_weight_test(spec: &str) -> Result<u64, String> {
        spec_string_parser(spec, |s| match s {
            "k" | "kilo" => Ok(1000),
            "g" | "gram" => Ok(1),
            "t" | "ton" => Ok(1000_000),
            err => Err(format!("unknown specifier {}", err)),
        })
    }
}
