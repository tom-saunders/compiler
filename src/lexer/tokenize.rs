use std::fs::read_to_string;
use std::path::Path;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::alphanumeric1;
use nom::character::complete::digit0;
use nom::character::complete::hex_digit1;
use nom::character::complete::multispace1;
use nom::character::complete::oct_digit1;
use nom::character::complete::satisfy;
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::combinator::recognize;
use nom::error;
use nom::multi::many0;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::IResult;

use crate::lexer::token::KeywordType;
use crate::lexer::token::Token;

pub fn tokenize(preprocessed_path: &Path) -> Option<Vec<Token>> {
    let preprocessed = match read_to_string(preprocessed_path) {
        Ok(s) => s,
        Err(err) => panic!(
            "unable to open preprocessed file [{:?}]: {}",
            preprocessed_path, err
        ),
    };

    match tokenize_str(&preprocessed) {
        Ok((rem, toks)) => {
            if !rem.is_empty() {
                panic!("unmatched input: [{}]", rem);
            }
            Some(toks)
        }
        Err(err) => panic!("tokenize_str returned an err: {}", err),
    }
}

fn keyword(input: &str) -> IResult<&str, Token> {
    error::context(
        "keyword",
        all_consuming(alt((
            map(tag("int"), |_| Token::Keyword(KeywordType::Int)),
            map(tag("void"), |_| Token::Keyword(KeywordType::Void)),
            map(tag("return"), |_| Token::Keyword(KeywordType::Return)),
        ))),
    )(input)
}

fn identifier(input: &str) -> IResult<&str, Token> {
    let ident = Token::Identifier(String::from(input));
    Ok(("", ident))
}

