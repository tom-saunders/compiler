use crate::lexer::LocatedToken as LT;
use crate::lexer::Location as L;
use crate::lexer::Span;
use crate::lexer::Token::Punctuation as P;
use crate::lexer::PunctuationType as PT;


use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::combinator::not;
use nom::combinator::peek;
use nom::sequence::terminated;
use nom::error;
use nom::IResult;

pub fn punctuation(input: Span) -> IResult<Span, LT> {
    error::context(
        "punctuation",
        alt((
            map(tag("("), |m| LT::of(L::from(&m), P(PT::OpenParen))),
            map(tag(")"), |m| LT::of(L::from(&m), P(PT::CloseParen))),
            map(tag("{"), |m| LT::of(L::from(&m), P(PT::OpenBrace))),
            map(tag("<%"), |m| LT::of(L::from(&m), P(PT::OpenBrace))),
            map(tag("}"), |m| LT::of(L::from(&m), P(PT::CloseBrace))),
            map(tag("%>"), |m| LT::of(L::from(&m), P(PT::CloseBrace))),
            map(tag(";"), |m| LT::of(L::from(&m), P(PT::Semicolon))),
            map(tag("["), |m| LT::of(L::from(&m), P(PT::OpenSquare))),
            map(tag("<:"), |m| LT::of(L::from(&m), P(PT::OpenSquare))),
            map(tag("]"), |m| LT::of(L::from(&m), P(PT::CloseSquare))),
            map(tag(":>"), |m| LT::of(L::from(&m), P(PT::CloseSquare))),
            map(tag(","), |m| LT::of(L::from(&m), P(PT::Comma))),
            map(terminated(tag(":"), peek(not(tag(">")))), |m| LT::of(L::from(&m), P(PT::Colon))),
                
        )),
    )(input)
}
