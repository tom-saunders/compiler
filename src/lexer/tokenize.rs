use std::fs::read_to_string;
use std::path::Path;

use crate::lexer::token::Token;

pub fn tokenize(preprocessed_path: &Path) -> Option<Vec<Token>> {
    let preprocessed = match read_to_string(preprocessed_path) {
        Ok(s) => s,
        Err(err) => panic!(
            "unable to open preprocessed file [{:?}]: {}",
            preprocessed_path, err
        ),
    };

    tokenize_str(&preprocessed)
}

fn tokenize_str(_input: &str) -> Option<Vec<Token>> {
    todo!();
}

#[cfg(test)]
mod test {
    use crate::lexer::token::Token;
    use crate::lexer::tokenize::tokenize_str;

    #[test]
    fn lexes_identifier() {
        let ident = String::from("ident");
        let tokens = tokenize_str(&ident).expect("expected tokens to be returned");

        let exp: Vec<Token> = vec![Token::Identifier(ident)];

        assert_eq!(exp, tokens);
    }

    #[test]
    fn lexes_int_const() {
        let int_const = String::from("1");
        let tokens = tokenize_str(&int_const).expect("expected tokens to be returned");

        let exp: Vec<Token> = vec![Token::IntConstant(int_const)];

        assert_eq!(exp, tokens);
    }

    #[test]
    fn lexes_keyword_int() {
        let tokens = tokenize_str("int").expect("expected tokens to be returned");

        let exp: Vec<Token> = vec![Token::KeywordInt];

        assert_eq!(exp, tokens);
    }

    #[test]
    fn lexes_keyword_return() {
        let tokens = tokenize_str("return").expect("expected tokens to be returned");

        let exp: Vec<Token> = vec![Token::KeywordReturn];

        assert_eq!(exp, tokens);
    }

    #[test]
    fn lexes_keyword_void() {
        let tokens = tokenize_str("void").expect("expected tokens to be returned");

        let exp: Vec<Token> = vec![Token::KeywordVoid];

        assert_eq!(exp, tokens);
    }

    #[test]
    fn lexes_open_paren() {
        let tokens = tokenize_str("(").expect("expected tokens to be returned");

        let exp: Vec<Token> = vec![Token::OpenParen];

        assert_eq!(exp, tokens);
    }

    #[test]
    fn lexes_close_paren() {
        let tokens = tokenize_str(")").expect("expected tokens to be returned");

        let exp: Vec<Token> = vec![Token::CloseParen];

        assert_eq!(exp, tokens);
    }

    #[test]
    fn lexes_open_brace() {
        let tokens = tokenize_str("{").expect("expected tokens to be returned");

        let exp: Vec<Token> = vec![Token::OpenBrace];

        assert_eq!(exp, tokens);
    }

    #[test]
    fn lexes_close_brace() {
        let tokens = tokenize_str("}").expect("expected tokens to be returned");

        let exp: Vec<Token> = vec![Token::CloseBrace];

        assert_eq!(exp, tokens);
    }

    #[test]
    fn lexes_semicolon() {
        let tokens = tokenize_str(";").expect("expected tokens to be returned");

        let exp: Vec<Token> = vec![Token::Semicolon];

        assert_eq!(exp, tokens);
    }

    #[test]
    fn no_lexes_invalid_at() {
        let tokens = tokenize_str("0@1");

        assert!(tokens.is_none());
    }

    #[test]
    fn no_lexes_invalid_backtick() {
        let tokens = tokenize_str("`");

        assert!(tokens.is_none());
    }

    #[test]
    fn no_lexes_invalid_backslash() {
        let tokens = tokenize_str("\\");

        assert!(tokens.is_none());
    }

    #[test]
    fn lexes_identifier_ints_not_keyword_int_and_identifier_s() {
        let ints = String::from("ints");
        let tokens = tokenize_str(&ints).expect("expected tokens to be returned");

        let exp: Vec<Token> = vec![Token::Identifier(ints)];

        assert_eq!(exp, tokens);
    }

    #[test]
    fn lexes_whitespace_empty() {
        let tokens = tokenize_str(" ").expect("expected tokens to be returned");

        let exp: Vec<Token> = vec![];

        assert_eq!(exp, tokens);
    }
}
