mod dec_float;
mod hex_float;

use crate::numeric_literal::numeric_literal_impl;
use crate::numeric_literal::numeric_state_impl;
use crate::tests::TestLocation;
use crate::Token;
use crate::Token::FloatLit32;
use crate::Token::FloatLit64;
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

fn exp_f32_and_actual(v: f32, input: &str) -> (Token, Token, usize) {
        let exp_t: Token = FloatLit32(v);
        let (act_t, act_sz) = actual(input);
        (exp_t, act_t, act_sz)
}

fn exp_f64_and_actual(v: f64, input: &str) -> (Token, Token, usize) {
        let exp_t: Token = FloatLit64(v);
        let (act_t, act_sz) = actual(input);
        (exp_t, act_t, act_sz)
}