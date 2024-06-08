mod constant;
mod identifier;
mod keyword;
mod punctuation;
mod operator;

use super::LocatedToken;
use super::Span;

use constant::constant;
use identifier::identifier;
use keyword::keyword;
use punctuation::punctuation;
use operator::operator;

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
    match alt((keyword, identifier))(matched) {
        Ok((_, token)) => Ok((rest, token)),
        Err(e) => Err(e),
    }
}

pub fn token(input: Span) -> IResult<Span, LocatedToken> {
    error::context("token", alt((keyword_or_ident, constant, punctuation, operator)))(input)
}
#[cfg(test)]
mod test {
    use super::keyword_or_ident;
    use crate::lexer::Span;
    use crate::lexer::LocatedToken as LT;
    use crate::lexer::Token::Keyword as K;
    use crate::lexer::KeywordType as KT;
    
    #[test]
    fn keyword_auto() {
        let s = Span::new("auto a");

        let (rest, token) = match keyword_or_ident(s) {
            Ok((r, LT{ loc: _, token: t})) => (r, t),
            Err(err) => panic!("expected matching keyword auto but got err: {err}"),
        };

        assert_eq!(&" a", rest.fragment());
        assert_eq!(K(KT::Auto), token);
    }

    #[test]
    fn keyword_break() {
        let s = Span::new("break a");

        let (rest, token) = match keyword_or_ident(s) {
            Ok((r, LT{ loc: _, token: t})) => (r, t),
            Err(err) => panic!("expected matching keyword auto but got err: {err}"),
        };

        assert_eq!(&" a", rest.fragment());
        assert_eq!(K(KT::Break), token);
    }

    #[test]
    fn keyword_case() {
        let s = Span::new("case a");

        let (rest, token) = match keyword_or_ident(s) {
            Ok((r, LT{ loc: _, token: t})) => (r, t),
            Err(err) => panic!("expected matching keyword auto but got err: {err}"),
        };

        assert_eq!(&" a", rest.fragment());
        assert_eq!(K(KT::Case), token);
    }

    #[test]
    fn keyword_char() {
        let s = Span::new("char a");

        let (rest, token) = match keyword_or_ident(s) {
            Ok((r, LT{ loc: _, token: t})) => (r, t),
            Err(err) => panic!("expected matching keyword auto but got err: {err}"),
        };

        assert_eq!(&" a", rest.fragment());
        assert_eq!(K(KT::Char), token);
    }

    #[test]
    fn keyword_const() {
        let s = Span::new("const a");

        let (rest, token) = match keyword_or_ident(s) {
            Ok((r, LT{ loc: _, token: t})) => (r, t),
            Err(err) => panic!("expected matching keyword auto but got err: {err}"),
        };

        assert_eq!(&" a", rest.fragment());
        assert_eq!(K(KT::Const), token);
    }

    #[test]
    fn keyword_continue() {
        let s = Span::new("continue a");

        let (rest, token) = match keyword_or_ident(s) {
            Ok((r, LT{ loc: _, token: t})) => (r, t),
            Err(err) => panic!("expected matching keyword auto but got err: {err}"),
        };

        assert_eq!(&" a", rest.fragment());
        assert_eq!(K(KT::Continue), token);
    }

    #[test]
    fn keyword_default() {
        let s = Span::new("default a");

        let (rest, token) = match keyword_or_ident(s) {
            Ok((r, LT{ loc: _, token: t})) => (r, t),
            Err(err) => panic!("expected matching keyword auto but got err: {err}"),
        };

        assert_eq!(&" a", rest.fragment());
        assert_eq!(K(KT::Default), token);
    }

    #[test]
    fn keyword_do() {
        let s = Span::new("do a");

        let (rest, token) = match keyword_or_ident(s) {
            Ok((r, LT{ loc: _, token: t})) => (r, t),
            Err(err) => panic!("expected matching keyword auto but got err: {err}"),
        };

        assert_eq!(&" a", rest.fragment());
        assert_eq!(K(KT::Do), token);
    }

