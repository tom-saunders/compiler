#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    Dot,
    Ellipsis,
    Unknown(&'a str),
}