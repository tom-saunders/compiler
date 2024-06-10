#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    CharLiteral(u8),
    Unknown(&'a str),
}