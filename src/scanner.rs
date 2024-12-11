pub struct Scanner {
    cursor: usize,
    characters: Vec<char>,
    enabled: bool,
}

impl Scanner {
    pub fn new(string: &str) -> Self {
        Self {
            cursor: 0,
            characters: string.chars().collect(),
            enabled: true,
        }
    }

    /// get current cursor
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn r#do(&mut self) {
        self.enabled = true;
    }

    pub fn dont(&mut self) {
        self.enabled = false;
    }

    /// get next char without advancing the cursor
    pub fn peek(&self) -> Option<&char> {
        self.characters.get(self.cursor)
    }

    /// return true if cursor is at the end
    pub fn is_done(&self) -> bool {
        self.cursor == self.characters.len()
    }

    /// get next char (if available) and advance the cursor
    pub fn pop(&mut self) -> Option<&char> {
        match self.characters.get(self.cursor) {
            Some(character) => {
                self.cursor += 1;

                Some(character)
            }
            None => None,
        }
    }

    /// return true if chars[cursor] == target and advance the cursor 
    pub fn take(&mut self, target: &char) -> bool {
        match self.characters.get(self.cursor) {
            Some(character) => {
                if target == character {
                    self.cursor += 1;

                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    /// Transform a character into some value
    pub fn transform<T>(
        &mut self,
        callback: impl FnOnce(&char) -> Option<T>,
    ) -> Option<T> {
        match self.characters.get(self.cursor) {
            Some(input) => match callback(input) {
                Some(output) => {
                    self.cursor += 1;

                    Some(output)
                }
                None => None,
            }
            None => None,
        }
    }

    /// Scan a slice for a pattern and perform an Action.
    pub fn scan<T>(
        &mut self,
        callback: impl Fn(&str) -> Option<Action<T>>,
    ) -> Result<Option<T>, Error> {
        let mut sequence = String::new();
        let mut require = false;
        let mut request = None;

        loop {
            match self.characters.get(self.cursor) {
                Some(target) => {
                    sequence.push(*target);

                    match callback(&sequence) {
                        Some(Action::Return(result)) => {
                            self.cursor += 1;

                            break Ok(Some(result))
                        },
                        Some(Action::Request(result)) => {
                            self.cursor += 1;
                            require = false;
                            request = Some(result);
                        },
                        Some(Action::Require) => {
                            self.cursor += 1;
                            require = true;
                        },
                        None => if require {
                            break Err(Error::Character(self.cursor))
                        } else {
                            break Ok(request)
                        },
                    }
                },
                None => if require {
                    break Err(Error::EndOfLine)
                } else {
                    break Ok(request)
                },
            }
        }
    }

    pub fn try_u32(&mut self) -> Option<u32> {
        match self.peek() {
            Some(char) => {
                if char.is_ascii_digit() {
                    let mut ret = char.to_digit(10).unwrap();
                    self.pop();

                    while let Some(char) = self.peek() {
                        if char.is_ascii_digit() {

                            ret = ret*10 + char.to_digit(10).unwrap();
                            self.pop();
                        } else {
                            break;
                        }
                    }

                    Some(ret)
                } else {
                    None
                }
            },
            None => None,
        }
    }

    pub fn try_i32(&mut self) -> Option<i32> {
        match self.peek() {
            Some('-') => {
                self.pop();
                
                let mut ret;
                if let Some(first) = self.peek() {
                    if first.is_ascii_digit() {
                        ret = first.to_digit(10).unwrap();
                        self.pop();
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
                
                while let Some(char) = self.peek() {
                    if char.is_ascii_digit() {
                        ret = ret*10 + char.to_digit(10).unwrap();
                        self.pop();
                    } else {
                        break;
                    }
                }

                Some(-(ret as i32))
            },
            Some(char) if char.is_ascii_digit() => {
                let mut ret = 0;
                while let Some(char) = self.peek() {
                    if char.is_ascii_digit() {
                        ret = ret*10 + char.to_digit(10).unwrap();
                        self.pop();
                    } else {
                        break;
                    }
                }

                Some(ret as i32)
            },
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Character(usize),
    EndOfLine,
}

pub enum Action<T> {
    /// If next iteration returns None,
    /// return T without advancing the cursor.
    Request(T),

    /// If next iteration returns None,
    /// return None without advancing the cursor.
    Require,

    /// Immediately advance the cursor and return T.
    Return(T),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_u32() {
        let input = "12345";
        let mut scanner = Scanner::new(input);

        let result = scanner.try_u32();

        assert_eq!(result, Some(12345));
    }

    #[test]
    fn fail_parse_u32() {
        let input = "ab12";
        let mut scanner = Scanner::new(input);

        let result = scanner.try_u32();

        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_i32_negative() {
        let input = "-13";
        let mut scanner = Scanner::new(input);

        let result = scanner.try_i32();

        assert_eq!(result, Some(-13));
    }
}
