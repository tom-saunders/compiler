use super::*;

use crate::char_escape::char_esc_impl;
use crate::hex_escape::hex_esc_impl;
use crate::oct_escape::oct_esc_impl;
use crate::text::text_state_impl_i8;
use crate::universal_char::univ_esc_impl;
use crate::Token;
use crate::CharLit;

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
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_2_1byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0x61_62, "'ab'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_3_1byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0x61_62_63, "'abc'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_4_1byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0x61_62_63_64, "'abcd'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_5_1byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0x62_63_64_65, "'abcde'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_1_2byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0xc2a0, "'\u{00a0}'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_2_2byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0xc2a0_c2a1_u32, "'\u{00a0}\u{00a1}'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_3_2byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0xc2a1_c2a2_u32, "'\u{00a0}\u{00a1}\u{00a2}'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_1_3byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0x00_e0a080, "'\u{0800}'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_2_3byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0x80_e0a081_u32, "'\u{0800}\u{0801}'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_1_4byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0xf0908080_u32, "'\u{010000}'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_2_4byte_utf8_char() {
    let (expected, actual) = exp_and_actual!(0xf0908081_u32, "'\u{010000}\u{010001}'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

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
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_low_2digit() {
    let (expected, actual) = exp_and_actual!(0x66, r"'\x66'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_low_3digit() {
    let (expected, actual) = exp_and_actual!(0x66, r"'\x666'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_low_4digit() {
    let (expected, actual) = exp_and_actual!(0x66, r"'\x6666'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_low_5digit() {
    let (expected, actual) = exp_and_actual!(0x66, r"'\x66666'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_low_6digit() {
    let (expected, actual) = exp_and_actual!(0x66, r"'\x666666'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_low_7digit() {
    let (expected, actual) = exp_and_actual!(0x66, r"'\x6666666'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_low_8digit() {
    let (expected, actual) = exp_and_actual!(0x66, r"'\x66666666'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_low_9digit() {
    let (expected, actual) = exp_and_actual!(0x66, r"'\x666666666'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_high_1digit() {
    let (expected, actual) = exp_and_actual!(0x0f, r"'\xf'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_high_2digit() {
    let (expected, actual) = exp_and_actual!(0xffffffff_u32, r"'\xff'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_high_3digit() {
    let (expected, actual) = exp_and_actual!(0xffffffff_u32, r"'\xfff'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_high_4digit() {
    let (expected, actual) = exp_and_actual!(0xffffffff_u32, r"'\xffff'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_high_5digit() {
    let (expected, actual) = exp_and_actual!(0xffffffff_u32, r"'\xfffff'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_high_6digit() {
    let (expected, actual) = exp_and_actual!(0xffffffff_u32, r"'\xffffff'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_high_7digit() {
    let (expected, actual) = exp_and_actual!(0xffffffff_u32, r"'\xfffffff'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_high_8digit() {
    let (expected, actual) = exp_and_actual!(0xffffffff_u32, r"'\xffffffff'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_high_9digit() {
    let (expected, actual) = exp_and_actual!(0xffffffff_u32, r"'\xfffffffff'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else { panic!() };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_hex_mixed_case() {
    let (expected, actual) = exp_and_actual!(0xffffffff_u32, r"'\xfF'");
    let CharLit(e) = expected else {
        panic!()
    };
    let CharLit(a) = actual else {
        panic!()
    };

    assert_eq!(expected, actual, "{e:#010x} != {a:#010x}")
}

#[test]
fn test_i8_char_literal_single_backslash() {
    let (expected, actual) = unknown_and_actual(r"'\'");

    assert_eq!(expected, actual)
}
