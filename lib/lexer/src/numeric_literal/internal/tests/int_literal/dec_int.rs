use super::*;

#[test]
fn test_int_literal_dec_no_suffix_min_i32() {
    let (exp_t, act_t, act_sz) = exp_i32_and_actual(1, "1");
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_no_suffix_max_i32() {
    let value = i32::MAX;
    let input = format!("{}", value);

    let (exp_t, act_t, act_sz) = exp_i32_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_no_suffix_min_i64() {
    let value = i32::MAX as i64 + 1;
    let input = format!("{}", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_no_suffix_max_i64() {
    let value = i64::MAX;
    let input = format!("{}", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}



#[test]
fn test_int_literal_dec_no_suffix_overflow_u64() {
    // {:o} format does not include leading zero
    // {:#o} format emits 0o10.... which isn't what we need
    let value = i64::MAX as u64 + 1;
    let input = format!("{}", value);

    let (exp_t, act_t, act_sz) = exp_i32_and_actual(0, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}
