use super::PeekBuffer;
use crate::lexer::Token;

pub struct TextParser {
    data: PeekBuffer<char>,
    tokens: Vec<Token>,
}

impl TextParser {
    pub fn new(data: PeekBuffer<char>) -> Self {
        Self {
            data,
            tokens: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Vec<Token> {
        while let Some(&c) = self.data.peek() {
            if c == 'q' {
                break;
            }
            print!("{:}", c);
            self.data.advance();
        }
        self.tokens.clone()
    }
}

impl From<String> for TextParser {
    fn from(s: String) -> Self {
        Self::new(s.into())
    }
}

impl From<&str> for TextParser {
    fn from(s: &str) -> Self {
        Self::new(s.into())
    }
}
