#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    Amp,
    AmpAmp,
    AmpEql,
    Bang,
    BangEql,
    Dot,
    Ellipsis,
    Unknown(&'a str),
}