use super::*;

#[test]
fn test_float_literal_dec_no_exponent() {
    let (exp_t, act_t, act_sz) = exp_f64_and_actual(1.0, "1.");
    let exp_sz = 2;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}