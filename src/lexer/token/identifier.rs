use crate::lexer::LocatedToken;
use crate::lexer::Location;
use crate::lexer::Span;
use crate::lexer::Token::Identifier;

use nom::combinator::map;
use nom::combinator::rest;
use nom::error;
use nom::IResult;

pub fn identifier(input: Span) -> IResult<Span, LocatedToken> {
    error::context(
        "identifier",
        map(rest, |_| {
            LocatedToken::of(
                Location::from(&input),
                Identifier(input.fragment().to_string()),
            )
        }),
    )(input)
}
