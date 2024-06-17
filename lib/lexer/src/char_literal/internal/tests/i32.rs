use super::*;

use crate::char_escape::char_esc_impl;
use crate::hex_escape::hex_esc_impl;
use crate::oct_escape::oct_esc_impl;
use crate::text::text_state_impl_i32;
use crate::universal_char::univ_esc_impl;

use crate::Token;
use crate::CharLit_U;

fn actual_i32(input: &str) -> Token {
    let location = Box::new(TestLocation);
    let text = text_state_impl_i32(input.chars().peekable());

    let hex_escape = hex_esc_impl::<i32>(location.as_ref(), text.as_ref());
    let oct_escape = oct_esc_impl::<i32>(location.as_ref(), text.as_ref());
    let univ_escape = univ_esc_impl::<i32>(location.as_ref(), text.as_ref());

    let char_escape = char_esc_impl(
        location.as_ref(),
        text.as_ref(),
        hex_escape.as_ref(),
        oct_escape.as_ref(),
        univ_escape.as_ref(),
    );

    let char_lit = char_literal_impl(location.as_ref(), text.as_ref(), char_escape.as_ref());
    char_lit.consume_char_literal()
}

fn unknown_and_actual_i32(input: &str) -> (Token, Token) {
    let expected = Token::Unknown(input.to_string());
    let actual = actual_i32(input);

    (expected, actual)
}

macro_rules! exp_and_actual_i32 {
    ($v: literal, $input: literal) => {
        (CharLit_U($v as i32), actual_i32($input))
    };
}

#[test]
fn test_i32_char_literal_empty() {
    let (expected, actual) = unknown_and_actual_i32("''");

    assert_eq!(expected, actual)
}

#[test]
fn test_i32_char_literals_unterminated_eoi() {
    let (expected, actual) = unknown_and_actual_i32("'");

    assert_eq!(expected, actual)
}

#[test]
fn test_i32_char_literals_unterminated_eol() {
    let expected = Token::Unknown("'".to_string());
    let actual = actual_i32("'\n");

    assert_eq!(expected, actual)
}

#[test]
fn test_i32_char_literal_1_1byte_utf8_char() {
    let (expected, actual) = exp_and_actual_i32!('a', "'a'");
    let CharLit_U(e) = expected else {
        panic!()
    };
    let CharLit_U(a) = actual else {
        panic!()
    };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i32_char_literal_2_1byte_utf8_char() {
    let (expected, actual) = exp_and_actual_i32!('b', "'ab'");
    let CharLit_U(e) = expected else {
        panic!()
    };
    let CharLit_U(a) = actual else {
        panic!()
    };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i32_char_literal_3_1byte_utf8_char() {
    let (expected, actual) = exp_and_actual_i32!('c', "'abc'");
    let CharLit_U(e) = expected else {
        panic!()
    };
    let CharLit_U(a) = actual else {
        panic!()
    };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i32_char_literal_4_1byte_utf8_char() {
    let (expected, actual) = exp_and_actual_i32!('d', "'abcd'");
    let CharLit_U(e) = expected else {
        panic!()
    };
    let CharLit_U(a) = actual else {
        panic!()
    };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i32_char_literal_5_1byte_utf8_char() {
    let (expected, actual) = exp_and_actual_i32!('e', "'abcde'");
    let CharLit_U(e) = expected else {
        panic!()
    };
    let CharLit_U(a) = actual else {
        panic!()
    };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i32_char_literal_1_2byte_utf8_char() {
    let (expected, actual) = exp_and_actual_i32!(0xa0, "'\u{00a0}'");
    let CharLit_U(e) = expected else {
        panic!()
    };
    let CharLit_U(a) = actual else {
        panic!()
    };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i32_char_literal_2_2byte_utf8_char() {
    let (expected, actual) = exp_and_actual_i32!(0xa1, "'\u{00a0}\u{00a1}'");
    let CharLit_U(e) = expected else {
        panic!()
    };
    let CharLit_U(a) = actual else {
        panic!()
    };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i32_char_literal_3_2byte_utf8_char() {
    let (expected, actual) = exp_and_actual_i32!(0xa2, "'\u{00a0}\u{00a1}\u{00a2}'");
    let CharLit_U(e) = expected else {
        panic!()
    };
    let CharLit_U(a) = actual else {
        panic!()
    };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i32_char_literal_1_3byte_utf8_char() {
    let (expected, actual) = exp_and_actual_i32!(0x0800, "'\u{0800}'");
    let CharLit_U(e) = expected else {
        panic!()
    };
    let CharLit_U(a) = actual else {
        panic!()
    };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i32_char_literal_2_3byte_utf8_char() {
    let (expected, actual) = exp_and_actual_i32!(0x0801, "'\u{0800}\u{0801}'");
    let CharLit_U(e) = expected else {
        panic!()
    };
    let CharLit_U(a) = actual else {
        panic!()
    };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i32_char_literal_1_4byte_utf8_char() {
    let (expected, actual) = exp_and_actual_i32!(0x010000, "'\u{010000}'");
    let CharLit_U(e) = expected else {
        panic!()
    };
    let CharLit_U(a) = actual else {
        panic!()
    };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i32_char_literal_2_4byte_utf8_char() {
    let (expected, actual) = exp_and_actual_i32!(0x010001, "'\u{010000}\u{010001}'");
    let CharLit_U(e) = expected else {
        panic!()
    };
    let CharLit_U(a) = actual else {
        panic!()
    };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}
