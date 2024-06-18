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
    let (exp_token, act_token, act_sz) = unknown_and_actual(r#"""#);
    let exp_sz = 1;

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_unterminated_eol() {
    let exp_token = Unknown("\"".to_string());
    let exp_sz = 1;
    let (act_token, act_sz) = actual("\"\n");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_unterminated_space_eoi() {
    let (exp_token, act_token, act_sz) = unknown_and_actual(r#""  "#);
    let exp_sz = 3;

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_unterminated_space_eol() {
    let exp_token = Unknown("\"  ".to_string());
    let exp_sz = 3;
    let (act_token, act_sz) = actual("\"  \n");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_1_1byte_utf8_char() {
    let exp_token = StringLit(vec![0x61]);
    let exp_sz = 3;
    let (act_token, act_sz) = actual(r#""a""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_2_1byte_utf8_char() {
    let exp_token = StringLit(vec![0x61, 0x62]);
    let exp_sz = 4;
    let (act_token, act_sz) = actual(r#""ab""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_3_1byte_utf8_char() {
    let exp_token = StringLit(vec![0x61, 0x62, 0x63,]);
    let exp_sz = 5;
    let (act_token, act_sz) = actual(r#""abc""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_4_1byte_utf8_char() {
    let exp_token = StringLit(vec![0x61, 0x62, 0x63, 0x64,]);
    let exp_sz = 6;
    let (act_token, act_sz) = actual(r#""abcd""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_5_1byte_utf8_char() {
    let exp_token = StringLit(vec![0x61, 0x62, 0x63, 0x64, 0x65,]);
    let exp_sz = 7;
    let (act_token, act_sz) = actual(r#""abcde""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_5_1byte_utf8_char_with_null() {
    let exp_token = StringLit(vec![0x61, 0x62, 0x00, 0x64, 0x65,]);
    let exp_sz = 8;
    let (act_token, act_sz) = actual(r#""ab\0de""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_1_2byte_utf8_char() {
    let exp_token = StringLit(vec![0xc2_u8 as i8, 0xa0_u8 as i8,]);
    let exp_sz = 3;
    let (act_token, act_sz) = actual("\"\u{00a0}\"");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_2_2byte_utf8_char() {
    let exp_token = StringLit(vec![0xc2_u8 as i8, 0xa0_u8 as i8, 0xc2_u8 as i8, 0xa1_u8 as i8]);
    let exp_sz = 4;
    let (act_token, act_sz) = actual("\"\u{00a0}\u{00a1}\"");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_3_2byte_utf8_char() {
    let exp_token = StringLit(vec![0xc2_u8 as i8, 0xa0_u8 as i8, 0xc2_u8 as i8, 0xa1_u8 as i8, 0xc2_u8 as i8, 0xa2_u8 as i8]);
    let exp_sz = 5;
    let (act_token, act_sz) = actual("\"\u{00a0}\u{00a1}\u{00a2}\"");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_1_3byte_utf8_char() {
    let exp_token = StringLit(vec![0xe0_u8 as i8, 0xa0_u8 as i8, 0x80_u8 as i8,]);
    let exp_sz = 3;
    let (act_token, act_sz) = actual("\"\u{0800}\"");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_2_3byte_utf8_char() {
    let exp_token = StringLit(vec![0xe0_u8 as i8, 0xa0_u8 as i8, 0x80_u8 as i8, 0xe0_u8 as i8, 0xa0_u8 as i8, 0x81_u8 as i8,]);
    let exp_sz = 4;
    let (act_token, act_sz) = actual("\"\u{0800}\u{0801}\"");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_1_4byte_utf8_char() {
    let exp_token = StringLit(vec![0xf0_u8 as i8, 0x90_u8 as i8, 0x80_u8 as i8, 0x80_u8 as i8,]);
    let exp_sz = 3;
    let (act_token, act_sz) = actual("\"\u{010000}\"");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_2_4byte_utf8_char() {
    let exp_token = StringLit(vec![0xf0_u8 as i8, 0x90_u8 as i8, 0x80_u8 as i8, 0x80_u8 as i8, 0xf0_u8 as i8, 0x90_u8 as i8, 0x80_u8 as i8, 0x81_u8 as i8,]);
    let exp_sz = 4;
    let (act_token, act_sz) = actual("\"\u{010000}\u{010001}\"");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_hex_0digit() {
    let exp_sz = 4;
    let (exp_token, act_token, act_sz) = unknown_and_actual(r#""\x""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_hex_low_1digit() {
    let exp_token = StringLit(vec![0x06]);
    let exp_sz = 5;
    let (act_token, act_sz) = actual(r#""\x6""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_hex_low_2digit() {
    let exp_token = StringLit(vec![0x66]);
    let exp_sz = 6;
    let (act_token, act_sz) = actual(r#""\x66""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_hex_low_3digit() {
    let exp_token = StringLit(vec![0x66]);
    let exp_sz = 7;
    let (act_token, act_sz) = actual(r#""\x666""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_hex_low_4digit() {
    let exp_token = StringLit(vec![0x66]);
    let exp_sz = 8;
    let (act_token, act_sz) = actual(r#""\x6666""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_hex_low_5digit() {
    let exp_token = StringLit(vec![0x66]);
    let exp_sz = 9;
    let (act_token, act_sz) = actual(r#""\x66666""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_hex_low_6digit() {
    let exp_token = StringLit(vec![0x66]);
    let exp_sz = 10;
    let (act_token, act_sz) = actual(r#""\x666666""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_hex_low_7digit() {
    let exp_token = StringLit(vec![0x66]);
    let exp_sz = 11;
    let (act_token, act_sz) = actual(r#""\x6666666""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_hex_low_8digit() {
    let exp_token = StringLit(vec![0x66]);
    let exp_sz = 12;
    let (act_token, act_sz) = actual(r#""\x66666666""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_hex_low_9digit() {
    let exp_token = StringLit(vec![0x66]);
    let exp_sz = 13;
    let (act_token, act_sz) = actual(r#""\x666666666""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_hex_high_1digit() {
    let exp_token = StringLit(vec![0x0f]);
    let exp_sz = 5;
    let (act_token, act_sz) = actual(r#""\xf""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_hex_high_2digit() {
    let exp_token = StringLit(vec![0xff_u8 as i8]);
    let exp_sz = 6;
    let (act_token, act_sz) = actual(r#""\xff""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_hex_high_3digit() {
    let exp_token = StringLit(vec![0xff_u8 as i8]);
    let exp_sz = 7;
    let (act_token, act_sz) = actual(r#""\xfff""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_hex_high_4digit() {
    let exp_token = StringLit(vec![0xff_u8 as i8]);
    let exp_sz = 8;
    let (act_token, act_sz) = actual(r#""\xffff""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_hex_high_5digit() {
    let exp_token = StringLit(vec![0xff_u8 as i8]);
    let exp_sz = 9;
    let (act_token, act_sz) = actual(r#""\xfffff""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_hex_high_6digit() {
    let exp_token = StringLit(vec![0xff_u8 as i8]);
    let exp_sz = 10;
    let (act_token, act_sz) = actual(r#""\xffffff""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_hex_high_7digit() {
    let exp_token = StringLit(vec![0xff_u8 as i8]);
    let exp_sz = 11;
    let (act_token, act_sz) = actual(r#""\xfffffff""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_hex_high_8digit() {
    let exp_token = StringLit(vec![0xff_u8 as i8]);
    let exp_sz = 12;
    let (act_token, act_sz) = actual(r#""\xffffffff""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_hex_high_9digit() {
    let exp_token = StringLit(vec![0xff_u8 as i8]);
    let exp_sz = 13;
    let (act_token, act_sz) = actual(r#""\xfffffffff""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_hex_mixed_case() {
    let exp_token = StringLit(vec![0xff_u8 as i8]);
    let exp_sz = 6;
    let (act_token, act_sz) = actual(r#""\xfF""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_single_backslash() {
    let exp_sz = 3;
    let (exp_token, act_token, act_sz) = unknown_and_actual(r#""\""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_oct_0() {
    let exp_token = StringLit(vec![0x00]);
    let exp_sz = 4;
    let (act_token, act_sz) = actual(r#""\0""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_oct_00() {
    let exp_token = StringLit(vec![0x00]);
    let exp_sz = 5;
    let (act_token, act_sz) = actual(r#""\00""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_oct_000() {
    let exp_token = StringLit(vec![0x00]);
    let exp_sz = 6;
    let (act_token, act_sz) = actual(r#""\000""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_oct_000_0() {
    let exp_token = StringLit(vec![0x00, 0x30]);
    let exp_sz = 7;
    let (act_token, act_sz) = actual(r#""\0000""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_oct_1() {
    let exp_token = StringLit(vec![0x01]);
    let exp_sz = 4;
    let (act_token, act_sz) = actual(r#""\1""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_oct_01() {
    let exp_token = StringLit(vec![0x01]);
    let exp_sz = 5;
    let (act_token, act_sz) = actual(r#""\01""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_oct_001() {
    let exp_token = StringLit(vec![0x01]);
    let exp_sz = 6;
    let (act_token, act_sz) = actual(r#""\001""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_oct_11() {
    let exp_token = StringLit(vec![0x09]);
    let exp_sz = 5;
    let (act_token, act_sz) = actual(r#""\11""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_oct_011() {
    let exp_token = StringLit(vec![0x09]);
    let exp_sz = 6;
    let (act_token, act_sz) = actual(r#""\011""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_oct_111() {
    let exp_token = StringLit(vec![0x49]);
    let exp_sz = 6;
    let (act_token, act_sz) = actual(r#""\111""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_oct_111_1() {
    let exp_token = StringLit(vec![0x49, 0x31]);
    let exp_sz = 7;
    let (act_token, act_sz) = actual(r#""\1111""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_oct_7() {
    let exp_token = StringLit(vec![0x07]);
    let exp_sz = 4;
    let (act_token, act_sz) = actual(r#""\7""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_oct_07() {
    let exp_token = StringLit(vec![0x07]);
    let exp_sz = 5;
    let (act_token, act_sz) = actual(r#""\07""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_oct_007() {
    let exp_token = StringLit(vec![0x07]);
    let exp_sz = 6;
    let (act_token, act_sz) = actual(r#""\007""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_oct_77() {
    let exp_token = StringLit(vec![0x3f]);
    let exp_sz = 5;
    let (act_token, act_sz) = actual(r#""\77""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_oct_077() {
    let exp_token = StringLit(vec![0x3f]);
    let exp_sz = 6;
    let (act_token, act_sz) = actual(r#""\077""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_oct_777() {
    let exp_token = StringLit(vec![0xff_u8 as i8,]);
    let exp_sz = 6;
    let (act_token, act_sz) = actual(r#""\777""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_oct_777_7() {
    let exp_token = StringLit(vec![0xff_u8 as i8, 0x37]);
    let exp_sz = 7;
    let (act_token, act_sz) = actual(r#""\7777""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_esc_a() {
    let exp_token = StringLit(vec![0x07]);
    let exp_sz = 4;
    let (act_token, act_sz) = actual(r#""\a""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_esc_b() {
    let exp_token = StringLit(vec![0x08]);
    let exp_sz = 4;
    let (act_token, act_sz) = actual(r#""\b""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_esc_f() {
    let exp_token = StringLit(vec![0x0c]);
    let exp_sz = 4;
    let (act_token, act_sz) = actual(r#""\f""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_esc_n() {
    let exp_token = StringLit(vec![0x0a]);
    let exp_sz = 4;
    let (act_token, act_sz) = actual(r#""\n""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_esc_r() {
    let exp_token = StringLit(vec![0x0d]);
    let exp_sz = 4;
    let (act_token, act_sz) = actual(r#""\r""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_esc_t() {
    let exp_token = StringLit(vec![0x09]);
    let exp_sz = 4;
    let (act_token, act_sz) = actual(r#""\t""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_esc_v() {
    let exp_token = StringLit(vec![0x0b]);
    let exp_sz = 4;
    let (act_token, act_sz) = actual(r#""\v""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_esc_a_b() {
    let exp_token = StringLit(vec![0x07, 0x62]);
    let exp_sz = 5;
    let (act_token, act_sz) = actual(r#""\ab""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_esc_a_esc_b() {
    let exp_token = StringLit(vec![0x07, 0x08]);
    let exp_sz = 6;
    let (act_token, act_sz) = actual(r#""\a\b""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_unkn_esc_8() {
    let exp_token = StringLit(vec![0x38]);
    let exp_sz = 4;
    let (act_token, act_sz) = actual(r#""\8""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_1_2byte_univ_char_short() {
    let exp_token = StringLit(vec![0xc2_u8 as i8, 0xa0_u8 as i8]);
    let exp_sz = 8;
    let (act_token, act_sz) = actual(r#""\u00a0""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_2_2byte_univ_char_short() {
    let exp_token = StringLit(vec![0xc2_u8 as i8, 0xa0_u8 as i8, 0xc2_u8 as i8, 0xa1_u8 as i8]);
    let exp_sz = 14;
    let (act_token, act_sz) = actual(r#""\u00a0\u00a1""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_1_3byte_univ_char_short() {
    let exp_token = StringLit(vec![0xe0_u8 as i8, 0xa0_u8 as i8, 0x80_u8 as i8]);
    let exp_sz = 8;
    let (act_token, act_sz) = actual(r#""\u0800""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_2_3byte_univ_char_short() {
    let exp_token = StringLit(vec![0xe0_u8 as i8, 0xa0_u8 as i8, 0x80_u8 as i8, 0xe0_u8 as i8, 0xa0_u8 as i8, 0x81_u8 as i8]);
    let exp_sz = 14;
    let (act_token, act_sz) = actual(r#""\u0800\u0801""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_1_2byte_univ_char_long() {
    let exp_token = StringLit(vec![0xc2_u8 as i8, 0xa0_u8 as i8,]);
    let exp_sz = 12;
    let (act_token, act_sz) = actual(r#""\U000000a0""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_2_2byte_univ_char_long() {
    let exp_token = StringLit(vec![0xc2_u8 as i8, 0xa0_u8 as i8, 0xc2_u8 as i8, 0xa1_u8 as i8,]);
    let exp_sz = 22;
    let (act_token, act_sz) = actual(r#""\U000000a0\U000000a1""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_1_3byte_univ_char_long() {
    let exp_token = StringLit(vec![0xe0_u8 as i8, 0xa0_u8 as i8, 0x80_u8 as i8]);
    let exp_sz = 12;
    let (act_token, act_sz) = actual(r#""\U00000800""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_2_3byte_univ_char_long() {
    let exp_token = StringLit(vec![0xe0_u8 as i8, 0xa0_u8 as i8, 0x80_u8 as i8, 0xe0_u8 as i8, 0xa0_u8 as i8, 0x81_u8 as i8]);
    let exp_sz = 22;
    let (act_token, act_sz) = actual(r#""\U00000800\U00000801""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_1_4byte_univ_char_long() {
    let exp_token = StringLit(vec![0xf4_u8 as i8, 0x80_u8 as i8, 0x80_u8 as i8, 0x80_u8 as i8,]);
    let exp_sz = 12;
    let (act_token, act_sz) = actual(r#""\U00100000""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_2_4byte_univ_char_long() {
    let exp_token = StringLit(vec![0xf4_u8 as i8, 0x80_u8 as i8, 0x80_u8 as i8, 0x80_u8 as i8, 0xf4_u8 as i8, 0x80_u8 as i8, 0x80_u8 as i8, 0x81_u8 as i8]);
    let exp_sz = 22;
    let (act_token, act_sz) = actual(r#""\U00100000\U00100001""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_misleading_univ_char_short_5hex() {
    let exp_token = StringLit(vec![0xc2_u8 as i8, 0xa0_u8 as i8, 0x61,]);
    let exp_sz = 9;
    let (act_token, act_sz) = actual(r#""\u00a0a""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_i8_string_literal_misleading_univ_char_long_9hex() {
    let exp_token = StringLit(vec![0xc2_u8 as i8, 0xa0_u8 as i8, 0x61,]);
    let exp_sz = 13;
    let (act_token, act_sz) = actual(r#""\U000000a0a""#);

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}
