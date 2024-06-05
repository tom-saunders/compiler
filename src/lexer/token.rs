use std::ops::Deref;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    IntConstant(String),
    KeywordInt,
    KeywordVoid,
    KeywordReturn,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
}

impl Deref for Token {
    type Target = Token;

    fn deref(&self) -> &Self::Target {
        self
    }
}
