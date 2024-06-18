use super::*;

use crate::text::char_escape::char_esc_impl;
use crate::text::hex_escape::hex_esc_impl;
use crate::text::oct_escape::oct_esc_impl;
use crate::text::text_state_impl_i8;
use crate::text::univ_esc::univ_esc_impl;
use crate::CharLit;
use crate::Token;

fn actual(input: &str) -> Token {
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
        (CharLit($v as i32), actual($input))
    };
}

fn unwrap_values(left: &Token, right: &Token) -> (i32, i32) {
    let CharLit(l) = left else { panic!() };
    let CharLit(r) = right else { panic!() };
    (*l, *r)
}

#[test]
fn test_i8_char_literal_empty() {
    let (expected, actual) = unknown_and_actual("''");

    assert_eq!(expected, actual)
}

#[test]
fn test_i8_char_literals_unterminated_eoi() {
    let (expected, actual) = unknown_and_actual("'");

    assert_eq!(expected, actual)
}

#[test]
fn test_i8_char_literals_unterminated_eol() {
    let expected = Token::Unknown("'".to_string());
    let actual = actual("'\n");

    assert_eq!(expected, actual)
}

#[test]
fn test_i8_char_literal_1_1byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0x61, "'a'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_2_1byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0x61_62, "'ab'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_3_1byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0x61_62_63, "'abc'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_4_1byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0x61_62_63_64, "'abcd'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_5_1byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0x62_63_64_65, "'abcde'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_1_2byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0xc2a0, "'\u{00a0}'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_2_2byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0xc2a0_c2a1_u32, "'\u{00a0}\u{00a1}'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_3_2byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0xc2a1_c2a2_u32, "'\u{00a0}\u{00a1}\u{00a2}'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_1_3byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0x00_e0a080, "'\u{0800}'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_2_3byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0x80_e0a081_u32, "'\u{0800}\u{0801}'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_1_4byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0xf0908080_u32, "'\u{010000}'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_2_4byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0xf0908081_u32, "'\u{010000}\u{010001}'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_0digit() {
    let (expected, actual) = unknown_and_actual(r"'\x'");

    assert_eq!(expected, actual)
}

