use crate::parser::PeekBuffer;

#[derive(Clone, Debug, Default)]
pub enum Token {
    EOL,
    EOS,
    #[default]
    EOF,

    Mnemonic(String),
    Label(String),
    Register(String),
    Number(i64),
    Float(f64),
}
