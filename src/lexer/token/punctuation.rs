use crate::lexer::LocatedToken;
use crate::lexer::Location;
use crate::lexer::Span;
use crate::lexer::Token::{CloseBrace, CloseParen, OpenBrace, OpenParen, Semicolon};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::error;
use nom::IResult;

pub fn punctuation(input: Span) -> IResult<Span, LocatedToken> {
    error::context(
        "punctuation",
        alt((
            map(tag("("), |_| {
                LocatedToken::of(Location::from(&input), OpenParen)
            }),
            map(tag(")"), |_| {
                LocatedToken::of(Location::from(&input), CloseParen)
            }),
            map(tag("{"), |_| {
                LocatedToken::of(Location::from(&input), OpenBrace)
            }),
            map(tag("}"), |_| {
                LocatedToken::of(Location::from(&input), CloseBrace)
            }),
            map(tag(";"), |_| {
                LocatedToken::of(Location::from(&input), Semicolon)
            }),
        )),
    )(input)
}
