#[derive(Clone, Debug)]
pub struct Input {
    pub line: usize,
    pub col: usize,
    pub source: String,
}

impl Input {
    pub fn new(line: usize, col: usize, source: String) -> Self {
        Input { line, col, source }
    }

    pub fn char_offset(mut self, count: usize) -> Self {
        self.col += count;
        self.source = self.source.chars().skip(count).collect();
        self
    }
}

impl Into<Input> for String {
    fn into(self) -> Input {
        Input {
            line: 0,
            col: 0,
            source: self,
        }
    }
}

impl Into<Input> for &str {
    fn into(self) -> Input {
        Input {
            line: 0,
            col: 0,
            source: self.to_string(),
        }
    }
}

#[cfg(test)]
mod test_input {
    use super::Input;

    #[test]
    fn test_offset() {
        let input = Input::new(3, 12, "hello there!".to_string());
        let input = input.char_offset(4);
        assert_eq!(input.line, 3);
        assert_eq!(input.col, 16);
        assert_eq!(input.source, "o there!".to_string());
    }
}
