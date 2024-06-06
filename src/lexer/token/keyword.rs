use crate::lexer::KeywordType;
use crate::lexer::LocatedToken;
use crate::lexer::Location;
use crate::lexer::Span;
use crate::lexer::Token::Keyword;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::error;
use nom::IResult;

pub fn keyword(input: Span) -> IResult<Span, LocatedToken> {
    error::context(
        "keyword",
        alt((
            map(tag("int"), |_| {
                LocatedToken(Location::from(&input), Keyword(KeywordType::Int))
            }),
            map(tag("void"), |_| {
                LocatedToken(Location::from(&input), Keyword(KeywordType::Void))
            }),
            map(tag("return"), |_| {
                LocatedToken(Location::from(&input), Keyword(KeywordType::Return))
            }),
        )),
    )(input)
}
