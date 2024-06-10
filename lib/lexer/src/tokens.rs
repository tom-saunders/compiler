#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    Amp,
    AmpAmp,
    AmpEql,
    Bang,
    BangEql,
    Caret,
    CaretEql,
    Comma,
    Dot,
    Ellipsis,
    Unknown(&'a str),
}