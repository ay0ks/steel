use crate::parser::PeekBuffer;

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Meta {
    #[default]
    None,
    Number {
        is_float: bool,
        size: usize,
        float_precision: usize,
    },
    Word {
        is_attribute: bool,
        is_mnemonic: bool,
    },
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Position(pub usize, pub usize);

impl From<(usize, usize)> for Position {
    fn from((line, column): (usize, usize)) -> Self {
        Self(line, column)
    }
}

impl From<&PeekBuffer<char>> for Position {
    fn from(buffer: &PeekBuffer<char>) -> Self {
        Self(buffer.line, buffer.column)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Token {
    #[default]
    End,
    EndBlock,

    Hashtag,
    Colon,
    Comma,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,

    Character(char),
    String(String),
    Number(f64, Meta),

    Word(String, Meta),

    List(Vec<TokenBox>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TokenBox(pub Token, pub Position, pub Position);
