mod token;

extern crate nom;
extern crate nom_locate;

use std::fs::read_to_string;
use std::path::Path;

use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::error;
use nom::multi::many0;
use nom::IResult;

use nom_locate::LocatedSpan;

use token::token;

pub type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, PartialEq, Clone)]
pub struct Location(u32, usize);

impl<'a> From<&Span<'a>> for Location {
    fn from(s: &Span<'a>) -> Self {
        Location(s.location_line(), s.get_utf8_column())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LocatedToken(Location, Token);

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Int32Constant(u32),
    Keyword(KeywordType),
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum KeywordType {
    Int,
    Void,
    Return,
}

pub fn tokenize(preprocessed_path: &Path) -> Vec<LocatedToken> {
    let preprocessed = match read_to_string(preprocessed_path) {
        Ok(s) => s,
        Err(err) => panic!(
            "unable to open preprocessed file [{:?}]: {}",
            preprocessed_path, err
        ),
    };

    let span = Span::new(&preprocessed);

    match tokenize_span(span) {
        Ok((rem, toks)) => {
            if !rem.is_empty() {
                panic!("unmatched input: [{}] toks: [{:?}]", rem, toks);
            }
            toks
        }
        Err(err) => panic!("tokenize_str returned an err: {}", err),
    }
}

fn tokenize_span(input: Span) -> IResult<Span, Vec<LocatedToken>> {
    error::context(
        "tokenize_span",
        map(
            all_consuming(many0(alt((map(token, Some), map(multispace1, |_| None))))),
            |t| t.into_iter().flatten().collect(),
        ),
    )(input)
}

#[cfg(test)]
mod test {
    use crate::lexer::tokenize_span;
    use crate::lexer::Span;
    use nom::Err::Error;

    #[test]
    fn int_then_ident_fails() {
        let input = "123\n123abc";
        let span = Span::new(&input);

        match tokenize_span(span) {
            Ok((r, ts)) => println!("unmatched input: [{}] toks: [{:?}]", r, ts),
            Err(Error(err)) => println!(
                "tokenize_str returned an err: {}, line: {} col: {}",
                err,
                err.input.location_line(),
                err.input.get_utf8_column()
            ),
            Err(_) => (),
        }
    }
}
