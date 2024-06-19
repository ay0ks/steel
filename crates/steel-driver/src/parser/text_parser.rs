use super::PeekBuffer;
use crate::lexer::{Meta, Position, Token, TokenBox};

pub struct TextParser {}

impl TextParser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(&mut self, mut data: PeekBuffer<char>) -> Vec<TokenBox> {
        let mut tokens = Vec::new();

        while let Some(&c) = data.peek() {
            if [' ', '\t'].contains(&c) {
                data.advance_char();
                continue;
            } else if ['\n', '\r'].contains(&c) {
                data.advance_char();
                continue;
            }

            if c == '"' || c == '\'' {
                let start_position = Position::from(&data);
                match c {
                    '"' => {
                        data.advance_char();
                        let mut string = String::new();
                        while let Some(&c) = data.peek() {
                            if c == '"' {
                                break;
                            }
                            string.push(c);
                            data.advance_char();
                        }
                        tokens.push(TokenBox(
                            Token::String(string),
                            start_position,
                            Position::from(&data),
                        ));
                    }
                    '\'' => {
                        data.advance_char();
                        if let Some(&c) = data.peek() {
                            if c == '\'' {
                                data.advance_char();
                                tokens.push(TokenBox(
                                    Token::Character('\0'),
                                    start_position,
                                    Position::from(&data),
                                ));
                            } else {
                                tokens.push(TokenBox(
                                    Token::Character(c),
                                    start_position,
                                    Position::from(&data),
                                ));
                                data.advance_char();
                                if let Some(&c) = data.peek() {
                                    if c == '\'' {
                                        data.advance_char();
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
                continue;
            }

            if c.is_numeric() {
                let start_position = Position::from(&data);
                let mut number = String::new();
                let mut is_float = false;

                while let Some(&c) = data.peek() {
                    if c.is_whitespace() {
                        break;
                    }
                    if c == '.' {
                        is_float = true;
                    }
                    number.push(c);
                    data.advance_char();
                }

                tokens.push(TokenBox(
                    Token::Number(
                        number.parse::<f64>().unwrap(),
                        Meta::Number {
                            is_float,
                            size: number.len(),
                            float_precision: if is_float {
                                number.split('.').collect::<Vec<&str>>()[1].len()
                            } else {
                                0
                            },
                        },
                    ),
                    start_position,
                    Position::from(&data),
                ));
                continue;
            }

            if c.is_alphabetic() || ['.', '_'].contains(&c) {
                let start_position = Position::from(&data);
                let mut word = String::new();

                while let Some(&c) = data.peek() {
                    if c.is_alphanumeric() || ['.', '_'].contains(&c) {
                        word.push(c);
                        data.advance_char();
                    } else {
                        break;
                    }
                }

                tokens.push(TokenBox(
                    Token::Word(
                        word.clone(),
                        match word.as_str() {
                            "add" | "nop" => Meta::Word {
                                is_mnemonic: true,
                                is_attribute: false,
                            },
                            _ if word.starts_with('.') => Meta::Word {
                                is_attribute: true,
                                is_mnemonic: false,
                            },
                            _ => Default::default(),
                        },
                    ),
                    start_position,
                    Position::from(&data),
                ));

                if word.starts_with('.') {
                    let mut value = String::new();

                    if let Some(&c) = data.peek() {
                        if c != '\n' {
                            data.advance_char();
                            while let Some(&c) = data.peek() {
                                if c == '\n' {
                                    break;
                                }
                                value.push(c);
                                data.advance_char();
                            }
                        }
                    }

                    let value_strs = value
                        .split(',')
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>();

                    if value_strs.len() > 0 && value_strs[0].len() > 0 {
                        let mut values: Vec<TokenBox> = Vec::new();
                        for value in value_strs {
                            values.append(&mut self.parse(value.into()));
                        }

                        for i in 0..values.len() - 2 {
                            if values[i].0 == Token::End {
                                if values.len() > i + 1 {
                                    if values[i + 1].0 != Token::End {
                                        values.remove(i);
                                    }
                                }
                            }
                        }
                        tokens.push(TokenBox(
                            Token::List(values),
                            start_position,
                            Position::from(&data),
                        ));
                    }
                    tokens.push(TokenBox(
                        Token::EndBlock,
                        start_position,
                        Position::from(&data),
                    ));
                }
                continue;
            }

            let start_position = Position::from(&data);
            match data.peek() {
                Some('#') => tokens.push(TokenBox(Token::Hashtag, start_position, start_position)),
                Some(':') => tokens.push(TokenBox(Token::Colon, start_position, start_position)),
                Some(',') => tokens.push(TokenBox(Token::Comma, start_position, start_position)),
                Some('(') => {
                    tokens.push(TokenBox(Token::LeftParen, start_position, start_position))
                }
                Some(')') => {
                    tokens.push(TokenBox(Token::RightParen, start_position, start_position))
                }
                Some('[') => {
                    tokens.push(TokenBox(Token::LeftBracket, start_position, start_position))
                }
                Some(']') => tokens.push(TokenBox(
                    Token::RightBracket,
                    start_position,
                    start_position,
                )),
                _ => {}
            }

            data.advance_char();
        }

        tokens.push(TokenBox(
            Token::End,
            Position::from(&data),
            Position::from(&data),
        ));
        tokens
    }
}
