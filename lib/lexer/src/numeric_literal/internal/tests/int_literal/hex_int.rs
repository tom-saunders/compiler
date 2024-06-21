use super::*;

#[test]
fn test_int_literal_hex_no_suffix_min_i32() {
    let (exp_t, act_t, act_sz) = exp_i32_and_actual(0, "0x0");
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_no_suffix_max_i32() {
    let value = i32::MAX;
    let input = format!("{:#x}", value);

    let (exp_t, act_t, act_sz) = exp_i32_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_no_suffix_min_u32() {
    let value = i32::MAX as u32 + 1;
    let input = format!("{:#x}", value);

    let (exp_t, act_t, act_sz) = exp_u32_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_no_suffix_max_u32() {
    let value = u32::MAX;
    let input = format!("{:#x}", value);

    let (exp_t, act_t, act_sz) = exp_u32_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_no_suffix_min_i64() {
    let value = u32::MAX as i64 + 1;
    let input = format!("{:#x}", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_no_suffix_max_i64() {
    let value = i64::MAX;
    let input = format!("{:#x}", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_no_suffix_min_u64() {
    let value = i64::MAX as u64 + 1;
    let input = format!("{:#x}", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_no_suffix_max_u64() {
    let value = u64::MAX;
    let input = format!("{:#x}", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_no_suffix_overflow_u64() {
    let value = u64::MAX as i128 + 1;
    let input = format!("{:#x}", value);

    let (exp_t, act_t, act_sz) = exp_i32_and_actual(0, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_no_suffix_leading_zeros_i32() {
    let input = "0x0000000000000000000000000000000000000000000000000000000000000000000000000000001";

    let (exp_t, act_t, act_sz) = exp_i32_and_actual(1, input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_u_suffix_min_i32_is_u32() {
    let value = 0;
    let input = format!("{:#x}u", value);

    let (exp_t, act_t, act_sz) = exp_u32_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_u_suffix_max_i32_is_u32() {
    let value = i32::MAX as u32;
    let input = format!("{:#x}u", value);

    let (exp_t, act_t, act_sz) = exp_u32_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_u_suffix_min_u32() {
    let value = i32::MAX as u32 + 1;
    let input = format!("{:#x}u", value);

    let (exp_t, act_t, act_sz) = exp_u32_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_u_suffix_max_u32() {
    let value = u32::MAX;
    let input = format!("{:#x}u", value);

    let (exp_t, act_t, act_sz) = exp_u32_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_u_suffix_min_i64_is_u64() {
    let value = u32::MAX as u64 + 1;
    let input = format!("{:#x}u", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_u_suffix_max_i64_is_u64() {
    let value = i64::MAX as u64;
    let input = format!("{:#x}u", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_u_suffix_min_u64() {
    let value = i64::MAX as u64 + 1;
    let input = format!("{:#x}u", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_u_suffix_max_u64() {
    let value = u64::MAX;
    let input = format!("{:#x}u", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_u_suffix_overflow_u64_is_u32() {
    let value = u64::MAX as i128 + 1;
    let input = format!("{:#x}u", value);

    let (exp_t, act_t, act_sz) = exp_u32_and_actual(0, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_l_suffix_min_i32_is_i64() {
    let value = 0;
    let input = format!("{:#x}l", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_l_suffix_max_i32_is_i64() {
    let value = i32::MAX as i64;
    let input = format!("{:#x}l", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_l_suffix_min_u32_is_i64() {
    let value = i32::MAX as i64 + 1;
    let input = format!("{:#x}l", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_l_suffix_max_u32_is_i64() {
    let value = u32::MAX as i64;
    let input = format!("{:#x}l", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_l_suffix_min_i64() {
    let value = u32::MAX as i64 + 1;
    let input = format!("{:#x}l", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_l_suffix_max_i64() {
    let value = i64::MAX;
    let input = format!("{:#x}u", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_l_suffix_min_u64() {
    let value = i64::MAX as u64 + 1;
    let input = format!("{:#x}l", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_l_suffix_max_u64() {
    let value = u64::MAX;
    let input = format!("{:#x}l", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_l_suffix_overflow_u64_is_u64() {
    let value = u64::MAX as i128 + 1;
    let input = format!("{:#x}l", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(0, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_ul_suffix_min_i32_is_u64() {
    let value = 0;
    let input = format!("{:#x}ul", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_ul_suffix_max_i32_is_u64() {
    let value = i32::MAX as u64;
    let input = format!("{:#x}ul", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_ul_suffix_min_u32_is_u64() {
    let value = i32::MAX as u64 + 1;
    let input = format!("{:#x}ul", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_ul_suffix_max_u32_is_u64() {
    let value = u32::MAX as u64;
    let input = format!("{:#x}ul", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_ul_suffix_min_i64_is_u64() {
    let value = u32::MAX as u64 + 1;
    let input = format!("{:#x}ul", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_ul_suffix_max_i64_is_u64() {
    let value = i64::MAX as u64;
    let input = format!("{:#x}ul", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_ul_suffix_min_u64() {
    let value = i64::MAX as u64 + 1;
    let input = format!("{:#x}ul", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_ul_suffix_max_u64() {
    let value = u64::MAX;
    let input = format!("{:#x}ul", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_ul_suffix_overflow_u64_is_u64() {
    let value = u64::MAX as i128 + 1;
    let input = format!("{:#x}ul", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(0, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_hex_ul_suffix_dot_terminates_before_dot() {
    let value = 1;
    let input = format!("{:#x}ul.", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 5;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}