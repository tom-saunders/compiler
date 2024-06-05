use std::ops::Deref;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    IntConstant(u128),
    Keyword(KeywordType),
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

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum KeywordType {
    Int,
    Void,
    Return,
}
