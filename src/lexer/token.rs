mod constant;
mod identifier;
mod keyword;
mod punctuation;

use super::LocatedToken;
use super::Span;

use constant::constant;
use identifier::identifier;
use keyword::keyword;
use punctuation::punctuation;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::alphanumeric1;
use nom::combinator::all_consuming;
use nom::combinator::recognize;
use nom::error;
use nom::multi::many0;
use nom::sequence::pair;
use nom::IResult;

fn keyword_or_ident(input: Span) -> IResult<Span, LocatedToken> {
    let (rest, matched) = error::context(
        "keyword_or_ident",
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
    )(input)?;
    match all_consuming(alt((keyword, identifier)))(matched) {
        Ok((_, token)) => Ok((rest, token)),
        Err(e) => Err(e),
    }
}

pub fn token(input: Span) -> IResult<Span, LocatedToken> {
    error::context("token", alt((keyword_or_ident, constant, punctuation)))(input)
}
