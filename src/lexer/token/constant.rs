mod int;

use crate::lexer::LocatedToken;
use crate::lexer::Span;

use int::int_constant;

use nom::error;
use nom::IResult;

pub fn constant(input: Span) -> IResult<Span, LocatedToken> {
    error::context("constant", int_constant)(input)
}