    #[test]
    fn keyword_double() {
        let s = Span::new("double a");

        let (rest, token) = match keyword_or_ident(s) {
            Ok((r, LT{ loc: _, token: t})) => (r, t),
            Err(err) => panic!("expected matching keyword auto but got err: {err}"),
        };

        assert_eq!(&" a", rest.fragment());
        assert_eq!(K(KT::Double), token);
    }

    #[test]
    fn keyword_else() {
        let s = Span::new("else a");

        let (rest, token) = match keyword_or_ident(s) {
            Ok((r, LT{ loc: _, token: t})) => (r, t),
            Err(err) => panic!("expected matching keyword auto but got err: {err}"),
        };

        assert_eq!(&" a", rest.fragment());
        assert_eq!(K(KT::Else), token);
    }

    #[test]
    fn keyword_enum() {
        let s = Span::new("enum a");

        let (rest, token) = match keyword_or_ident(s) {
            Ok((r, LT{ loc: _, token: t})) => (r, t),
            Err(err) => panic!("expected matching keyword auto but got err: {err}"),
        };

        assert_eq!(&" a", rest.fragment());
        assert_eq!(K(KT::Enum), token);
    }

    #[test]
    fn keyword_extern() {
        let s = Span::new("extern a");

        let (rest, token) = match keyword_or_ident(s) {
            Ok((r, LT{ loc: _, token: t})) => (r, t),
            Err(err) => panic!("expected matching keyword auto but got err: {err}"),
        };

        assert_eq!(&" a", rest.fragment());
        assert_eq!(K(KT::Extern), token);
    }

    #[test]
    fn keyword_float() {
        let s = Span::new("float a");

        let (rest, token) = match keyword_or_ident(s) {
            Ok((r, LT{ loc: _, token: t})) => (r, t),
            Err(err) => panic!("expected matching keyword auto but got err: {err}"),
        };

        assert_eq!(&" a", rest.fragment());
        assert_eq!(K(KT::Float), token);
    }

    #[test]
    fn keyword_for() {
        let s = Span::new("for a");

        let (rest, token) = match keyword_or_ident(s) {
            Ok((r, LT{ loc: _, token: t})) => (r, t),
            Err(err) => panic!("expected matching keyword auto but got err: {err}"),
        };

        assert_eq!(&" a", rest.fragment());
        assert_eq!(K(KT::For), token);
    }

    #[test]
    fn keyword_goto() {
        let s = Span::new("goto a");

        let (rest, token) = match keyword_or_ident(s) {
            Ok((r, LT{ loc: _, token: t})) => (r, t),
            Err(err) => panic!("expected matching keyword auto but got err: {err}"),
        };

        assert_eq!(&" a", rest.fragment());
        assert_eq!(K(KT::Goto), token);
    }

    #[test]
    fn keyword_if() {
        let s = Span::new("if a");

        let (rest, token) = match keyword_or_ident(s) {
            Ok((r, LT{ loc: _, token: t})) => (r, t),
            Err(err) => panic!("expected matching keyword auto but got err: {err}"),
        };

        assert_eq!(&" a", rest.fragment());
        assert_eq!(K(KT::If), token);
    }

    #[test]
    fn keyword_inline() {
        let s = Span::new("inline a");

        let (rest, token) = match keyword_or_ident(s) {
            Ok((r, LT{ loc: _, token: t})) => (r, t),
            Err(err) => panic!("expected matching keyword auto but got err: {err}"),
        };

        assert_eq!(&" a", rest.fragment());
        assert_eq!(K(KT::Inline), token);
    }

    #[test]
    fn keyword_int() {
        let s = Span::new("int a");

        let (rest, token) = match keyword_or_ident(s) {
            Ok((r, LT{ loc: _, token: t})) => (r, t),
            Err(err) => panic!("expected matching keyword auto but got err: {err}"),
        };

        assert_eq!(&" a", rest.fragment());
        assert_eq!(K(KT::Int), token);
    }

    #[test]
    fn keyword_long() {
        let s = Span::new("long a");

        let (rest, token) = match keyword_or_ident(s) {
            Ok((r, LT{ loc: _, token: t})) => (r, t),
            Err(err) => panic!("expected matching keyword auto but got err: {err}"),
        };

        assert_eq!(&" a", rest.fragment());
        assert_eq!(K(KT::Long), token);
    }
}