mod int32;

use crate::lexer::LocatedToken;
use crate::lexer::Span;

use int32::int32_constant;

use nom::error;
use nom::IResult;

pub fn constant(input: Span) -> IResult<Span, LocatedToken> {
    error::context("constant", int32_constant)(input)
}
