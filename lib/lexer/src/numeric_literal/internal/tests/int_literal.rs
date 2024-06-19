mod dec_int;
mod hex_int;
mod oct_int;

use crate::numeric_literal::numeric_literal_impl;
use crate::numeric_literal::numeric_state_impl;
use crate::tests::TestLocation;
use crate::Token;
use crate::Token::IntLitI32;
use crate::Token::IntLitI64;
use crate::Token::IntLitU32;
use crate::Token::IntLitU64;
use crate::Token::Unknown;

fn actual(input: &str) -> (Token, usize) {
    let location = Box::new(TestLocation);
    let numeric = numeric_state_impl(input.chars().peekable());

    let numeric_literal = numeric_literal_impl(location.as_ref(), numeric.as_ref());

    let token = numeric_literal.consume_numeric_literal();
    let sz = numeric.chars_consumed();

    (token, sz)
}

fn unknown_and_actual(input: &str) -> (Token, Token, usize) {
    let exp_token = Unknown(input.to_string());
    let (act_token, act_sz) = actual(input);

    (exp_token, act_token, act_sz)
}

fn exp_i32_and_actual(v: i32, input: &str) -> (Token, Token, usize) {
        let exp_t: Token = IntLitI32(v);
        let (act_t, act_sz) = actual(input);
        (exp_t, act_t, act_sz)
}

fn exp_u32_and_actual(v: u32, input: &str) -> (Token,  Token, usize) {
        let exp_t: Token = IntLitU32(v);
        let (act_t, act_sz) = actual(input);
        (exp_t, act_t, act_sz)
}

fn exp_i64_and_actual(v: i64, input: &str) -> (Token, Token, usize) {
        let exp_t: Token = IntLitI64(v);
        let (act_t, act_sz) = actual(input);
        (exp_t, act_t, act_sz)
}

fn exp_u64_and_actual(v: u64, input: &str) -> (Token, Token, usize) {
        let exp_t: Token = IntLitU64(v);
        let (act_t, act_sz) = actual(input);
        (exp_t, act_t, act_sz)
}

#[test]
#[should_panic(expected = "TEST:1:1 - FATAL - not a numeric literal")]
fn test_int_literal_empty() {
    actual("");
}

#[test]
#[should_panic(expected = "TEST:1:1 - FATAL - not a numeric literal")]
fn test_int_literal_nondigit() {
    actual("a");
}