#[test]
fn test_i8_char_literal_hex_low_1digit() {
    let (expected, actual) = exp_and_actual!(0x06, r"'\x6'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_low_2digit() {
    let (expected, actual) = exp_and_actual!(0x66, r"'\x66'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_low_3digit() {
    let (expected, actual) = exp_and_actual!(0x66, r"'\x666'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_low_4digit() {
    let (expected, actual) = exp_and_actual!(0x66, r"'\x6666'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_low_5digit() {
    let (expected, actual) = exp_and_actual!(0x66, r"'\x66666'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_low_6digit() {
    let (expected, actual) = exp_and_actual!(0x66, r"'\x666666'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_low_7digit() {
    let (expected, actual) = exp_and_actual!(0x66, r"'\x6666666'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_low_8digit() {
    let (expected, actual) = exp_and_actual!(0x66, r"'\x66666666'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_low_9digit() {
    let (expected, actual) = exp_and_actual!(0x66, r"'\x666666666'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_high_1digit() {
    let (expected, actual) = exp_and_actual!(0x0f, r"'\xf'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_high_2digit() {
    let (expected, actual) = exp_and_actual!(0xffffffff_u32, r"'\xff'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_high_3digit() {
    let (expected, actual) = exp_and_actual!(0xffffffff_u32, r"'\xfff'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_high_4digit() {
    let (expected, actual) = exp_and_actual!(0xffffffff_u32, r"'\xffff'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_high_5digit() {
    let (expected, actual) = exp_and_actual!(0xffffffff_u32, r"'\xfffff'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_high_6digit() {
    let (expected, actual) = exp_and_actual!(0xffffffff_u32, r"'\xffffff'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_high_7digit() {
    let (expected, actual) = exp_and_actual!(0xffffffff_u32, r"'\xfffffff'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_high_8digit() {
    let (expected, actual) = exp_and_actual!(0xffffffff_u32, r"'\xffffffff'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_high_9digit() {
    let (expected, actual) = exp_and_actual!(0xffffffff_u32, r"'\xfffffffff'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_mixed_case() {
    let (expected, actual) = exp_and_actual!(0xffffffff_u32, r"'\xfF'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_single_backslash() {
    let (expected, actual) = unknown_and_actual(r"'\'");

    assert_eq!(expected, actual)
}

#[test]
fn test_i8_char_literal_oct_0() {
    let (expected, actual) = exp_and_actual!(0x00, r"'\0'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_oct_00() {
    let (expected, actual) = exp_and_actual!(0x00, r"'\00'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_oct_000() {
    let (expected, actual) = exp_and_actual!(0x00, r"'\000'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_oct_000_0() {
    let (expected, actual) = exp_and_actual!(0x30, r"'\0000'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_oct_1() {
    let (expected, actual) = exp_and_actual!(0x01, r"'\1'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_oct_01() {
    let (expected, actual) = exp_and_actual!(0x01, r"'\01'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_oct_001() {
    let (expected, actual) = exp_and_actual!(0x01, r"'\001'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_oct_11() {
    let (expected, actual) = exp_and_actual!(0x09, r"'\11'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_oct_011() {
    let (expected, actual) = exp_and_actual!(0x09, r"'\011'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_oct_111() {
    let (expected, actual) = exp_and_actual!(0x49, r"'\111'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_oct_111_1() {
    let (expected, actual) = exp_and_actual!(0x4931, r"'\1111'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_oct_7() {
    let (expected, actual) = exp_and_actual!(0x07, r"'\7'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_oct_07() {
    let (expected, actual) = exp_and_actual!(0x07, r"'\07'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_oct_007() {
    let (expected, actual) = exp_and_actual!(0x07, r"'\007'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_oct_77() {
    let (expected, actual) = exp_and_actual!(0x3f, r"'\77'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_oct_077() {
    let (expected, actual) = exp_and_actual!(0x3f, r"'\077'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_oct_777() {
    let (expected, actual) = exp_and_actual!(0xffffffff_u32, r"'\777'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_oct_777_7() {
    let (expected, actual) = exp_and_actual!(0xff37, r"'\7777'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_esc_a() {
    let (expected, actual) = exp_and_actual!(0x07, r"'\a'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_esc_b() {
    let (expected, actual) = exp_and_actual!(0x08, r"'\b'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_esc_f() {
    let (expected, actual) = exp_and_actual!(0x0c, r"'\f'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_esc_n() {
    let (expected, actual) = exp_and_actual!(0x0a, r"'\n'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_esc_r() {
    let (expected, actual) = exp_and_actual!(0x0d, r"'\r'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_esc_t() {
    let (expected, actual) = exp_and_actual!(0x09, r"'\t'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_esc_v() {
    let (expected, actual) = exp_and_actual!(0x0b, r"'\v'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_esc_a_b() {
    let (expected, actual) = exp_and_actual!(0x0762, r"'\ab'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_esc_a_esc_b() {
    let (expected, actual) = exp_and_actual!(0x0708, r"'\a\b'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_unkn_escape_8() {
    let (expected, actual) = exp_and_actual!(0x38, r"'\8'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_1_2byte_univ_char_short() {
    let (expected, actual) = exp_and_actual!(0xc2a0, r"'\u00a0'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_2_2byte_univ_char_short() {
    let (expected, actual) = exp_and_actual!(0xc2a0c2a1_u32, r"'\u00a0\u00a1'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_1_3byte_univ_char_short() {
    let (expected, actual) = exp_and_actual!(0xe0a080, r"'\u0800'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_2_3byte_univ_char_short() {
    let (expected, actual) = exp_and_actual!(0x80e0a081_u32, r"'\u0800\u0801'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_1_2byte_univ_char_long() {
    let (expected, actual) = exp_and_actual!(0xc2a0, r"'\U000000a0'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_2_2byte_univ_char_long() {
    let (expected, actual) = exp_and_actual!(0xc2a0c2a1_u32, r"'\U000000a0\U000000a1'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_1_3byte_univ_char_long() {
    let (expected, actual) = exp_and_actual!(0xe0a080, r"'\U00000800'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_2_3byte_univ_char_long() {
    let (expected, actual) = exp_and_actual!(0x80e0a081_u32, r"'\U00000800\U00000801'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_1_4byte_univ_char_long() {
    let (expected, actual) = exp_and_actual!(0xf4808080_u32, r"'\U00100000'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_2_4byte_univ_char_long() {
    let (expected, actual) = exp_and_actual!(0xf4808081_u32, r"'\U00100000\U00100001'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn text_i8_char_literal_incomplete_univ_char_short_0hex() {
    let (expected, actual) = unknown_and_actual(r"'\ug'");

    assert_eq!(expected, actual);
}

#[test]
fn text_i8_char_literal_incomplete_univ_char_short_1hex() {
    let (expected, actual) = unknown_and_actual(r"'\u0g'");

    assert_eq!(expected, actual);
}

#[test]
fn text_i8_char_literal_incomplete_univ_char_short_2hex() {
    let (expected, actual) = unknown_and_actual(r"'\ua0g'");

    assert_eq!(expected, actual);
}

#[test]
fn text_i8_char_literal_incomplete_univ_char_short_3hex() {
    let (expected, actual) = unknown_and_actual(r"'\u0a0g'");

    assert_eq!(expected, actual);
}

#[test]
fn text_i8_char_literal_misleading_univ_char_short_5hex() {
    let (expected, actual) = exp_and_actual!(0xc2a0_61, r"'\u00a0a'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn text_i8_char_literal_incomplete_univ_char_long_0hex() {
    let (expected, actual) = unknown_and_actual(r"'\Ug'");

    assert_eq!(expected, actual);
}

#[test]
fn text_i8_char_literal_incomplete_univ_char_long_1hex() {
    let (expected, actual) = unknown_and_actual(r"'\U0g'");

    assert_eq!(expected, actual);
}

#[test]
fn text_i8_char_literal_incomplete_univ_char_long_2hex() {
    let (expected, actual) = unknown_and_actual(r"'\Ua0g'");

    assert_eq!(expected, actual);
}

#[test]
fn text_i8_char_literal_incomplete_univ_char_long_3hex() {
    let (expected, actual) = unknown_and_actual(r"'\U0a0g'");

    assert_eq!(expected, actual);
}

#[test]
fn text_i8_char_literal_incomplete_univ_char_long_4hex() {
    let (expected, actual) = unknown_and_actual(r"'\U00a0g'");

    assert_eq!(expected, actual);
}

#[test]
fn text_i8_char_literal_incomplete_univ_char_long_5hex() {
    let (expected, actual) = unknown_and_actual(r"'\U000a0g'");

    assert_eq!(expected, actual);
}

#[test]
fn text_i8_char_literal_incomplete_univ_char_long_6hex() {
    let (expected, actual) = unknown_and_actual(r"'\U0000a0g'");

    assert_eq!(expected, actual);
}

#[test]
fn text_i8_char_literal_incomplete_univ_char_long_7hex() {
    let (expected, actual) = unknown_and_actual(r"'\U00000a0g'");

    assert_eq!(expected, actual);
}

#[test]
fn text_i8_char_literal_misleading_univ_char_long_9hex() {
    let (expected, actual) = exp_and_actual!(0xc2a0_61, r"'\U000000a0a'");
    let (e, a) = unwrap_values(&expected, &actual);

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}
