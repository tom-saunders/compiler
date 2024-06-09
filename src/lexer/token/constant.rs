mod chr;
mod int;

use crate::lexer::LocatedToken;
use crate::lexer::Span;

use int::int_constant;
use chr::chr_constant;

use nom::branch::alt;
use nom::error;
use nom::IResult;

pub fn constant(input: Span) -> IResult<Span, LocatedToken> {
    error::context("constant", alt((int_constant, chr_constant)))(input)
}
