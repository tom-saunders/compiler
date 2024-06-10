#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    Amp,
    AmpAmp,
    AmpEql,
    Dot,
    Ellipsis,
    Unknown(&'a str),
}