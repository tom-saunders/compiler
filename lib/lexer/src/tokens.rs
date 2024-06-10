#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    Amp,
    AmpAmp,
    AmpEql,
    Bang,
    BangEql,
    Caret,
    CaretEql,
    Colon,
    Comma,
    Dot,
    Ellipsis,
    RSqu,
    Unknown(&'a str),
}