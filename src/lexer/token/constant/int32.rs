use crate::lexer::LocatedToken;
use crate::lexer::Location;
use crate::lexer::Span;
use crate::lexer::Token::Int32Constant;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::character::complete::digit0;
use nom::character::complete::hex_digit1;
use nom::character::complete::oct_digit1;
use nom::character::complete::satisfy;
use nom::combinator::not;
use nom::combinator::peek;
use nom::combinator::recognize;
use nom::error;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::IResult;

fn is_nonzero_digit(c: char) -> bool {
    ('1'..='9').contains(&c)
}

fn int32_constant_dec(input: Span) -> IResult<Span, LocatedToken> {
    let (rest, matched) = error::context(
        "int_constant_dec",
        recognize(terminated(
            pair(satisfy(is_nonzero_digit), digit0),
            peek(not(alphanumeric1)),
        )),
    )(input)?;
    match matched.fragment().parse::<u32>() {
        Ok(value) => Ok((
            rest,
            LocatedToken(Location::from(&matched), Int32Constant(value)),
        )),
        Err(err) => panic!("have nonparsable dec value [{}]: {}", matched, err),
    }
}

fn int32_constant_hex(input: Span) -> IResult<Span, LocatedToken> {
    let (rest, matched) = error::context(
        "int_constant_hex",
        delimited(tag("0x"), hex_digit1, peek(not(alphanumeric1))),
    )(input)?;
    match u32::from_str_radix(matched.fragment(), 16) {
        Ok(value) => Ok((
            rest,
            LocatedToken(Location::from(&matched), Int32Constant(value)),
        )),
        Err(err) => panic!("have nonparsable hex value [{}]: {}", matched, err),
    }
}

fn int32_constant_oct(input: Span) -> IResult<Span, LocatedToken> {
    let (rest, matched) = error::context(
        "int_constant_oct",
        terminated(
            alt((preceded(tag("0"), oct_digit1), tag("0"))),
            peek(not(alphanumeric1)),
        ),
    )(input)?;
    match u32::from_str_radix(matched.fragment(), 8) {
        Ok(value) => Ok((
            rest,
            LocatedToken(Location::from(&matched), Int32Constant(value)),
        )),
        Err(err) => panic!("have nonparsable oct value [{}]: {}", matched, err),
    }
}

pub fn int32_constant(input: Span) -> IResult<Span, LocatedToken> {
    error::context(
        "int32_constant",
        alt((int32_constant_dec, int32_constant_hex, int32_constant_oct)),
    )(input)
}
