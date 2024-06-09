
use crate::lexer::LocatedToken;
use crate::lexer::Location;
use crate::lexer::Span;
use crate::lexer::Token;
use crate::lexer::Token::Literal;
use crate::lexer::LiteralType;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::character::complete::alphanumeric1;
use nom::character::complete::digit0;
use nom::character::complete::hex_digit1;
use nom::character::complete::oct_digit0;
use nom::character::complete::one_of;
use nom::character::complete::satisfy;
use nom::combinator::consumed;
use nom::combinator::map;
use nom::combinator::not;
use nom::combinator::opt;
use nom::combinator::peek;
use nom::combinator::recognize;
use nom::error;
use nom::error::Error;
use nom::error::ErrorKind;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::IResult;

pub fn chr_escape(input: Span) -> IResult<Span, u8> {
    todo!("map escaped chars")
}

fn chr_or_escape(input: Span) -> IResult<Span, u8> {
    let (rest, (matched, value)) = error::context(
        "chr_or_escape",
        consumed(map(take(1usize), |m: Span| m.fragment().as_bytes()[0])),
    )(input)?;
    match value {
        0x0a => {
            println!("found literal LineFeed in char literal at line {} col {}", input.location_line(), input.get_column());
            Err(nom::Err::Error(Error::new(input, ErrorKind::Tag)))
        },
        0x0d => {
            println!("found literal CarriageReturn in char literal at line {} col {}", input.location_line(), input.get_column());
            Err(nom::Err::Error(Error::new(input, ErrorKind::Tag)))
        },
        0x27 => {
            println!("expected value before closing ' character at line {} col {}", input.location_line(), input.get_column());
            Err(nom::Err::Error(Error::new(input, ErrorKind::Tag)))
        }
        0x5c => chr_escape(input),
        o => Ok((rest, o)),
    }
}

pub fn chr_constant(input: Span) -> IResult<Span, LocatedToken> {
	error::context(
        "chr_constant",
        delimited(
            tag("'"),
            map(consumed(chr_or_escape), |(m, c)| LocatedToken::of(Location::from(&m), Literal(LiteralType::Char(c)))),
            tag("'")
        )
    )(input)
}

#[cfg(test)]
mod test {
    use crate::lexer::Span;
    use super::chr_or_escape;

    #[test]
    fn test_chr_noescape() {
        let s = Span::from("a ");

        match chr_or_escape(s) {
            Ok((r, u)) => panic!("matched: ({r}, {u})"),
            Err(e) => panic!("Err: ({e})"),
        }
    }

    #[test]
    fn test_chr_escape() {
        let s = Span::from("\\t ");

        match chr_or_escape(s) {
            Ok((r, u)) => panic!("matched: ({r}, {u})"),
            Err(e) => panic!("Err: ({e})"),
        }
    }

    #[test]
    fn test_chr_unescaped() {
        let s = Span::from("\n ");

        match chr_or_escape(s) {
            Ok((r, u)) => panic!("matched: ({r}, {u})"),
            Err(e) => panic!("Err: ({e})"),
        }
    }
}