use super::*;

#[test]
fn test_float_literal_hex_no_exponent() {
    let (exp_t, act_t, act_sz) = unknown_and_actual("0x1.");
    let exp_sz = 4;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_exponent_no_digits() {
    let (exp_t, act_t, act_sz) = unknown_and_actual("0x1.p");
    let exp_sz = 5;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);

}
#[test]
fn test_float_literal_hex_no_digits() {
    let (exp_t, act_t, act_sz) = unknown_and_actual("0x.p");
    let exp_sz = 5;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_exponent_pos_no_digits() {
    let (exp_t, act_t, act_sz) = unknown_and_actual("0x1.p+");
    let exp_sz = 5;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_exponent_neg_no_digits() {
    let (exp_t, act_t, act_sz) = unknown_and_actual("0x1.p-");
    let exp_sz = 5;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_no_suffix_0_p0() {
    let value_bits: u64 = 0b0__000_0000_0000__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let value = f64::from_bits(value_bits);
    let (exp_t, act_t, act_sz) = exp_f64_and_actual(value, "0x0.p0");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_no_suffix_1_p0() {
    let value_bits: u64 = 0b0__011_1111_1111__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let value = f64::from_bits(value_bits);
    let (exp_t, act_t, act_sz) = exp_f64_and_actual(value, "0x1.p0");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_no_suffix_1_p4() {
    let value_bits: u64 = 0b0__100_0000_0011__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let value = f64::from_bits(value_bits);
    let (exp_t, act_t, act_sz) = exp_f64_and_actual(value, "0x1.p4");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_no_suffix_min() {
    let value_bits: u64 = 0b0__000_0000_0000__0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001;
    let value = f64::from_bits(value_bits);

    let (exp_t, act_t, act_sz) = exp_f64_and_actual(value, "0x.0000000000002p-1023");
    let exp_sz = 22;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_no_suffix_1_max() {
    let value_bits: u64 = 0b0__111_1111_1110__1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
    let value = f64::from_bits(value_bits);
    let (exp_t, act_t, act_sz) = exp_f64_and_actual(value, "0x1.fffffffffffffp1023");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_no_suffix_no_integer() {
    let value_bits: u64 = 0b0__011_1111_1111__1110_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let value = f64::from_bits(value_bits);
    let (exp_t, act_t, act_sz) = exp_f64_and_actual(value, "0x.Fp1");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_no_suffix_no_point() {
    let value_bits: u64 = 0b0__100_0000_0011__1110_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let value = f64::from_bits(value_bits);
    let (exp_t, act_t, act_sz) = exp_f64_and_actual(value, "0xFp1");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_no_suffix_int_point() {
    let value_bits: u64 = 0b0__100_0000_0111__1110_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let value = f64::from_bits(value_bits);
    let (exp_t, act_t, act_sz) = exp_f64_and_actual(value, "0xF.p1");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_no_suffix_upcase_p() {
    let value_bits: u64 = 0b0__100_0000_0111__1110_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let value = f64::from_bits(value_bits);
    let (exp_t, act_t, act_sz) = exp_f64_and_actual(value, "0xF.P1");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_no_suffix_p_pos() {
    let value_bits: u64 = 0b0__100_0000_0111__1110_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let value = f64::from_bits(value_bits);
    let (exp_t, act_t, act_sz) = exp_f64_and_actual(value, "0xF.p+1");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_no_suffix_p_neg() {
    let value_bits: u64 = 0b0__100_0000_0001__1110_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let value = f64::from_bits(value_bits);
    let (exp_t, act_t, act_sz) = exp_f64_and_actual(value, "0xF.p-1");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_f_suffix_0_p0() {
    let value_bits: u32 = 0b0__000_0000_0__000_0000_0000_0000_0000_0000;
    let value = f32::from_bits(value_bits);
    let (exp_t, act_t, act_sz) = exp_f32_and_actual(value, "0x0.p0f");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_f_suffix_1_p0() {
    let value_bits: u32 = 0b0__0111_1111__000_0000_0000_0000_0000_0000;
    let value = f32::from_bits(value_bits);
    let (exp_t, act_t, act_sz) = exp_f32_and_actual(value, "0x1.p0f");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_upcase_f_suffix_1_p4() {
    let value_bits: u32 = 0b0__1000_0011__000_0000_0000_0000_0000_0000;
    let value = f32::from_bits(value_bits);
    let (exp_t, act_t, act_sz) = exp_f32_and_actual(value, "0x1.p4F");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_f_suffix_min() {
    let value_bits: u32 = 0b0__0000_0000__000_0000_0000_0000_0000_0001;
    let value = f32::from_bits(value_bits);

    let (exp_t, act_t, act_sz) = exp_f32_and_actual(value, "0x.000004p-127f");
    let exp_sz = 22;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_f_suffix_1_max() {
    let value_bits: u32 = 0b0__1111_1110__111_1111_1111_1111_1111_1111;
    let value = f32::from_bits(value_bits);
    let (exp_t, act_t, act_sz) = exp_f32_and_actual(value, "0x1.fffffep127f");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_f_suffix_no_integer() {
    let value_bits: u32 = 0b0__0111_1111__111_0000_0000_0000_0000_0000;
    let value = f32::from_bits(value_bits);
    let (exp_t, act_t, act_sz) = exp_f32_and_actual(value, "0x.Fp1f");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_f_suffix_no_point() {
    let value_bits: u32 = 0b0__1000_0011__111_0000_0000_0000_0000_0000;
    let value = f32::from_bits(value_bits);
    let (exp_t, act_t, act_sz) = exp_f32_and_actual(value, "0xFp1f");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_f_suffix_int_point() {
    let value_bits: u32 = 0b0__1000_0011__111_0000_0000_0000_0000_0000;
    let value = f32::from_bits(value_bits);
    let (exp_t, act_t, act_sz) = exp_f32_and_actual(value, "0xF.p1f");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_f_suffix_upcase_p() {
    let value_bits: u32 = 0b0__1000_0011__111_0000_0000_0000_0000_0000;
    let value = f32::from_bits(value_bits);
    let (exp_t, act_t, act_sz) = exp_f32_and_actual(value, "0xF.P1f");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_f_suffix_p_pos() {
    let value_bits: u32 = 0b0__1000_0011__111_0000_0000_0000_0000_0000;
    let value = f32::from_bits(value_bits);
    let (exp_t, act_t, act_sz) = exp_f32_and_actual(value, "0xF.p+1f");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_f_suffix_p_neg() {
    let value_bits: u32 = 0b0__1000_0001__111_0000_0000_0000_0000_0000;
    let value = f32::from_bits(value_bits);
    let (exp_t, act_t, act_sz) = exp_f32_and_actual(value, "0xF.p-1f");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_l_suffix_0_p0() {
    let value: u128 = 0b0__000_0000_0000_0000__0__000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let (exp_t, act_t, act_sz) = exp_f80_and_actual(value, "0x0.p0l");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_l_suffix_1_p0() {
    let value: u128 = 0b0__011_1111_1111_1111__1__000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let (exp_t, act_t, act_sz) = exp_f80_and_actual(value, "0x1.p0l");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_upcase_l_suffix_1_p4() {
    let value: u128 = 0b0__100_0000_0000_0011__1__000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let (exp_t, act_t, act_sz) = exp_f80_and_actual(value, "0x1.p4L");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_l_suffix_min() {
    let value: u128 = 0b0__000_0000_0000_0000__0__000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001;
    let (exp_t, act_t, act_sz) = exp_f80_and_actual(value, "0x1p-16445l");
    let exp_sz = 22;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_l_suffix_1_max() {
    let value: u128 = 0b0__111_1111_1111_1110__1__111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
    let (exp_t, act_t, act_sz) = exp_f80_and_actual(value, "0x1.fffffffffffffffep+16383l");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_l_suffix_no_integer() {
    let value: u128 = 0b0__011_1111_1111_1111__1__111_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let (exp_t, act_t, act_sz) = exp_f80_and_actual(value, "0x.Fp1l");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_l_suffix_no_point() {
    let value: u128 = 0b0__100_0000_0000_0011__1__111_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let (exp_t, act_t, act_sz) = exp_f80_and_actual(value, "0xFp1f");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_l_suffix_int_point() {
    let value: u128 = 0b0__100_0000_0000_0011__1__111_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let (exp_t, act_t, act_sz) = exp_f80_and_actual(value, "0xF.p1f");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_l_suffix_upcase_p() {
    let value: u128 = 0b0__100_0000_0000_0011__1__111_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let (exp_t, act_t, act_sz) = exp_f80_and_actual(value, "0xF.P1f");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_l_suffix_p_pos() {
    let value: u128 = 0b0__100_0000_0000_0011__1__111_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let (exp_t, act_t, act_sz) = exp_f80_and_actual(value, "0xF.p+1f");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_l_suffix_p_neg() {
    let value: u128 = 0b0__100_0000_0000_0001__1__111_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let (exp_t, act_t, act_sz) = exp_f80_and_actual(value, "0xF.p-1f");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_fl_suffix_rejected() {
    let (exp_t, act_t, act_sz) = unknown_and_actual("0xf.p1fl");
    let exp_sz = 8;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_abc_suffix_rejected() {
    let (exp_t, act_t, act_sz) = unknown_and_actual("0xf.p1abc");
    let exp_sz = 8;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_f_suffix_dot_rejected() {
    let (exp_t, act_t, act_sz) = unknown_and_actual("0xf.p1f.");
    let exp_sz = 8;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}