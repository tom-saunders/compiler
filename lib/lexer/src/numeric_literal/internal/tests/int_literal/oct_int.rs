use super::*;

#[test]
fn test_int_literal_oct_no_suffix_min_i32() {
    // {:o} format does not include leading zero but we are outputting zero anyway
    // {:#o} format emits 0o17.... which isn't what we need
    let value = 0;
    let input = format!("{:o}", value);

    let (exp_t, act_t, act_sz) = exp_i32_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_oct_no_suffix_max_i32() {
    // {:o} format does not include leading zero
    // {:#o} format emits 0o17.... which isn't what we need
    let value = i32::MAX;
    let input = format!("0{:o}", value);

    let (exp_t, act_t, act_sz) = exp_i32_and_actual(value, &input);
    let exp_sz = 12;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_oct_no_suffix_min_u32() {
    // {:o} format does not include leading zero
    // {:#o} format emits 0o17.... which isn't what we need
    let value = i32::MAX as u32 + 1;
    let input = format!("0{:o}", value);

    let (exp_t, act_t, act_sz) = exp_u32_and_actual(value, &input);
    let exp_sz = 12;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_oct_no_suffix_max_u32() {
    // {:o} format does not include leading zero
    // {:#o} format emits 0o17.... which isn't what we need
    let value = u32::MAX;
    let input = format!("0{:o}", value);

    let (exp_t, act_t, act_sz) = exp_u32_and_actual(value, &input);
    let exp_sz = 12;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_oct_no_suffix_min_i64() {
    // {:o} format does not include leading zero
    // {:#o} format emits 0o17.... which isn't what we need
    let value = u32::MAX as i64 + 1;
    let input = format!("0{:o}", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 12;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_oct_no_suffix_max_i64() {
    // {:o} format does not include leading zero
    // {:#o} format emits 0o77.... which isn't what we need
    let value = i64::MAX;
    let input = format!("0{:o}", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 22;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_oct_no_suffix_min_u64() {
    // {:o} format does not include leading zero
    // {:#o} format emits 0o17.... which isn't what we need
    let value = i64::MAX as u64 + 1;
    let input = format!("0{:o}", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 23;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_oct_no_suffix_max_u64() {
    // {:o} format does not include leading zero
    // {:#o} format emits 0o77.... which isn't what we need
    let value = u64::MAX;
    let input = format!("0{:o}", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 23;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_oct_no_suffix_overflow_u64() {
    // {:o} format does not include leading zero
    // {:#o} format emits 0o10.... which isn't what we need
    let value = u64::MAX as i128 + 1;
    let input = format!("0{:o}", value);

    let (exp_t, act_t, act_sz) = exp_i32_and_actual(0, &input);
    let exp_sz = 23;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_oct_no_suffix_leading_zeros_i32() {
    let input = "00000000000000000000000000000000000000000000000000000000000000000000000000000001";

    let (exp_t, act_t, act_sz) = exp_i32_and_actual(1, input);
    let exp_sz = 80;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

/* ======================================
 * ======================================
 * ======================================
 * ======================================
 */

 #[test]
 fn test_int_literal_oct_u_suffix_min_i32_is_u32() {
     // {:o} format does not include leading zero but we are outputting zero anyway
     // {:#o} format emits 0o17.... which isn't what we need
     let value = 0;
     let input = format!("{:o}u", value);

     let (exp_t, act_t, act_sz) = exp_u32_and_actual(value, &input);
     let exp_sz = 2;

     assert_eq!(exp_t, act_t);
     assert_eq!(exp_sz, act_sz);
 }

 #[test]
 fn test_int_literal_oct_u_suffix_max_i32_is_u32() {
     // {:o} format does not include leading zero
     // {:#o} format emits 0o17.... which isn't what we need
     let value = i32::MAX as u32;
     let input = format!("0{:o}u", value);

     let (exp_t, act_t, act_sz) = exp_u32_and_actual(value, &input);
     let exp_sz = 13;

     assert_eq!(exp_t, act_t);
     assert_eq!(exp_sz, act_sz);
 }

 #[test]
 fn test_int_literal_oct_u_suffix_min_u32() {
     // {:o} format does not include leading zero
     // {:#o} format emits 0o17.... which isn't what we need
     let value = i32::MAX as u32 + 1;
     let input = format!("0{:o}u", value);

     let (exp_t, act_t, act_sz) = exp_u32_and_actual(value, &input);
     let exp_sz = 13;

     assert_eq!(exp_t, act_t);
     assert_eq!(exp_sz, act_sz);
 }

 #[test]
 fn test_int_literal_oct_u_suffix_max_u32() {
     // {:o} format does not include leading zero
     // {:#o} format emits 0o17.... which isn't what we need
     let value = u32::MAX;
     let input = format!("0{:o}u", value);

     let (exp_t, act_t, act_sz) = exp_u32_and_actual(value, &input);
     let exp_sz = 13;

     assert_eq!(exp_t, act_t);
     assert_eq!(exp_sz, act_sz);
 }

 #[test]
 fn test_int_literal_oct_u_suffix_min_i64_is_u64() {
     // {:o} format does not include leading zero
     // {:#o} format emits 0o17.... which isn't what we need
     let value = u32::MAX as u64 + 1;
     let input = format!("0{:o}u", value);

     let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
     let exp_sz = 13;

     assert_eq!(exp_t, act_t);
     assert_eq!(exp_sz, act_sz);
 }

 #[test]
 fn test_int_literal_oct_u_suffix_max_i64_is_u64() {
     // {:o} format does not include leading zero
     // {:#o} format emits 0o77.... which isn't what we need
     let value = i64::MAX as u64;
     let input = format!("0{:o}u", value);

     let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
     let exp_sz = 23;

     assert_eq!(exp_t, act_t);
     assert_eq!(exp_sz, act_sz);
 }

 #[test]
 fn test_int_literal_oct_u_suffix_min_u64() {
     // {:o} format does not include leading zero
     // {:#o} format emits 0o17.... which isn't what we need
     let value = i64::MAX as u64 + 1;
     let input = format!("0{:o}u", value);

     let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
     let exp_sz = 24;

     assert_eq!(exp_t, act_t);
     assert_eq!(exp_sz, act_sz);
 }

 #[test]
 fn test_int_literal_oct_u_suffix_max_u64() {
     // {:o} format does not include leading zero
     // {:#o} format emits 0o77.... which isn't what we need
     let value = u64::MAX;
     let input = format!("0{:o}u", value);

     let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
     let exp_sz = 24;

     assert_eq!(exp_t, act_t);
     assert_eq!(exp_sz, act_sz);
 }

 #[test]
 fn test_int_literal_oct_u_suffix_overflow_u64() {
     // {:o} format does not include leading zero
     // {:#o} format emits 0o10.... which isn't what we need
     let value = u64::MAX as i128 + 1;
     let input = format!("0{:o}u", value);

     let (exp_t, act_t, act_sz) = exp_u32_and_actual(0, &input);
     let exp_sz = 24;

     assert_eq!(exp_t, act_t);
     assert_eq!(exp_sz, act_sz);
 }

 /* ======================================
 * ======================================
 * ======================================
 * ======================================
 */

#[test]
fn test_int_literal_oct_l_suffix_min_i32_is_i64() {
    // {:o} format does not include leading zero but we are outputting zero anyway
    // {:#o} format emits 0o17.... which isn't what we need
    let value = 0;
    let input = format!("{:o}l", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 2;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_oct_l_suffix_max_i32_is_i64() {
    // {:o} format does not include leading zero
    // {:#o} format emits 0o17.... which isn't what we need
    let value = i32::MAX as i64;
    let input = format!("0{:o}l", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 13;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_oct_l_suffix_min_u32_is_i64() {
    // {:o} format does not include leading zero
    // {:#o} format emits 0o17.... which isn't what we need
    let value = i32::MAX as i64 + 1;
    let input = format!("0{:o}l", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 13;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_oct_l_suffix_max_u32_is_i64() {
    // {:o} format does not include leading zero
    // {:#o} format emits 0o17.... which isn't what we need
    let value = u32::MAX as i64;
    let input = format!("0{:o}l", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 13;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_oct_l_suffix_min_i64() {
    // {:o} format does not include leading zero
    // {:#o} format emits 0o17.... which isn't what we need
    let value = u32::MAX as i64 + 1;
    let input = format!("0{:o}l", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 13;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_oct_l_suffix_max_i64() {
    // {:o} format does not include leading zero
    // {:#o} format emits 0o77.... which isn't what we need
    let value = i64::MAX;
    let input = format!("0{:o}l", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 23;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_oct_l_suffix_min_u64() {
    // {:o} format does not include leading zero
    // {:#o} format emits 0o17.... which isn't what we need
    let value = i64::MAX as u64 + 1;
    let input = format!("0{:o}l", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 24;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_oct_l_suffix_max_u64() {
    // {:o} format does not include leading zero
    // {:#o} format emits 0o77.... which isn't what we need
    let value = u64::MAX;
    let input = format!("0{:o}l", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 24;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_oct_l_suffix_overflow_u64_is_i64() {
    // {:o} format does not include leading zero
    // {:#o} format emits 0o10.... which isn't what we need
    let value = u64::MAX as i128 + 1;
    let input = format!("0{:o}l", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(0, &input);
    let exp_sz = 24;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

/* ======================================
 * ======================================
 * ======================================
 * ======================================
 */

#[test]
fn test_int_literal_oct_ul_suffix_min_i32_is_u64() {
    // {:o} format does not include leading zero but we are outputting zero anyway
    // {:#o} format emits 0o17.... which isn't what we need
    let value = 0;
    let input = format!("{:o}ul", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 3;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_oct_ul_suffix_max_i32_is_u64() {
    // {:o} format does not include leading zero
    // {:#o} format emits 0o17.... which isn't what we need
    let value = i32::MAX as u64;
    let input = format!("0{:o}ul", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 14;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_oct_ul_suffix_min_u32_is_u64() {
    // {:o} format does not include leading zero
    // {:#o} format emits 0o17.... which isn't what we need
    let value = i32::MAX as u64 + 1;
    let input = format!("0{:o}ul", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 14;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_oct_ul_suffix_max_u32_is_u64() {
    // {:o} format does not include leading zero
    // {:#o} format emits 0o17.... which isn't what we need
    let value = u32::MAX as u64;
    let input = format!("0{:o}ul", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 14;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_oct_ul_suffix_min_i64_is_u64() {
    // {:o} format does not include leading zero
    // {:#o} format emits 0o17.... which isn't what we need
    let value = u32::MAX as u64 + 1;
    let input = format!("0{:o}ul", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 14;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_oct_ul_suffix_max_i64_is_u64() {
    // {:o} format does not include leading zero
    // {:#o} format emits 0o77.... which isn't what we need
    let value = i64::MAX as u64;
    let input = format!("0{:o}ul", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 24;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_oct_ul_suffix_min_u64() {
    // {:o} format does not include leading zero
    // {:#o} format emits 0o17.... which isn't what we need
    let value = i64::MAX as u64 + 1;
    let input = format!("0{:o}ul", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 25;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_oct_ul_suffix_max_u64() {
    // {:o} format does not include leading zero
    // {:#o} format emits 0o77.... which isn't what we need
    let value = u64::MAX;
    let input = format!("0{:o}ul", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 25;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_oct_ul_suffix_overflow_u64_is_u64() {
    // {:o} format does not include leading zero
    // {:#o} format emits 0o10.... which isn't what we need
    let value = u64::MAX as i128 + 1;
    let input = format!("0{:o}ul", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(0, &input);
    let exp_sz = 25;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_oct_ul_suffix_dot_rejected() {
    // {:o} format does not include leading zero
    // {:#o} format emits 0o10.... which isn't what we need
    let value = 1;
    let input = format!("0{:o}ul.", value);

    let (exp_t, act_t, act_sz) = unknown_and_actual(&input);
    let exp_sz = 5;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}
