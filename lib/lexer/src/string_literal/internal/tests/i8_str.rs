use super::*;

use crate::string_literal::string_literal_impl;
use crate::text::char_escape::char_esc_impl;
use crate::text::hex_escape::hex_esc_impl;
use crate::text::oct_escape::oct_esc_impl;
use crate::text::text_state_impl_i8;
use crate::text::univ_esc::univ_esc_impl;
use crate::Token;
use crate::Token::StringLit;
use crate::Token::Unknown;

fn actual(input: &str) -> (Token, usize) {
    let location = Box::new(TestLocation);
    let text = text_state_impl_i8(input.chars().peekable());

    let hex_escape = hex_esc_impl::<i8>(location.as_ref(), text.as_ref());
    let oct_escape = oct_esc_impl::<i8>(location.as_ref(), text.as_ref());
    let univ_escape = univ_esc_impl::<i8>(location.as_ref(), text.as_ref());

    let char_escape = char_esc_impl(
        location.as_ref(),
        text.as_ref(),
        hex_escape.as_ref(),
        oct_escape.as_ref(),
        univ_escape.as_ref(),
    );

    let string_literal = string_literal_impl(location.as_ref(), text.as_ref(), char_escape.as_ref(),);
    let token = string_literal.consume_string_literal();
    let n_chars = text.chars_consumed();

    (token, n_chars)
}

fn unknown_and_actual(input: &str) -> (Token, Token, usize) {
    let unknown = Unknown(input.to_string());
    let (actual, n_size) = actual(input);

    (unknown, actual, n_size)
}

#[test]
fn test_i8_string_literal_empty() {
    let exp_token = StringLit(vec![]);
    let exp_sz = 2;
    let (act_token, act_sz) = actual("\"\"");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_unterminated_eoi() {
    let (exp_token, act_token, act_sz) = unknown_and_actual("\"");
    let exp_sz = 1;

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_unterminated_eol() {
    let (exp_token, act_token, act_sz) = unknown_and_actual("\"\n");
    let exp_sz = 1;

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_unterminated_space_eoi() {
    let (exp_token, act_token, act_sz) = unknown_and_actual("\"  ");
    let exp_sz = 3;

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_unterminated_space_eol() {
    let (exp_token, act_token, act_sz) = unknown_and_actual("\"  \n");
    let exp_sz = 3;

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}