use core::{panic, slice::Iter};
extern crate unicode_names2;
use peekable_fwd_bwd::Peekable;
use std::str::Chars;
use unicode_names2::character;

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

#[derive(Clone)]
pub struct Lexer<const BWD: usize, const FWD: usize> {
    lines: Vec<String>,
    position_start: (usize, usize),
    position_end: (usize, usize),
}

impl<const BWD: usize, const FWD: usize> Lexer<BWD, FWD> {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            position_start: (1, 1),
            position_end: (1, 1),
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
                'u' => {
                    input.next();
                    self.position_end.1 += 1;

                    let mut escape = String::new();
                    if let Some(&c) = input.peek() {
                        if c == '{' {
                            input.next();
                            self.position_end.1 += 1;

                            'value_loop: loop {
                                if let Some(&c) = input.peek() {
                                    if c == '}' {
                                        input.next();
                                        self.position_end.1 += 1;
                                        break 'value_loop;
                                    } else if c.is_digit(16) {
                                        escape.push(c);
                                        input.next();
                                        self.position_end.1 += 1;
                                    } else {
                                        panic!("Invalid unicode escape sequence");
                                    }
                                } else if let None = input.peek() {
                                    panic!("Unterminated unicode escape sequence");
                                }
                            }
                        } else {
                            panic!("Invalid unicode escape sequence");
                        }
                    }

                    value =
                        char::from_u32(u32::from_str_radix(escape.as_str(), 16).unwrap()).unwrap();
                }
                'U' => {
                    input.next();
                    self.position_end.1 += 1;

                    let mut escape = String::new();
                    if let Some(&c) = input.peek() {
                        if c == '{' {
                            input.next();
                            self.position_end.1 += 1;

                            'value_loop: loop {
                                if let Some(&c) = input.peek() {
                                    if c == '}' {
                                        input.next();
                                        self.position_end.1 += 1;
                                        break 'value_loop;
                                    } else {
                                        escape.push(c);
                                        self.position_end.1 += 1;
                                        input.next();
                                    }
                                } else if let None = input.peek() {
                                    panic!("Unterminated unicode escape sequence");
                                }
                            }
                        } else {
                            panic!("Invalid unicode escape sequence");
                        }
                    }

                    value = match character(escape.as_str()) {
                        Some(x) => x,
                        None => panic!("Invalid unicode escape sequence"),
                    };
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
                    self.position_end.1 += 1;
                    input.next();
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
        self.lines = input.lines().map(|x| x.to_string()).collect();

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
                self.position_end.1 = 1;
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
