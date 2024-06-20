use super::*;

#[test]
fn test_int_literal_dec_no_suffix_min_i32() {
    let value = 1;
    let input = format!("{}", value);

    let (exp_t, act_t, act_sz) = exp_i32_and_actual(value, &input);
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
    let value = i64::MAX as u64 + 1;
    let input = format!("{}", value);

    let (exp_t, act_t, act_sz) = exp_i32_and_actual(0, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_u_suffix_min_u32() {
    let value = 1;
    let input = format!("{}u", value);

    let (exp_t, act_t, act_sz) = exp_u32_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_u_suffix_max_u32() {
    let value = u32::MAX;
    let input = format!("{}u", value);

    let (exp_t, act_t, act_sz) = exp_u32_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_u_suffix_min_u64() {
    let value = u32::MAX as u64 + 1;
    let input = format!("{}u", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_u_suffix_max_u64() {
    let value = u64::MAX;
    let input = format!("{}u", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_u_suffix_overflow_u64() {
    let value = u64::MAX as i128 + 1;
    let input = format!("{}u", value);

    let (exp_t, act_t, act_sz) = exp_u32_and_actual(0, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_U_suffix_min_u32() {
    let value = 1;
    let input = format!("{}U", value);

    let (exp_t, act_t, act_sz) = exp_u32_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_U_suffix_max_u32() {
    let value = u32::MAX;
    let input = format!("{}U", value);

    let (exp_t, act_t, act_sz) = exp_u32_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_U_suffix_min_u64() {
    let value = u32::MAX as u64 + 1;
    let input = format!("{}U", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_U_suffix_max_u64() {
    let value = u64::MAX;
    let input = format!("{}U", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_U_suffix_overflow_u32() {
    let value = u64::MAX as i128 + 1;
    let input = format!("{}U", value);

    let (exp_t, act_t, act_sz) = exp_u32_and_actual(0, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_l_suffix_min_i32_is_i64() {
    let value = 1;
    let input = format!("{}l", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_l_suffix_max_i32_is_i64() {
    let value = i32::MAX as i64;
    let input = format!("{}l", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_l_suffix_min_i64() {
    let value = i32::MAX as i64 + 1;
    let input = format!("{}l", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_l_suffix_max_i64() {
    let value = i64::MAX;
    let input = format!("{}l", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_l_suffix_overflow_i64() {
    let value = u64::MAX as i128 + 1;
    let input = format!("{}l", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(0, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_L_suffix_min_i32_is_i64() {
    let value = 1;
    let input = format!("{}L", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_L_suffix_max_i32_is_i64() {
    let value = i32::MAX as i64;
    let input = format!("{}L", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_L_suffix_min_i64() {
    let value = i32::MAX as i64 + 1;
    let input = format!("{}L", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_L_suffix_max_i64() {
    let value = i64::MAX;
    let input = format!("{}L", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_L_suffix_overflow_i64() {
    let value = u64::MAX as i128 + 1;
    let input = format!("{}L", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(0, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_ll_suffix_min_i32_is_i64() {
    let value = 1;
    let input = format!("{}ll", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_ll_suffix_max_i32_is_i64() {
    let value = i32::MAX as i64;
    let input = format!("{}ll", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_ll_suffix_min_i64() {
    let value = i32::MAX as i64 + 1;
    let input = format!("{}ll", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_ll_suffix_max_i64() {
    let value = i64::MAX;
    let input = format!("{}ll", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_ll_suffix_overflow_i64() {
    let value = u64::MAX as i128 + 1;
    let input = format!("{}ll", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(0, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_LL_suffix_min_i32_is_i64() {
    let value = 1;
    let input = format!("{}LL", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_LL_suffix_max_i32_is_i64() {
    let value = i32::MAX as i64;
    let input = format!("{}LL", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_LL_suffix_min_i64() {
    let value = i32::MAX as i64 + 1;
    let input = format!("{}LL", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_LL_suffix_max_i64() {
    let value = i64::MAX;
    let input = format!("{}LL", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_LL_suffix_overflow_i64() {
    let value = u64::MAX as i128 + 1;
    let input = format!("{}LL", value);

    let (exp_t, act_t, act_sz) = exp_i64_and_actual(0, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_ul_suffix_min_i32_is_u64() {
    let value = 1;
    let input = format!("{}ul", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_ul_suffix_max_i32_is_u64() {
    let value = i32::MAX as u64;
    let input = format!("{}ul", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_ul_suffix_min_i64_is_u64() {
    let value = i32::MAX as u64 + 1;
    let input = format!("{}ul", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_ul_suffix_max_i64_is_u64() {
    let value = i64::MAX as u64;
    let input = format!("{}ul", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_ul_suffix_overflow_u64() {
    let value = u64::MAX as i128 + 1;
    let input = format!("{}ul", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(0, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_ull_suffix_min_i32_is_u64() {
    let value = 1;
    let input = format!("{}ull", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_ull_suffix_max_i32_is_u64() {
    let value = i32::MAX as u64;
    let input = format!("{}ull", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_ull_suffix_min_i64_is_u64() {
    let value = i32::MAX as u64 + 1;
    let input = format!("{}ull", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_ull_suffix_max_i64_is_u64() {
    let value = i64::MAX as u64;
    let input = format!("{}ull", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_ull_suffix_overflow_u64() {
    let value = u64::MAX as i128 + 1;
    let input = format!("{}ull", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(0, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_lu_suffix_min_i32_is_u64() {
    let value = 1;
    let input = format!("{}lu", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_lu_suffix_max_i32_is_u64() {
    let value = i32::MAX as u64;
    let input = format!("{}lu", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_lu_suffix_min_i64_is_u64() {
    let value = i32::MAX as u64 + 1;
    let input = format!("{}lu", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_lu_suffix_max_i64_is_u64() {
    let value = i64::MAX as u64;
    let input = format!("{}lu", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_lu_suffix_overflow_u64() {
    let value = u64::MAX as i128 + 1;
    let input = format!("{}lu", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(0, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_llu_suffix_min_i32_is_u64() {
    let value = 1;
    let input = format!("{}llu", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_llu_suffix_max_i32_is_u64() {
    let value = i32::MAX as u64;
    let input = format!("{}llu", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_dec_llu_suffix_min_i64_is_u64() {
    let value = i32::MAX as u64 + 1;
    let input = format!("{}llu", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_llu_suffix_max_i64_is_u64() {
    let value = i64::MAX as u64;
    let input = format!("{}llu", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_llu_suffix_overflow_u64() {
    let value = u64::MAX as i128 + 1;
    let input = format!("{}llu", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(0, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_Ul_suffix_min_i32_is_u64() {
    let value = 1;
    let input = format!("{}Ul", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_lU_suffix_min_i32_is_u64() {
    let value = 1;
    let input = format!("{}lU", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_Ull_suffix_min_i32_is_u64() {
    let value = 1;
    let input = format!("{}Ull", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_llU_suffix_min_i32_is_u64() {
    let value = 1;
    let input = format!("{}llU", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_uL_suffix_min_i32_is_u64() {
    let value = 1;
    let input = format!("{}uL", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_Lu_suffix_min_i32_is_u64() {
    let value = 1;
    let input = format!("{}Lu", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_uLL_suffix_min_i32_is_u64() {
    let value = 1;
    let input = format!("{}uLL", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_LLu_suffix_min_i32_is_u64() {
    let value = 1;
    let input = format!("{}LLu", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_UL_suffix_min_i32_is_u64() {
    let value = 1;
    let input = format!("{}UL", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_LU_suffix_min_i32_is_u64() {
    let value = 1;
    let input = format!("{}LU", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_ULL_suffix_min_i32_is_u64() {
    let value = 1;
    let input = format!("{}ULL", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_LLU_suffix_min_i32_is_u64() {
    let value = 1;
    let input = format!("{}LLU", value);

    let (exp_t, act_t, act_sz) = exp_u64_and_actual(value, &input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_Ll_suffix_unknown() {
    let value = 1;
    let input = format!("{}Ll", value);

    let (exp_t, act_t, act_sz) = unknown_and_actual(&input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_lL_suffix_unknown() {
    let value = 1;
    let input = format!("{}lL", value);

    let (exp_t, act_t, act_sz) = unknown_and_actual(&input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_lull_suffix_unknown() {
    let value = 1;
    let input = format!("{}lull", value);

    let (exp_t, act_t, act_sz) = unknown_and_actual(&input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_llul_suffix_unknown() {
    let value = 1;
    let input = format!("{}llul", value);

    let (exp_t, act_t, act_sz) = unknown_and_actual(&input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_abc_suffix_unknown() {
    let value = 1;
    let input = format!("{}abc", value);

    let (exp_t, act_t, act_sz) = unknown_and_actual(&input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[allow(non_snake_case)]
fn test_int_literal_dec_e_suffix_unknown() {
    let value = 1;
    let input = format!("{}e", value);

    let (exp_t, act_t, act_sz) = unknown_and_actual(&input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_int_literal_oct_ul_suffix_dot_rejected() {
    let value = 1;
    let input = format!("{}ul.", value);

    let (exp_t, act_t, act_sz) = unknown_and_actual(&input);
    let exp_sz = 1;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}
