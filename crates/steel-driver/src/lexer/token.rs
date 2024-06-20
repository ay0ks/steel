use super::marker::Marker;

#[derive(Clone, Debug, Default, PartialEq)]
pub enum TokenVariant {
    #[default]
    None,
    Character(char),
    String(String),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Token {
    #[default]
    None,
    Marker(Marker),
    Token(TokenVariant),
}