fn keyword_or_ident(input: &str) -> IResult<&str, Token> {
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

fn is_nonzero_digit(c: char) -> bool {
    ('1'..='9').contains(&c)
}

fn int_constant_dec(input: &str) -> IResult<&str, Token> {
    let (rest, matched) = error::context(
        "int_constant_dec",
        recognize(pair(satisfy(is_nonzero_digit), digit0)),
    )(input)?;
    match matched.parse::<u128>() {
        Ok(value) => Ok((rest, Token::IntConstant(value))),
        Err(err) => panic!("have nonparsable dec value [{}]: {}", matched, err),
    }
}

fn int_constant_hex(input: &str) -> IResult<&str, Token> {
    let (rest, matched) =
        error::context("int_constant_hex", preceded(tag("0x"), hex_digit1))(input)?;
    match u128::from_str_radix(matched, 16) {
        Ok(value) => Ok((rest, Token::IntConstant(value))),
        Err(err) => panic!("have nonparsable hex value [{}]: {}", matched, err),
    }
}

fn int_constant_oct(input: &str) -> IResult<&str, Token> {
    let (rest, matched) = error::context(
        "int_constant_oct",
        alt((preceded(tag("0"), oct_digit1), tag("0"))),
    )(input)?;
    match u128::from_str_radix(matched, 8) {
        Ok(value) => Ok((rest, Token::IntConstant(value))),
        Err(err) => panic!("have nonparsable oct value [{}]: {}", matched, err),
    }
}

fn int_constant(input: &str) -> IResult<&str, Token> {
    error::context(
        "int_constant",
        alt((int_constant_dec, int_constant_hex, int_constant_oct)),
    )(input)
}

fn constant(input: &str) -> IResult<&str, Token> {
    error::context("constant", int_constant)(input)
}

fn punctuation(input: &str) -> IResult<&str, Token> {
    error::context(
        "punctuation",
        all_consuming(alt((
            map(tag("("), |_| Token::OpenParen),
            map(tag(")"), |_| Token::CloseParen),
            map(tag("{"), |_| Token::OpenBrace),
            map(tag("}"), |_| Token::CloseBrace),
            map(tag(";"), |_| Token::Semicolon),
        ))),
    )(input)
}

fn token(input: &str) -> IResult<&str, Token> {
    error::context("token", alt((keyword_or_ident, constant, punctuation)))(input)
}

fn tokenize_str(input: &str) -> IResult<&str, Vec<Token>> {
    error::context(
        "tokenize",
        map(
            many0(alt((map(token, Some), map(multispace1, |_| None)))),
            |t| t.into_iter().flatten().collect(),
        ),
    )(input)
}

#[cfg(test)]
mod test {
    use crate::lexer::token::KeywordType;
    use crate::lexer::token::Token;
    use crate::lexer::tokenize::tokenize_str;

    #[test]
    fn lexes_identifier() {
        let ident = "ident";
        let (unmatched, tokens) = tokenize_str(ident).expect("expected tokens to be returned");

        let exp: Vec<Token> = vec![Token::Identifier(String::from(ident))];

        assert_eq!(exp, tokens);
        assert!(unmatched.is_empty())
    }

    #[test]
    fn lexes_int_const() {
        let int_const = 1;
        let int_str = int_const.to_string();
        let (unmatched, tokens) = tokenize_str(&int_str).expect("expected tokens to be returned");

        let exp: Vec<Token> = vec![Token::IntConstant(int_const)];

        assert_eq!(exp, tokens);
        assert!(unmatched.is_empty())
    }

    #[test]
    fn lexes_keyword_int() {
        let (unmatched, tokens) = tokenize_str("int").expect("expected tokens to be returned");

        let exp: Vec<Token> = vec![Token::Keyword(KeywordType::Int)];

        assert_eq!(exp, tokens);
        assert!(unmatched.is_empty())
    }

    #[test]
    fn lexes_keyword_return() {
        let (unmatched, tokens) = tokenize_str("return").expect("expected tokens to be returned");

        let exp: Vec<Token> = vec![Token::Keyword(KeywordType::Return)];

        assert_eq!(exp, tokens);
        assert!(unmatched.is_empty())
    }

    #[test]
    fn lexes_keyword_void() {
        let (unmatched, tokens) = tokenize_str("void").expect("expected tokens to be returned");

        let exp: Vec<Token> = vec![Token::Keyword(KeywordType::Void)];

        assert_eq!(exp, tokens);
        assert!(unmatched.is_empty())
    }

    #[test]
    fn lexes_open_paren() {
        let (unmatched, tokens) = tokenize_str("(").expect("expected tokens to be returned");

        let exp: Vec<Token> = vec![Token::OpenParen];

        assert_eq!(exp, tokens);
        assert!(unmatched.is_empty())
    }

    #[test]
    fn lexes_close_paren() {
        let (unmatched, tokens) = tokenize_str(")").expect("expected tokens to be returned");

        let exp: Vec<Token> = vec![Token::CloseParen];

        assert_eq!(exp, tokens);
        assert!(unmatched.is_empty())
    }

    #[test]
    fn lexes_open_brace() {
        let (unmatched, tokens) = tokenize_str("{").expect("expected tokens to be returned");

        let exp: Vec<Token> = vec![Token::OpenBrace];

        assert_eq!(exp, tokens);
        assert!(unmatched.is_empty())
    }

    #[test]
    fn lexes_close_brace() {
        let (unmatched, tokens) = tokenize_str("}").expect("expected tokens to be returned");

        let exp: Vec<Token> = vec![Token::CloseBrace];

        assert_eq!(exp, tokens);
        assert!(unmatched.is_empty())
    }

    #[test]
    fn lexes_semicolon() {
        let (unmatched, tokens) = tokenize_str(";").expect("expected tokens to be returned");

        let exp: Vec<Token> = vec![Token::Semicolon];

        assert_eq!(exp, tokens);
        assert!(unmatched.is_empty())
    }

    #[test]
    fn no_lexes_invalid_at() {
        let (unmatched, tokens) = tokenize_str("0@1").expect("expected tokens to be returned");

        let exp: Vec<Token> = vec![Token::IntConstant(0)];

        assert_eq!(exp, tokens);
        assert!(!unmatched.is_empty())
    }

    #[test]
    fn no_lexes_invalid_backtick() {
        let (unmatched, tokens) = tokenize_str("`").expect("expected tokens to be returned");

        assert!(tokens.len() == 0);
        assert!(!unmatched.is_empty())
    }

    #[test]
    fn no_lexes_invalid_backslash() {
        let (unmatched, tokens) = tokenize_str("\\").expect("expected tokens to be returned");

        assert!(tokens.len() == 0);
        assert!(!unmatched.is_empty())
    }

    #[test]
    fn lexes_identifier_ints_not_keyword_int_and_identifier_s() {
        let ints = "ints";
        let (unmatched, tokens) = tokenize_str(ints).expect("expected tokens to be returned");

        let exp: Vec<Token> = vec![Token::Identifier(String::from(ints))];

        assert_eq!(exp, tokens);
        assert!(unmatched.is_empty())
    }

    #[test]
    fn lexes_identifiers_omits_whitespace() {
        let ints = "inta intb intc";
        let (unmatched, tokens) = tokenize_str(ints).expect("expected tokens to be returned");

        let exp: Vec<Token> = vec![
            Token::Identifier(String::from("inta")),
            Token::Identifier(String::from("intb")),
            Token::Identifier(String::from("intc")),
        ];

        assert_eq!(exp, tokens);
        assert!(unmatched.is_empty())
    }

    #[test]
    fn lexes_whitespace_empty() {
        let (unmatched, tokens) = tokenize_str(" ").expect("expected tokens to be returned");

        let exp: Vec<Token> = vec![];

        assert_eq!(exp, tokens);
        assert!(unmatched.is_empty())
    }

    #[test]
    fn lexes_octal_hex_constants() {
        let (unmatched, tokens) = tokenize_str(" 01 02 0 0x0 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF")
            .expect("expected tokens to be returned");

        let exp: Vec<Token> = vec![
            Token::IntConstant(1),
            Token::IntConstant(2),
            Token::IntConstant(0),
            Token::IntConstant(0),
            Token::IntConstant(0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF),
        ];

        assert_eq!(exp, tokens);
        assert!(unmatched.is_empty())
    }
}
