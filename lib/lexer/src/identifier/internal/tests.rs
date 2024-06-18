use super::*;

use crate::text::text_state_impl_i8;
use crate::universal_char::univ_esc_impl;
use crate::tests::TestLocation;

use crate::Token::Unknown;
use crate::Token::Identifier;

fn actual(input: &str) -> (Token, usize) {
    let location = Box::new(TestLocation);
    let text = text_state_impl_i8(input.chars().peekable());
    let univ_escape = univ_esc_impl::<i8>(location.as_ref(), text.as_ref());

    let identifier = identifier_impl(location.as_ref(), text.as_ref(), univ_escape.as_ref());
    let token = identifier.consume_identifier();
    let n_chars = text.chars_consumed();

    (token, n_chars)
}

fn unknown_and_actual(input: &str) -> (Token, Token, usize) {
    let unknown = Unknown(input.to_string());
    let (actual, consumed) = actual(input);

    (unknown, actual, consumed)
}

#[test]
fn test_kw_auto() {
    let exp_token = Token::KwAuto;
    let exp_sz = 4;
    let (actual, sz) = actual("auto");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_break() {
    let exp_token = Token::KwBreak;
    let exp_sz = 5;
    let (actual, sz) = actual("break");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_case() {
    let exp_token = Token::KwCase;
    let exp_sz = 4;
    let (actual, sz) = actual("case");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_char() {
    let exp_token = Token::KwChar;
    let exp_sz = 4;
    let (actual, sz) = actual("char");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_const() {
    let exp_token = Token::KwConst;
    let exp_sz = 5;
    let (actual, sz) = actual("const");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_continue() {
    let exp_token = Token::KwContinue;
    let exp_sz = 8;
    let (actual, sz) = actual("continue");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_default() {
    let exp_token = Token::KwDefault;
    let exp_sz = 7;
    let (actual, sz) = actual("default");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_do() {
    let exp_token = Token::KwDo;
    let exp_sz = 2;
    let (actual, sz) = actual("do");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_double() {
    let exp_token = Token::KwDouble;
    let exp_sz = 6;
    let (actual, sz) = actual("double");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_else() {
    let exp_token = Token::KwElse;
    let exp_sz = 4;
    let (actual, sz) = actual("else");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_enum() {
    let exp_token = Token::KwEnum;
    let exp_sz = 4;
    let (actual, sz) = actual("enum");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_extern() {
    let exp_token = Token::KwExtern;
    let exp_sz = 6;
    let (actual, sz) = actual("extern");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_float() {
    let exp_token = Token::KwFloat;
    let exp_sz = 5;
    let (actual, sz) = actual("float");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_for() {
    let exp_token = Token::KwFor;
    let exp_sz = 3;
    let (actual, sz) = actual("for");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_goto() {
    let exp_token = Token::KwGoto;
    let exp_sz = 4;
    let (actual, sz) = actual("goto");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_if() {
    let exp_token = Token::KwIf;
    let exp_sz = 2;
    let (actual, sz) = actual("if");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_inline() {
    let exp_token = Token::KwInline;
    let exp_sz = 6;
    let (actual, sz) = actual("inline");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_int() {
    let exp_token = Token::KwInt;
    let exp_sz = 3;
    let (actual, sz) = actual("int");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_long() {
    let exp_token = Token::KwLong;
    let exp_sz = 4;
    let (actual, sz) = actual("long");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_register() {
    let exp_token = Token::KwRegister;
    let exp_sz = 8;
    let (actual, sz) = actual("register");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_restrict() {
    let exp_token = Token::KwRestrict;
    let exp_sz = 8;
    let (actual, sz) = actual("restrict");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_return() {
    let exp_token = Token::KwReturn;
    let exp_sz = 6;
    let (actual, sz) = actual("return");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_short() {
    let exp_token = Token::KwShort;
    let exp_sz = 5;
    let (actual, sz) = actual("short");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_signed() {
    let exp_token = Token::KwSigned;
    let exp_sz = 6;
    let (actual, sz) = actual("signed");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_sizeof() {
    let exp_token = Token::KwSizeof;
    let exp_sz = 6;
    let (actual, sz) = actual("sizeof");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_static() {
    let exp_token = Token::KwStatic;
    let exp_sz = 6;
    let (actual, sz) = actual("static");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_struct() {
    let exp_token = Token::KwStruct;
    let exp_sz = 6;
    let (actual, sz) = actual("struct");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_switch() {
    let exp_token = Token::KwSwitch;
    let exp_sz = 6;
    let (actual, sz) = actual("switch");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_typedef() {
    let exp_token = Token::KwTypedef;
    let exp_sz = 7;
    let (actual, sz) = actual("typedef");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_union() {
    let exp_token = Token::KwUnion;
    let exp_sz = 5;
    let (actual, sz) = actual("union");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_unsigned() {
    let exp_token = Token::KwUnsigned;
    let exp_sz = 8;
    let (actual, sz) = actual("unsigned");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_void() {
    let exp_token = Token::KwVoid;
    let exp_sz = 4;
    let (actual, sz) = actual("void");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
fn test_kw_while() {
    let exp_token = Token::KwWhile;
    let exp_sz = 5;
    let (actual, sz) = actual("while");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
#[allow(non_snake_case)]
fn test_kw__Alignas() {
    let exp_token = Token::Kw_Alignas;
    let exp_sz = 8;
    let (actual, sz) = actual("_Alignas");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
#[allow(non_snake_case)]
fn test_kw__Alignof() {
    let exp_token = Token::Kw_Alignof;
    let exp_sz = 8;
    let (actual, sz) = actual("_Alignof");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
#[allow(non_snake_case)]
fn test_kw__Atomic() {
    let exp_token = Token::Kw_Atomic;
    let exp_sz = 7;
    let (actual, sz) = actual("_Atomic");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
#[allow(non_snake_case)]
fn test_kw__Bool() {
    let exp_token = Token::Kw_Bool;
    let exp_sz = 5;
    let (actual, sz) = actual("_Bool");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
#[allow(non_snake_case)]
fn test_kw__Complex() {
    let exp_token = Token::Kw_Complex;
    let exp_sz = 8;
    let (actual, sz) = actual("_Complex");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
#[allow(non_snake_case)]
fn test_kw__Generic() {
    let exp_token = Token::Kw_Generic;
    let exp_sz = 8;
    let (actual, sz) = actual("_Generic");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
#[allow(non_snake_case)]
fn test_kw__Imaginary() {
    let exp_token = Token::Kw_Imaginary;
    let exp_sz = 10;
    let (actual, sz) = actual("_Imaginary");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
#[allow(non_snake_case)]
fn test_kw__Noreturn() {
    let exp_token = Token::Kw_Noreturn;
    let exp_sz = 9;
    let (actual, sz) = actual("_Noreturn");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
#[allow(non_snake_case)]
fn test_kw__Static_assert() {
    let exp_token = Token::Kw_Static_assert;
    let exp_sz = 14;
    let (actual, sz) = actual("_Static_assert");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

#[test]
#[allow(non_snake_case)]
fn test_kw__Thread_local() {
    let exp_token = Token::Kw_Thread_local;
    let exp_sz = 13;
    let (actual, sz) = actual("_Thread_local");

    assert_eq!(exp_token, actual);
    assert_eq!(exp_sz, sz);
}

macro_rules! ascii_exp_and_actual {
    ($input: literal) => {
        ((Identifier($input.to_string()), $input.len()), actual($input))
    };
}

#[test]
fn test_identifier_uppercase_auto() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("AUTO");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_break() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("BREAK");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_case() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("CASE");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_char() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("CHAR");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_const() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("CONST");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_continue() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("CONTINUE");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_default() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("DEFAULT");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_do() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("DO");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_double() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("DOUBLE");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_else() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("ELSE");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_enum() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("ENUM");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_extern() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("EXTERN");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_float() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("FLOAT");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_for() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("FOR");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_goto() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("GOTO");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_if() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("IF");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_inline() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("INLINE");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_int() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("INT");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_long() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("LONG");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_register() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("REGISTER");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_restrict() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("RESTRICT");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_return() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("RETURN");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_short() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("SHORT");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_signed() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("SIGNED");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_sizeof() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("SIZEOF");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_static() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("STATIC");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_struct() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("STRUCT");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_switch() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("SWITCH");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_typedef() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("TYPEDEF");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_union() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("UNION");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_unsigned() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("UNSIGNED");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_void() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("VOID");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_uppercase_while() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("WHILE");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_identifier_uppercase__Alignas() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("_ALIGNAS");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_identifier_uppercase__Alignof() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("_ALIGNOF");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_identifier_uppercase__Atomic() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("_ATOMIC");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_identifier_uppercase__Bool() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("_BOOL");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_identifier_uppercase__Complex() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("_COMPLEX");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_identifier_uppercase__Generic() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("_GENERIC");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_identifier_uppercase__Imaginary() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("_IMAGINARY");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_identifier_uppercase__Noreturn() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("_NORETURN");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_identifier_uppercase__Static_assert() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("_STATIC_ASSERT");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_identifier_uppercase__Thread_local() {
    let ((exp_token, exp_sz), (act_token, act_sz)) = ascii_exp_and_actual!("_THREAD_LOCAL");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[should_panic(expected = "TEST:1:1 - FATAL - invalid initial character for identifier")]
fn test_identifier_invalid_initial_char_panics_space() {
    let _ = actual(" ");
}

#[test]
#[should_panic(expected = "TEST:1:1 - FATAL - invalid initial character for identifier")]
fn test_identifier_invalid_initial_char_panics_plus() {
    let _ = actual("+");
}

#[test]
#[should_panic(expected = "TEST:1:1 - FATAL - invalid initial character for identifier")]
fn test_identifier_invalid_initial_char_panics_digit() {
    let _ = actual("0");
}

#[test]
#[should_panic(expected = "TEST:1:1 - FATAL - invalid initial character for identifier")]
fn test_identifier_invalid_initial_char_panics_newline() {
    let _ = actual("\n");
}

#[test]
#[should_panic(expected = "TEST:1:1 - FATAL - invalid initial character for identifier")]
fn test_identifier_empty_input_panics() {
    let _ = actual("");
}

#[test]
fn test_identifier_single_backslash_unknown() {
    let exp_sz = 1;
    let (exp_token, act_token, act_sz) = unknown_and_actual(r"\");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_non_univ_escape_unknown() {
    let exp_sz = 2;
    let (exp_token, act_token, act_sz) = unknown_and_actual(r"\n");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_univ_escape_short_0digits() {
    let exp_sz = 2;
    let (exp_token, act_token, act_sz) = unknown_and_actual(r"\u");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_univ_escape_short_1digits() {
    let exp_sz = 3;
    let (exp_token, act_token, act_sz) = unknown_and_actual(r"\u0");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_univ_escape_short_2digits() {
    let exp_sz = 4;
    let (exp_token, act_token, act_sz) = unknown_and_actual(r"\ua0");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_univ_escape_short_3digits() {
    let exp_sz = 5;
    let (exp_token, act_token, act_sz) = unknown_and_actual(r"\u0a0");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_univ_escape_short_4digits() {
    let exp_token = Identifier("\u{00a0}".to_string());
    let exp_sz = 6;
    let (act_token, act_sz) = actual(r"\u00a0");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_misleading_univ_escape_short_5digits() {
    let exp_token = Identifier("\u{00a0}0".to_string());
    let exp_sz = 7;
    let (act_token, act_sz) = actual(r"\u00a00");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_univ_escape_long_0digits() {
    let exp_sz = 2;
    let (exp_token, act_token, act_sz) = unknown_and_actual(r"\U");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_univ_escape_long_1digits() {
    let exp_sz = 3;
    let (exp_token, act_token, act_sz) = unknown_and_actual(r"\U0");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_univ_escape_long_2digits() {
    let exp_sz = 4;
    let (exp_token, act_token, act_sz) = unknown_and_actual(r"\Ua0");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_univ_escape_long_3digits() {
    let exp_sz = 5;
    let (exp_token, act_token, act_sz) = unknown_and_actual(r"\U0a0");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_univ_escape_long_4digits() {
    let exp_sz = 6;
    let (exp_token, act_token, act_sz) = unknown_and_actual(r"\U00a0");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_univ_escape_long_5digits() {
    let exp_sz = 7;
    let (exp_token, act_token, act_sz) = unknown_and_actual(r"\U000a0");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_univ_escape_long_6digits() {
    let exp_sz = 8;
    let (exp_token, act_token, act_sz) = unknown_and_actual(r"\U0000a0");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_univ_escape_long_7digits() {
    let exp_sz = 9;
    let (exp_token, act_token, act_sz) = unknown_and_actual(r"\U00000a0");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_univ_escape_long_8digits() {
    let exp_token = Identifier("\u{00a0}".to_string());
    let exp_sz = 10;
    let (act_token, act_sz) = actual(r"\U000000a0");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_misleading_univ_escape_long_9digits() {
    let exp_token = Identifier("\u{00a0}0".to_string());
    let exp_sz = 11;
    let (act_token, act_sz) = actual(r"\U000000a00");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_forbidden_univ_escape_short_0x0000() {
    let exp_sz = 6;
    let (exp_token, act_token, act_sz) = unknown_and_actual(r"\u0000");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_allowed_univ_escape_short_0x0024() {
    let exp_token = Identifier("\u{0024}".to_string());
    let exp_sz = 6;
    let (act_token, act_sz) = actual(r"\u0024");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_allowed_univ_escape_short_0x0040() {
    let exp_token = Identifier("\u{0040}".to_string());
    let exp_sz = 6;
    let (act_token, act_sz) = actual(r"\u0040");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_allowed_univ_escape_short_0x0060() {
    let exp_token = Identifier("\u{0060}".to_string());
    let exp_sz = 6;
    let (act_token, act_sz) = actual(r"\u0060");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_forbidden_univ_escape_short_0x009f() {
    let exp_sz = 6;
    let (exp_token, act_token, act_sz) = unknown_and_actual(r"\u009f");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_allowed_univ_escape_short_0x00a0() {
    let exp_token = Identifier("\u{00a0}".to_string());
    let exp_sz = 6;
    let (act_token, act_sz) = actual(r"\u00a0");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_allowed_univ_escape_short_0x02ff() {
    let exp_token = Identifier("\u{02ff}".to_string());
    let exp_sz = 6;
    let (act_token, act_sz) = actual(r"\u02ff");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}


#[test]
fn test_identifier_forbidden_univ_escape_short_0x0300() {
    let exp_sz = 6;
    let (exp_token, act_token, act_sz) = unknown_and_actual(r"\u0300");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}


#[test]
fn test_identifier_forbidden_univ_escape_short_0x036f() {
    let exp_sz = 6;
    let (exp_token, act_token, act_sz) = unknown_and_actual(r"\u036f");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_allowed_univ_escape_short_0x0370() {
    let exp_token = Identifier("\u{0370}".to_string());
    let exp_sz = 6;
    let (act_token, act_sz) = actual(r"\u0370");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_allowed_univ_escape_short_0x1dbf() {
    let exp_token = Identifier("\u{1dbf}".to_string());
    let exp_sz = 6;
    let (act_token, act_sz) = actual(r"\u1dbf");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}


#[test]
fn test_identifier_forbidden_univ_escape_short_0x1dc0() {
    let exp_sz = 6;
    let (exp_token, act_token, act_sz) = unknown_and_actual(r"\u1dc0");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}


#[test]
fn test_identifier_forbidden_univ_escape_short_0x1dff() {
    let exp_sz = 6;
    let (exp_token, act_token, act_sz) = unknown_and_actual(r"\u1dff");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_allowed_univ_escape_short_0x1e00() {
    let exp_token = Identifier("\u{1e00}".to_string());
    let exp_sz = 6;
    let (act_token, act_sz) = actual(r"\u1e00");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_allowed_univ_escape_short_0x20cf() {
    let exp_token = Identifier("\u{20cf}".to_string());
    let exp_sz = 6;
    let (act_token, act_sz) = actual(r"\u20cf");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}


#[test]
fn test_identifier_forbidden_univ_escape_short_0x20d0() {
    let exp_sz = 6;
    let (exp_token, act_token, act_sz) = unknown_and_actual(r"\u20d0");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}


#[test]
fn test_identifier_forbidden_univ_escape_short_0x20ff() {
    let exp_sz = 6;
    let (exp_token, act_token, act_sz) = unknown_and_actual(r"\u20ff");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_allowed_univ_escape_short_0x2100() {
    let exp_token = Identifier("\u{2100}".to_string());
    let exp_sz = 6;
    let (act_token, act_sz) = actual(r"\u2100");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_allowed_univ_escape_short_0xd7ff() {
    let exp_token = Identifier("\u{d7ff}".to_string());
    let exp_sz = 6;
    let (act_token, act_sz) = actual(r"\ud7ff");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}


#[test]
fn test_identifier_forbidden_univ_escape_short_0xd800() {
    let exp_sz = 6;
    let (exp_token, act_token, act_sz) = unknown_and_actual(r"\ud800");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}


#[test]
fn test_identifier_forbidden_univ_escape_short_0xdfff() {
    let exp_sz = 6;
    let (exp_token, act_token, act_sz) = unknown_and_actual(r"\udfff");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_allowed_univ_escape_short_0xe000() {
    let exp_token = Identifier("\u{e000}".to_string());
    let exp_sz = 6;
    let (act_token, act_sz) = actual(r"\ue000");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_allowed_univ_escape_short_0xfe1f() {
    let exp_token = Identifier("\u{fe1f}".to_string());
    let exp_sz = 6;
    let (act_token, act_sz) = actual(r"\ufe1f");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}


#[test]
fn test_identifier_forbidden_univ_escape_short_0xfe20() {
    let exp_sz = 6;
    let (exp_token, act_token, act_sz) = unknown_and_actual(r"\ufe20");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}


#[test]
fn test_identifier_forbidden_univ_escape_short_0xfe2f() {
    let exp_sz = 6;
    let (exp_token, act_token, act_sz) = unknown_and_actual(r"\ufe2f");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_identifier_allowed_univ_escape_short_0xfe30() {
    let exp_token = Identifier("\u{fe30}".to_string());
    let exp_sz = 6;
    let (act_token, act_sz) = actual(r"\ufe30");

    assert_eq!(exp_token, act_token);
    assert_eq!(exp_sz, act_sz);
}
