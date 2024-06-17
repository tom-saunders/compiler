use super::*;

use crate::char_escape::char_esc_impl;
use crate::hex_escape::hex_esc_impl;
use crate::oct_escape::oct_esc_impl;
use crate::text::text_state_impl_i16;
use crate::universal_char::univ_esc_impl;

use crate::Token;
use crate::CharLit_u;

fn actual(input: &str) -> Token {
    let location = Box::new(TestLocation);
    let text = text_state_impl_i16(input.chars().peekable());

    let hex_escape = hex_esc_impl::<i16>(location.as_ref(), text.as_ref());
    let oct_escape = oct_esc_impl::<i16>(location.as_ref(), text.as_ref());
    let univ_escape = univ_esc_impl::<i16>(location.as_ref(), text.as_ref());

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

fn unknown_and_actual(input: &str) -> (Token, Token) {
    let expected = Token::Unknown(input.to_string());
    let actual = actual(input);

    (expected, actual)
}

macro_rules! exp_and_actual {
    ($v: literal, $input: literal) => {
        (CharLit_u($v as i32), actual($input))
    };
}

fn unwrap_values(left: &Token, right: &Token) -> (i32, i32) {
    let CharLit_u(l) = left else {panic!()};
    let CharLit_u(r) = right else {panic!()};
    (*l, *r)
}

#[test]
fn test_i16_char_literal_empty() {
    let (expected, actual) = unknown_and_actual("''");

    assert_eq!(expected, actual)
}

#[test]
fn test_i16_char_literals_unterminated_eoi() {
    let (expected, actual) = unknown_and_actual("'");

    assert_eq!(expected, actual)
}

#[test]
fn test_i16_char_literals_unterminated_eol() {
    let expected = Token::Unknown("'".to_string());
    let actual = actual("'\n");

    assert_eq!(expected, actual)
}

#[test]
fn test_i16_char_literal_1_1byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0x61, "'a'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_2_1byte_utf8_char() {
    // this is _not_ the same behaviour as GCC
    let (expected, actual) = exp_and_actual!(0x62, "'ab'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_3_1byte_utf8_char() {
    // this is _not_ the same behaviour as GCC
    let (expected, actual) = exp_and_actual!(0x63, "'abc'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_4_1byte_utf8_char() {
    // this is _not_ the same behaviour as GCC
    let (expected, actual) = exp_and_actual!(0x64, "'abcd'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_5_1byte_utf8_char() {
    // this is _not_ the same behaviour as GCC
    let (expected, actual) = exp_and_actual!(0x65, "'abcde'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_1_2byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0xa0, "'\u{00a0}'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_2_2byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0xa1, "'\u{00a0}\u{00a1}'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_3_2byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0xa2, "'\u{00a0}\u{00a1}\u{00a2}'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_1_3byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0x0800, "'\u{0800}'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_2_3byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0x0801, "'\u{0800}\u{0801}'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_1_4byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0xdc00, "'\u{010000}'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_2_4byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0xdc01, "'\u{010000}\u{010001}'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_hex_0digit() {
    let (expected, actual) = unknown_and_actual(r"'\x'");

    assert_eq!(expected, actual)
}

#[test]
fn test_i16_char_literal_hex_low_1digit() {
    let (expected, actual) = exp_and_actual!(0x06, r"'\x6'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_hex_low_2digit() {
    let (expected, actual) = exp_and_actual!(0x66, r"'\x66'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_hex_low_3digit() {
    let (expected, actual) = exp_and_actual!(0x666, r"'\x666'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_hex_low_4digit() {
    let (expected, actual) = exp_and_actual!(0x6666, r"'\x6666'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_hex_low_5digit() {
    let (expected, actual) = exp_and_actual!(0x6666, r"'\x66666'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_hex_low_6digit() {
    let (expected, actual) = exp_and_actual!(0x6666, r"'\x666666'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_hex_low_7digit() {
    let (expected, actual) = exp_and_actual!(0x6666, r"'\x6666666'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_hex_low_8digit() {
    let (expected, actual) = exp_and_actual!(0x6666, r"'\x66666666'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_hex_low_9digit() {
    let (expected, actual) = exp_and_actual!(0x6666, r"'\x666666666'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_hex_high_1digit() {
    let (expected, actual) = exp_and_actual!(0x0f, r"'\xf'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_hex_high_2digit() {
    let (expected, actual) = exp_and_actual!(0xff, r"'\xff'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_hex_high_3digit() {
    let (expected, actual) = exp_and_actual!(0x0fff, r"'\xfff'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_hex_high_4digit() {
    let (expected, actual) = exp_and_actual!(0xffff, r"'\xffff'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_hex_high_5digit() {
    let (expected, actual) = exp_and_actual!(0xffff, r"'\xfffff'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_hex_high_6digit() {
    let (expected, actual) = exp_and_actual!(0xffff, r"'\xffffff'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_hex_high_7digit() {
    let (expected, actual) = exp_and_actual!(0xffff, r"'\xfffffff'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_hex_high_8digit() {
    let (expected, actual) = exp_and_actual!(0xffff, r"'\xffffffff'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_hex_high_9digit() {
    let (expected, actual) = exp_and_actual!(0xffff, r"'\xfffffffff'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_hex_mixed_case() {
    let (expected, actual) = exp_and_actual!(0xffff, r"'\xfFfF'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_single_backslash() {
    let (expected, actual) = unknown_and_actual(r"'\'");

    assert_eq!(expected, actual)
}

#[test]
fn test_i16_char_literal_oct_0() {
    let (expected, actual) = exp_and_actual!(0x00, r"'\0'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_oct_00() {
    let (expected, actual) = exp_and_actual!(0x00, r"'\00'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_oct_000() {
    let (expected, actual) = exp_and_actual!(0x00, r"'\000'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_oct_000_0() {
    let (expected, actual) = exp_and_actual!(0x30, r"'\0000'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_oct_1() {
    let (expected, actual) = exp_and_actual!(0x01, r"'\1'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_oct_01() {
    let (expected, actual) = exp_and_actual!(0x01, r"'\01'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_oct_001() {
    let (expected, actual) = exp_and_actual!(0x01, r"'\001'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_oct_11() {
    let (expected, actual) = exp_and_actual!(0x09, r"'\11'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_oct_011() {
    let (expected, actual) = exp_and_actual!(0x09, r"'\011'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_oct_111() {
    let (expected, actual) = exp_and_actual!(0x49, r"'\111'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_oct_111_1() {
    let (expected, actual) = exp_and_actual!(0x31, r"'\1111'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_oct_7() {
    let (expected, actual) = exp_and_actual!(0x07, r"'\7'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_oct_07() {
    let (expected, actual) = exp_and_actual!(0x07, r"'\07'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_oct_007() {
    let (expected, actual) = exp_and_actual!(0x07, r"'\007'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_oct_77() {
    let (expected, actual) = exp_and_actual!(0x3f, r"'\77'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_oct_077() {
    let (expected, actual) = exp_and_actual!(0x3f, r"'\077'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_oct_777() {
    let (expected, actual) = exp_and_actual!(0x01ff, r"'\777'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_oct_777_7() {
    let (expected, actual) = exp_and_actual!(0x37, r"'\7777'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_esc_a() {
    let (expected, actual) = exp_and_actual!(0x07, r"'\a'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_esc_b() {
    let (expected, actual) = exp_and_actual!(0x08, r"'\b'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_esc_f() {
    let (expected, actual) = exp_and_actual!(0x0c, r"'\f'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_esc_n() {
    let (expected, actual) = exp_and_actual!(0x0a, r"'\n'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_esc_r() {
    let (expected, actual) = exp_and_actual!(0x0d, r"'\r'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_esc_t() {
    let (expected, actual) = exp_and_actual!(0x09, r"'\t'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_esc_v() {
    let (expected, actual) = exp_and_actual!(0x0b, r"'\v'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_esc_a_b() {
    let (expected, actual) = exp_and_actual!(0x62, r"'\ab'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_esc_a_esc_b() {
    let (expected, actual) = exp_and_actual!(0x08, r"'\a\b'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_unkn_escape_8() {
    let (expected, actual) = exp_and_actual!(0x38, r"'\8'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_1_2byte_univ_char_short() {
    let (expected, actual) = exp_and_actual!(0xa0, r"'\u00a0'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_2_2byte_univ_char_short() {
    let (expected, actual) = exp_and_actual!(0xa1_u32, r"'\u00a0\u00a1'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_1_3byte_univ_char_short() {
    let (expected, actual) = exp_and_actual!(0x0800, r"'\u0800'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_2_3byte_univ_char_short() {
    let (expected, actual) = exp_and_actual!(0x0801_u32, r"'\u0800\u0801'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_1_2byte_univ_char_long() {
    let (expected, actual) = exp_and_actual!(0xa0, r"'\U000000a0'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_2_2byte_univ_char_long() {
    let (expected, actual) = exp_and_actual!(0xa1_u32, r"'\U000000a0\U000000a1'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_1_3byte_univ_char_long() {
    let (expected, actual) = exp_and_actual!(0x0800, r"'\U00000800'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_2_3byte_univ_char_long() {
    let (expected, actual) = exp_and_actual!(0x0801, r"'\U00000800\U00000801'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_1_4byte_univ_char_long() {
    let (expected, actual) = exp_and_actual!(0xdc00, r"'\U00100000'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i16_char_literal_2_4byte_univ_char_long() {
    let (expected, actual) = exp_and_actual!(0xdc01, r"'\U00100000\U00100001'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}
