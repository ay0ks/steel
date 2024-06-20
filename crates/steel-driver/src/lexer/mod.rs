use core::slice::Iter;
use peekable_fwd_bwd::Peekable;
use std::{io::Chain, slice::range, str::Chars};

mod marker;
mod token;
pub use marker::{Marker, Position};
pub use token::{Token, TokenVariant};

macro_rules! marker {
    ($name:ident, $position:expr) => {
        Token::Marker(Marker::$name($position.into()))
    };
}

macro_rules! token {
    ($variant:expr) => {
        Token::Token($variant)
    };
}

macro_rules! token_variant {
    ($name:ident, $variant:ident) => {
        TokenVariant::$name($variant)
    };
}

#[derive(Clone, Copy)]
pub struct Lexer<const BWD: usize, const FWD: usize> {
    position_start: (usize, usize),
    position_end: (usize, usize),
}

impl<const BWD: usize, const FWD: usize> Lexer<BWD, FWD> {
    pub fn new() -> Self {
        Self {
            position_start: (1, 0),
            position_end: (1, 0),
        }
    }

    pub fn lex_escape_sequence<'a>(&mut self, input: &mut Peekable<Chars, BWD, FWD>) -> char {
        let mut value = '\0';

        match input.peek() {
            Some(x) => match x {
                '\\' => {
                    input.next();
                    self.position_end.1 += 1;
                    value = '\\';
                }
                '\'' => {
                    input.next();
                    self.position_end.1 += 1;
                    value = '\'';
                }
                '\"' => {
                    input.next();
                    self.position_end.1 += 1;
                    value = '\"';
                }
                ' ' => {
                    input.next();
                    self.position_end.1 += 1;
                    value = ' ';
                }
                'a' => {
                    input.next();
                    self.position_end.1 += 1;
                    value = '\x07'; // bell
                }
                'b' => {
                    input.next();
                    self.position_end.1 += 1;
                    value = '\x08'; // backspace
                }
                'f' => {
                    input.next();
                    self.position_end.1 += 1;
                    value = '\x0c'; // form feed
                }
                'n' => {
                    input.next();
                    self.position_end.1 += 1;
                    value = '\n';
                }
                'r' => {
                    input.next();
                    self.position_end.1 += 1;
                    value = '\r';
                }
                't' => {
                    input.next();
                    self.position_end.1 += 1;
                    value = '\t'; // horizontal tab
                }
                'v' => {
                    input.next();
                    self.position_end.1 += 1;
                    value = '\x0b'; // vertical tab
                }
                x if x.is_numeric() => {
                    // octal escape
                    input.next();
                    self.position_end.1 += 1;
                }
                'x' => {
                    // hex escape
                    input.next();
                    self.position_end.1 += 1;

                    let mut escape = String::new();
                    for _ in 0..2 {
                        if let Some(&cc) = input.peek() {
                            if cc.is_ascii_hexdigit() {
                                escape.push(cc);
                                input.next();
                                self.position_end.1 += 1;
                            }
                        } else if let None = input.peek() {
                            panic!("Invalid hex escape sequence");
                        }
                    }

                    value =
                        char::from_u32(u32::from_str_radix(escape.as_str(), 16).unwrap()).unwrap();
                }
                'u' => {
                    input.next();
                    self.position_end.1 += 1;
                    value = 'u';
                }
                'U' => {
                    input.next();
                    self.position_end.1 += 1;
                    value = 'U';
                }
                'N' => {
                    input.next();
                    self.position_end.1 += 1;
                    value = 'N';
                }
                _ => {
                    panic!("Invalid escape sequence");
                }
            },
            None => panic!("Invalud escape sequence"),
        }

        value
    }

    pub fn lex_string(&mut self, input: &mut Peekable<Chars, BWD, FWD>) -> Vec<Token> {
        let mut tokens = Vec::new();

        self.position_start = self.position_end;

        tokens.push(marker! { BeginString, self.position_start });

        let mut value = String::new();
        'value_loop: loop {
            if let Some(&c) = input.peek() {
                if c == '"' {
                    input.next();
                    self.position_end.1 += 1;
                    break 'value_loop;
                } else if c == '\\' {
                    input.next();
                    self.position_end.1 += 1;

                    value.push(self.lex_escape_sequence(input));

                    if let Some(&cc) = input.peek() {
                        if cc == '"' {
                            input.next();
                            self.position_end.1 += 1;
                            break 'value_loop;
                        }
                    } else if let None = input.peek() {
                        panic!("Unterminated string literal 2");
                    }
                } else {
                    value.push(c);
                    input.next();
                    self.position_end.1 += 1;
                }
            } else if let None = input.peek() {
                panic!("Unterminated string literal");
            }
        }

        tokens.push(token!(token_variant! { String, value }));
        tokens.push(marker! { EndString, self.position_end });

        tokens
    }

    pub fn lex_character(&mut self, input: &mut Peekable<Chars, BWD, FWD>) -> Vec<Token> {
        let mut tokens = Vec::new();

        self.position_start = self.position_end;

        tokens.push(marker! { BeginCharacter, self.position_start });

        let mut value = String::new();
        'value_loop: loop {
            if let Some(&c) = input.peek() {
                if c == '\'' {
                    input.next();
                    self.position_end.1 += 1;
                    break 'value_loop;
                } else if c == '\\' {
                    input.next();
                    self.position_end.1 += 1;

                    value.push(self.lex_escape_sequence(input));

                    if let Some(&cc) = input.peek() {
                        if cc == '\'' {
                            input.next();
                            self.position_end.1 += 1;
                            break 'value_loop;
                        }
                    } else if let None = input.peek() {
                        panic!("Unterminated character literal 2");
                    }
                } else {
                    value.push(c);
                    input.next();
                    self.position_end.1 += 1;
                }
            } else if let None = input.peek() {
                panic!("Unterminated character literal");
            }

            if value.len() > 1 {
                panic!("Invalid character literal, perhaps you meant to use a string?");
            }
        }
        self.position_end.1 += 1;

        let value = value.chars().next().unwrap();

        tokens.push(token!(token_variant! { Character, value }));
        tokens.push(marker! { EndCharacter, self.position_end });

        tokens
    }

    pub fn lex<'a>(&mut self, input: String) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut input = Peekable::<Iter<char>, BWD, FWD>::new(input.chars());

        while let Some(&c) = input.peek() {
            if [' ', '\t'].contains(&c) {
                input.next();
                self.position_end.1 += 1;
                continue;
            }

            if ['\n', '\r'].contains(&c) {
                input.next();
                self.position_end.0 += 1;
                self.position_end.1 = 0;
                continue;
            }

            if c == '"' {
                input.next();
                tokens.append(&mut self.lex_string(&mut input));
            } else if c == '\'' {
                input.next();
                tokens.append(&mut self.lex_character(&mut input));
            }
        }

        tokens.into_iter().collect()
    }
}
