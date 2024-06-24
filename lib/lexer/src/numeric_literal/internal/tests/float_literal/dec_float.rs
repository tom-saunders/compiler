use super::*;

#[test]
fn test_float_literal_dec_no_suffix_no_exponent() {
    let (exp_t, act_t, act_sz) = exp_f64_and_actual(1.0, "1.0");
    let exp_sz = 3;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_dec_no_suffix_no_exponent_no_integer() {
    let (exp_t, act_t, act_sz) = exp_f64_and_actual(0.25, ".25");
    let exp_sz = 3;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_dec_no_suffix_no_exponent_no_fract() {
    let (exp_t, act_t, act_sz) = exp_f64_and_actual(1.0, "1.");
    let exp_sz = 2;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_dec_no_suffix_no_integer() {
    let (exp_t, act_t, act_sz) = exp_f64_and_actual(0.25, ".25e0");
    let exp_sz = 5;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_dec_no_suffix_no_fract() {
    let (exp_t, act_t, act_sz) = exp_f64_and_actual(1.0, "1e0");
    let exp_sz = 3;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_dec_no_suffix_no_fract_pos() {
    let (exp_t, act_t, act_sz) = exp_f64_and_actual(1.0, "1e+0");
    let exp_sz = 4;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_dec_no_suffix_no_fract_neg() {
    let (exp_t, act_t, act_sz) = exp_f64_and_actual(1.0, "1e-0");
    let exp_sz = 4;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_dec_no_suffix_from_oct() {
    let (exp_t, act_t, act_sz) = exp_f64_and_actual(1.0, "01.");
    let exp_sz = 3;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_dec_f_suffix_no_exponent() {
    let (exp_t, act_t, act_sz) = exp_f32_and_actual(1.0, "1.0f");
    let exp_sz = 4;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_dec_f_suffix_no_exponent_no_integer() {
    let (exp_t, act_t, act_sz) = exp_f32_and_actual(0.25, ".25f");
    let exp_sz = 4;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_dec_f_suffix_no_exponent_no_fract() {
    let (exp_t, act_t, act_sz) = exp_f32_and_actual(1.0, "1.f");
    let exp_sz = 3;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_dec_f_suffix_no_integer() {
    let (exp_t, act_t, act_sz) = exp_f32_and_actual(0.25, ".25e0f");
    let exp_sz = 6;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_dec_f_suffix_no_fract() {
    let (exp_t, act_t, act_sz) = exp_f32_and_actual(1.0, "1e0f");
    let exp_sz = 4;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_dec_f_suffix_no_fract_pos() {
    let (exp_t, act_t, act_sz) = exp_f32_and_actual(1.0, "1e+0f");
    let exp_sz = 5;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_dec_f_suffix_no_fract_neg() {
    let (exp_t, act_t, act_sz) = exp_f32_and_actual(1.0, "1e-0f");
    let exp_sz = 5;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_dec_f_suffix_from_oct() {
    let (exp_t, act_t, act_sz) = exp_f32_and_actual(1.0, "01.f");
    let exp_sz = 4;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[ignore = "long doubles unimplemented"]
fn test_float_literal_dec_l_suffix_no_exponent() {
    let value = 0b0__011_1111_1111_1101__1__000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let (exp_t, act_t, act_sz) = exp_f80_and_actual(value, "1.0l");
    let exp_sz = 2;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[ignore = "long doubles unimplemented"]
fn test_float_literal_dec_l_suffix_no_exponent_no_integer() {
    let value = 0b0__011_1111_1101_1101__1__000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let (exp_t, act_t, act_sz) = exp_f80_and_actual(value, ".25l");
    let exp_sz = 2;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[ignore = "long doubles unimplemented"]
fn test_float_literal_dec_l_suffix_no_exponent_no_fract() {
    let value = 0b0__011_1111_1111_1101__1__000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let (exp_t, act_t, act_sz) = exp_f80_and_actual(value, "1.l");
    let exp_sz = 2;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[ignore = "long doubles unimplemented"]
fn test_float_literal_dec_l_suffix_no_integer() {
    let value = 0b0__011_1111_1101_1101__1__000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let (exp_t, act_t, act_sz) = exp_f80_and_actual(value, ".25e1l");
    let exp_sz = 2;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[ignore = "long doubles unimplemented"]
fn test_float_literal_dec_l_suffix_no_fract() {
    let value = 0b0__011_1111_1111_1101__1__000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let (exp_t, act_t, act_sz) = exp_f80_and_actual(value, "1e0l");
    let exp_sz = 2;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[ignore = "long doubles unimplemented"]
fn test_float_literal_dec_l_suffix_no_fract_pos() {
    let value = 0b0__011_1111_1111_1101__1__000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let (exp_t, act_t, act_sz) = exp_f80_and_actual(value, "1e+0l");
    let exp_sz = 2;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[ignore = "long doubles unimplemented"]
fn test_float_literal_dec_l_suffix_no_fract_neg() {
    let value = 0b0__011_1111_1111_1101__1__000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let (exp_t, act_t, act_sz) = exp_f80_and_actual(value, "1e-0l");
    let exp_sz = 2;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
#[ignore = "long doubles unimplemented"]
fn test_float_literal_dec_l_suffix_from_oct() {
    let value = 0b0__011_1111_1111_1101__1__000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
    let (exp_t, act_t, act_sz) = exp_f80_and_actual(value, "01.l");
    let exp_sz = 2;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_dec_fl_suffix_rejected() {
    let (exp_t, act_t, act_sz) = unknown_and_actual("1.fl");
    let exp_sz = 4;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_abc_suffix_rejected() {
    let (exp_t, act_t, act_sz) = unknown_and_actual("1.abc");
    let exp_sz = 5;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_f_suffix_dot_rejected() {
    let (exp_t, act_t, act_sz) = unknown_and_actual("1.f.");
    let exp_sz = 4;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}

#[test]
fn test_float_literal_hex_f_no_suffix_dot_dot_rejected() {
    let (exp_t, act_t, act_sz) = unknown_and_actual("1..");
    let exp_sz = 3;

    assert_eq!(exp_t, act_t);
    assert_eq!(exp_sz, act_sz);
}
