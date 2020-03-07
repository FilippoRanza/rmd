#[derive(Debug, PartialEq)]
pub enum SpecToken<'a> {
    Number(i32),
    Text(&'a str),
}

enum State {
    Init,
    Number,
    Text,
}

pub struct SpecTokenizer<'a> {
    string: &'a str,
    state: State,
    begin: usize,
    end: usize,
}

impl<'a> SpecTokenizer<'a> {
    pub fn new(string: &'a str) -> Self {
        SpecTokenizer {
            string,
            state: State::Init,
            begin: 0,
            end: 0,
        }
    }
    fn take_next(&mut self) {
        self.begin = self.end;
        self.state = State::Init;
        let mut end = true;
        for ch in self.string[self.begin..].chars() {
            match self.state {
                State::Init => {
                    if ch.is_ascii_digit() {
                        self.state = State::Number;
                        self.end = self.begin;
                    } else if ch.is_ascii_alphabetic() {
                        self.state = State::Text;
                        self.end = self.begin;
                    } else {
                        self.begin += 1;
                    }
                }
                State::Number => {
                    self.end += 1;
                    if !ch.is_numeric() {
                        end = false;
                        break;
                    }
                }
                State::Text => {
                    self.end += 1;
                    if !ch.is_ascii_alphabetic() {
                        end = false;
                        break;
                    }
                }
            }
        }
        if end {
            self.end = self.string.len();
        }
    }
}

impl<'a> Iterator for SpecTokenizer<'a> {
    type Item = SpecToken<'a>;

    fn next(&mut self) -> Option<SpecToken<'a>> {
        self.take_next();
        if self.begin == self.string.len() {
            return None;
        }

        let tmp = &self.string[self.begin..self.end];
        let out = match self.state {
            State::Number => {
                let n: i32 = tmp.parse().unwrap();
                SpecToken::Number(n)
            }
            State::Text => SpecToken::Text(tmp),
            State::Init => panic!(),
        };
        Some(out)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_specification_parser() {
        let spec = "56year,,,12month++,+,5day";
        let mut tokenizer = SpecTokenizer::new(spec);
        assert_eq!(tokenizer.next(), Some(SpecToken::Number(56)));
        assert_eq!(tokenizer.next(), Some(SpecToken::Text("year")));
        assert_eq!(tokenizer.next(), Some(SpecToken::Number(12)));
        assert_eq!(tokenizer.next(), Some(SpecToken::Text("month")));
        assert_eq!(tokenizer.next(), Some(SpecToken::Number(5)));
        assert_eq!(tokenizer.next(), Some(SpecToken::Text("day")));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn test_wrong_specification() {
        let spec = "...........";
        let mut tokenizer = SpecTokenizer::new(spec);
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn test_take_next() {
        let spec = "55year,12month+5day";
        let mut tokenizer = SpecTokenizer::new(spec);
        // 56
        run_take_next(&mut tokenizer, 0, 2);
        // year
        run_take_next(&mut tokenizer, 2, 6);
        // 12
        run_take_next(&mut tokenizer, 7, 9);
        // month
        run_take_next(&mut tokenizer, 9, 14);
        // 5
        run_take_next(&mut tokenizer, 15, 16);
        // day
        run_take_next(&mut tokenizer, 16, 19);
    }

    fn run_take_next(tokenizer: &mut SpecTokenizer, begin: usize, end: usize) {
        tokenizer.take_next();
        assert_eq!(
            tokenizer.begin,
            begin,
            "Fail: expected {} {}, actual {} {}, result '{}'",
            begin,
            end,
            tokenizer.begin,
            tokenizer.end,
            &tokenizer.string[tokenizer.begin..tokenizer.end]
        );
        assert_eq!(
            tokenizer.end,
            end,
            "Fail: expected {} {}, actual {} {}, result '{}'",
            begin,
            end,
            tokenizer.begin,
            tokenizer.end,
            &tokenizer.string[tokenizer.begin..tokenizer.end]
        );
    }
}
