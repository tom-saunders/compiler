use crate::lexer::ConstantType::Int32;
use crate::lexer::ConstantType::Int64;
use crate::lexer::ConstantType::Uint32;
use crate::lexer::ConstantType::Uint64;
use crate::lexer::LocatedToken;
use crate::lexer::Location;
use crate::lexer::Span;
use crate::lexer::Token;
use crate::lexer::Token::Constant;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::character::complete::digit0;
use nom::character::complete::hex_digit1;
use nom::character::complete::oct_digit0;
use nom::character::complete::oct_digit1;
use nom::character::complete::satisfy;
use nom::combinator::not;
use nom::combinator::peek;
use nom::combinator::recognize;
use nom::error;
use nom::error::Error;
use nom::error::ErrorKind;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::IResult;

fn is_nonzero_digit(c: char) -> bool {
    ('1'..='9').contains(&c)
}

fn int_constant_dec(input: Span) -> IResult<Span, LocatedToken> {
    let (rest, matched) = error::context(
        "int_constant_dec",
        recognize(
            pair(satisfy(is_nonzero_digit), digit0)
        ),
    )(input)?;
    match matched.fragment().parse::<i64>() {
        Ok(value) => Ok((
            rest,
            LocatedToken::of(Location::from(&matched), dec_int_width(value)),
        )),
        Err(_) => Err(nom::Err::Error(Error::new(input, ErrorKind::TooLarge))),
    }
}

fn int_constant_hex(input: Span) -> IResult<Span, LocatedToken> {
    let (rest, matched) = error::context(
        "int_constant_hex",
        preceded(tag("0x"), hex_digit1),
    )(input)?;
    match u64::from_str_radix(matched.fragment(), 16) {
        Ok(value) => Ok((
            rest,
            LocatedToken::of(Location::from(&matched), nondec_int_width(value)),
        )),
        Err(_) => Err(nom::Err::Error(Error::new(input, ErrorKind::TooLarge))),
    }
}

fn int_constant_oct(input: Span) -> IResult<Span, LocatedToken> {
    let (rest, matched) = error::context(
        "int_constant_oct",
            recognize(pair(tag("0"), oct_digit0))
        ,
    )(input)?;
    match u64::from_str_radix(matched.fragment(), 8) {
        Ok(value) => Ok((
            rest,
            LocatedToken::of(Location::from(&matched), nondec_int_width(value)),
        )),
        Err(_) => Err(nom::Err::Error(Error::new(input, ErrorKind::TooLarge))),
    }
}

fn dec_int_width(v64: i64) -> Token {
    match i32::try_from(v64) {
        Ok(v32) => Constant(Int32(v32)),
        Err(_) => Constant(Int64(v64)),
    }
}

fn nondec_int_width(q64: u64) -> Token {
    match i32::try_from(q64) {
        Ok(v32) => Constant(Int32(v32)),
        Err(_) => match u32::try_from(q64) {
            Ok(q32) => Constant(Uint32(q32)),
            Err(_) => match i64::try_from(q64) {
                Ok(v64) => Constant(Int64(v64)),
                Err(_) => Constant(Uint64(q64)),
            }
        },
    }
}

pub fn int_constant(input: Span) -> IResult<Span, LocatedToken> {
    error::context(
        "int_constant",
        terminated(
            alt((int_constant_dec, int_constant_hex, int_constant_oct)),
            peek(not(alphanumeric1))
        ),
    )(input)
}

#[cfg(test)]
mod test {
    use super::int_constant;
    use super::int_constant_dec;
    use super::int_constant_hex;
    use super::int_constant_oct;

    use crate::lexer::ConstantType;
    use crate::lexer::ConstantType::Int32;
    use crate::lexer::ConstantType::Int64;
    use crate::lexer::ConstantType::Uint32;
    use crate::lexer::ConstantType::Uint64;
    use crate::lexer::LocatedToken;
    use crate::lexer::Span;
    use crate::lexer::Token;
    use crate::lexer::Token::Constant;

    #[test]
    fn test_zero() {
        let s = Span::from("0 a");

        let (rest, loc_token) = match int_constant(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("Expected to match token of Constant(Int32(0)) but got err: {e}"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Int32(0));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

    #[test]
    fn test_zero_not_dec() {
        let s = Span::from("0 a");

        match int_constant_dec(s) {
            Ok((r, t)) => panic!("Expected failure to match token but matched {t:?}"),
            Err(e) => (),
        }
    }

    #[test]
    fn test_zero_is_oct() {
        let s = Span::from("0 a");

        let (rest, loc_token) = match int_constant_oct(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("Expected to match token of Constant(Int32(0)) but got err: {e}"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Int32(0));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

    #[test]
    fn test_above_range_dec() {
        // int64_max is     9223372036854775807 (i.e. 2^63 -1)
        let s = Span::from("9223372036854775808 a");

        match int_constant_dec(s) {
            Ok((r, t)) => panic!("Expected match failure as constant value is too large"),
            Err(_) => (),
        }
    }

    #[test]
    fn test_above_range_hex() {
        // uint64_max is    0x0FFFFFFFFFFFFFFFF (i.e. 2^64 -1)
        let s = Span::from("0x10000000000000000 a");

        match int_constant_hex(s) {
            Ok((r, t)) => panic!("Expected match failure as constant value is too large"),
            Err(_) => (),
        }
    }

    #[test]
    fn test_above_range_oct() {
        // uint64_max is    01777777777777777777777 (i.e. 2^64 -1)
        let s = Span::from("02000000000000000000000 a");

        match int_constant_oct(s) {
            Ok((r, t)) => panic!("Expected match failure as constant value is too large"),
            Err(_) => (),
        }
    }

    #[test]
    fn test_sized_no_suffix_dec_min_int32() {
        let s = Span::from("1 a");

        let (rest, loc_token) = match int_constant_dec(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("Expected to match token of Constant(Int32(1)) but got err: {e}"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Int32(1));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

    #[test]
    fn test_sized_no_suffix_dec_max_int32() {
        // int32 max is     2147483647 (2^31 -1)
        let s = Span::from("2147483647 a");

        let (rest, loc_token) = match int_constant_dec(s) {
            Ok((r, t)) => (r, t),
            Err(e) => {
                panic!("Expected to match token of Constant(Int32(2147483647)) but got err: {e}")
            }
        };

        let token = loc_token.token;

        let exp_token = Constant(Int32(2147483647));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

    #[test]
    fn test_sized_no_suffix_dec_min_int64() {
        // int64 min is     2147483648 (2^31)
        let s = Span::from("2147483648 a");

        let (rest, loc_token) = match int_constant_dec(s) {
            Ok((r, t)) => (r, t),
            Err(e) => {
                panic!("Expected to match token of Constant(Int64(2147483648)) but got err: {e}")
            }
        };

        let token = loc_token.token;

        let exp_token = Constant(Int64(2147483648));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

    #[test]
    fn test_sized_no_suffix_dec_max_int64() {
        // int64 max is     9223372036854775807 (2^64-1)
        let s = Span::from("9223372036854775807 a");

        let (rest, loc_token) = match int_constant_dec(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!(
                "Expected to match token of Constant(Int64(9223372036854775807)) but got err: {e}"
            ),
        };

        let token = loc_token.token;

        let exp_token = Constant(Int64(9223372036854775807));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

     #[test]
    fn test_sized_no_suffix_oct_min_int32() {
        let s = Span::from("00000000000000000000000 a");

        let (rest, loc_token) = match int_constant_oct(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("Expected to match token of Constant(Int32(0)) but got err: {e}"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Int32(0));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

     #[test]
    fn test_sized_no_suffix_oct_max_int32() {
        let s = Span::from("017777777777 a");

        let (rest, loc_token) = match int_constant_oct(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("Expected to match token of Constant(Int32(017777777777)) but got err: {e}"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Int32(0o17777777777));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

     #[test]
    fn test_sized_no_suffix_oct_min_uint32() {
        let s = Span::from("020000000000 a");

        let (rest, loc_token) = match int_constant_oct(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("Expected to match token of Constant(Uint32(020000000000)) but got err: {e}"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Uint32(0o20000000000));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

     #[test]
    fn test_sized_no_suffix_oct_max_uint32() {
        let s = Span::from("037777777777 a");

        let (rest, loc_token) = match int_constant_oct(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("Expected to match token of Constant(Uint32(037777777777)) but got err: {e}"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Uint32(0o37777777777));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

     #[test]
    fn test_sized_no_suffix_oct_min_int64() {
        let s = Span::from("040000000000 a");

        let (rest, loc_token) = match int_constant_oct(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("Expected to match token of Constant(Int64(040000000000)) but got err: {e}"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Int64(0o40000000000));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

     #[test]
    fn test_sized_no_suffix_oct_max_int64() {
        let s = Span::from("0777777777777777777777 a");

        let (rest, loc_token) = match int_constant_oct(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("Expected to match token of Constant(Int64(0777777777777777777777)) but got err: {e}"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Int64(0o777777777777777777777));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

     #[test]
    fn test_sized_no_suffix_oct_min_uint64() {
        let s = Span::from("01000000000000000000000 a");

        let (rest, loc_token) = match int_constant_oct(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("Expected to match token of Constant(Uint64(01000000000000000000000)) but got err: {e}"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Uint64(0o1000000000000000000000));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

     #[test]
    fn test_sized_no_suffix_oct_max_uint64() {
        let s = Span::from("01777777777777777777777 a");

        let (rest, loc_token) = match int_constant_oct(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("Expected to match token of Constant(Uint64(01777777777777777777777)) but got err: {e}"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Uint64(0o1777777777777777777777));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

    

     #[test]
    fn test_sized_no_suffix_hex_min_int32() {
        let s = Span::from("0x0 a");

        let (rest, loc_token) = match int_constant_hex(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("Expected to match token of Constant(Int32(0)) but got err: {e}"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Int32(0));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

     #[test]
    fn test_sized_no_suffix_hex_max_int32() {
        let s = Span::from("0x7fffffff a");

        let (rest, loc_token) = match int_constant_hex(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("Expected to match token of Constant(Int32(0x7fffffff)) but got err: {e}"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Int32(0x7fffffff));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

     #[test]
    fn test_sized_no_suffix_hex_min_uint32() {
        let s = Span::from("0x80000000 a");

        let (rest, loc_token) = match int_constant_hex(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("Expected to match token of Constant(Uint32(0x80000000)) but got err: {e}"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Uint32(0x80000000));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

     #[test]
    fn test_sized_no_suffix_hex_max_uint32() {
        let s = Span::from("0xffffffff a");

        let (rest, loc_token) = match int_constant_hex(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("Expected to match token of Constant(Uint32(0xffffffff)) but got err: {e}"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Uint32(0xffffffff));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

     #[test]
    fn test_sized_no_suffix_hex_min_int64() {
        let s = Span::from("0x100000000 a");

        let (rest, loc_token) = match int_constant_hex(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("Expected to match token of Constant(Int64(0x100000000)) but got err: {e}"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Int64(0x100000000));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

     #[test]
    fn test_sized_no_suffix_hex_max_int64() {
        let s = Span::from("0x7fffffffffffffff a");

        let (rest, loc_token) = match int_constant_hex(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("Expected to match token of Constant(Int64(0x7fffffffffffffff)) but got err: {e}"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Int64(0x7fffffffffffffff));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

     #[test]
    fn test_sized_no_suffix_hex_min_uint64() {
        let s = Span::from("0x8000000000000000 a");

        let (rest, loc_token) = match int_constant_hex(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("Expected to match token of Constant(Uint64(0x8000000000000000)) but got err: {e}"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Uint64(0x8000000000000000));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

     #[test]
    fn test_sized_no_suffix_hex_max_uint64() {
        let s = Span::from("0xffffffffffffffff a");

        let (rest, loc_token) = match int_constant_hex(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("Expected to match token of Constant(Uint64(0xffffffffffffffff)) but got err: {e}"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Uint64(0xffffffffffffffff));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

    #[test]
    fn test_sized_u_suffix_min_uint32() {
        let s = Span::from("1u a");

        let (rest, loc_token) = match int_constant(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("expected to match token of Constant(Uint32(1)) but got err: {e})"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Uint32(1));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

    #[test]
    fn test_sized_U_suffix_min_uint32() {
        let s = Span::from("1U a");

        let (rest, loc_token) = match int_constant(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("expected to match token of Constant(Uint32(1)) but got err: {e})"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Uint32(1));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

    #[test]
    fn test_sized_l_suffix_min_int64() {
        let s = Span::from("1l a");

        let (rest, loc_token) = match int_constant(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("expected to match token of Constant(Int64(1)) but got err: {e})"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Int64(1));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

    #[test]
    fn test_sized_L_suffix_min_int64() {
        let s = Span::from("1L a");

        let (rest, loc_token) = match int_constant(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("expected to match token of Constant(Int64(1)) but got err: {e})"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Int64(1));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

    #[test]
    fn test_sized_ll_suffix_min_int64() {
        let s = Span::from("1ll a");

        let (rest, loc_token) = match int_constant(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("expected to match token of Constant(Int64(1)) but got err: {e})"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Int64(1));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

    #[test]
    fn test_sized_LL_suffix_min_int64() {
        let s = Span::from("1LL a");

        let (rest, loc_token) = match int_constant(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("expected to match token of Constant(Int64(1)) but got err: {e})"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Int64(1));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

    #[test]
    fn test_sized_lL_suffix_fails() {
        let s = Span::from("1lL a");

        match int_constant(s) {
            Ok((r, t)) => panic!("expected not to match token of Constant(Int64(1)) but got ({r}, {t:?})"),
            Err(e) => (),
        }
    }

    #[test]
    fn test_sized_Ll_suffix_fails() {
        let s = Span::from("1Ll a");

        match int_constant(s) {
            Ok((r, t)) => panic!("expected not to match token of Constant(Int64(1)) but got ({r}, {t:?})"),
            Err(e) => (),
        }
    }

    

    #[test]
    fn test_sized_ul_suffix_min_uint64() {
        let s = Span::from("1ul a");

        let (rest, loc_token) = match int_constant(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("expected to match token of Constant(Uint64(1)) but got err: {e})"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Uint64(1));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

    #[test]
    fn test_sized_lu_suffix_min_int64() {
        let s = Span::from("1lu a");

        let (rest, loc_token) = match int_constant(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("expected to match token of Constant(Uint64(1)) but got err: {e})"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Uint64(1));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

    #[test]
    fn test_sized_Ul_suffix_min_uint64() {
        let s = Span::from("1Ul a");

        let (rest, loc_token) = match int_constant(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("expected to match token of Constant(Uint64(1)) but got err: {e})"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Uint64(1));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

    #[test]
    fn test_sized_lU_suffix_min_int64() {
        let s = Span::from("1lU a");

        let (rest, loc_token) = match int_constant(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("expected to match token of Constant(Uint64(1)) but got err: {e})"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Uint64(1));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

    #[test]
    fn test_sized_uL_suffix_min_uint64() {
        let s = Span::from("1uL a");

        let (rest, loc_token) = match int_constant(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("expected to match token of Constant(Uint64(1)) but got err: {e})"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Uint64(1));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

    #[test]
    fn test_sized_Lu_suffix_min_int64() {
        let s = Span::from("1Lu a");

        let (rest, loc_token) = match int_constant(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("expected to match token of Constant(Uint64(1)) but got err: {e})"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Uint64(1));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

    #[test]
    fn test_sized_UL_suffix_min_uint64() {
        let s = Span::from("1UL a");

        let (rest, loc_token) = match int_constant(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("expected to match token of Constant(Uint64(1)) but got err: {e})"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Uint64(1));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

    #[test]
    fn test_sized_LU_suffix_min_int64() {
        let s = Span::from("1LU a");

        let (rest, loc_token) = match int_constant(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("expected to match token of Constant(Uint64(1)) but got err: {e})"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Uint64(1));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }
    
    #[test]
    fn test_sized_ull_suffix_min_uint64() {
        let s = Span::from("1ull a");

        let (rest, loc_token) = match int_constant(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("expected to match token of Constant(Uint64(1)) but got err: {e})"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Uint64(1));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

    #[test]
    fn test_sized_llu_suffix_min_int64() {
        let s = Span::from("1llu a");

        let (rest, loc_token) = match int_constant(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("expected to match token of Constant(Uint64(1)) but got err: {e})"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Uint64(1));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }
    
    #[test]
    fn test_sized_Ull_suffix_min_uint64() {
        let s = Span::from("1Ull a");

        let (rest, loc_token) = match int_constant(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("expected to match token of Constant(Uint64(1)) but got err: {e})"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Uint64(1));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

    #[test]
    fn test_sized_llU_suffix_min_int64() {
        let s = Span::from("1llU a");

        let (rest, loc_token) = match int_constant(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("expected to match token of Constant(Uint64(1)) but got err: {e})"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Uint64(1));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }
    
    #[test]
    fn test_sized_uLL_suffix_min_uint64() {
        let s = Span::from("1uLL a");

        let (rest, loc_token) = match int_constant(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("expected to match token of Constant(Uint64(1)) but got err: {e})"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Uint64(1));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

    #[test]
    fn test_sized_LLu_suffix_min_int64() {
        let s = Span::from("1LLu a");

        let (rest, loc_token) = match int_constant(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("expected to match token of Constant(Uint64(1)) but got err: {e})"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Uint64(1));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }
    
    #[test]
    fn test_sized_ULL_suffix_min_uint64() {
        let s = Span::from("1ULL a");

        let (rest, loc_token) = match int_constant(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("expected to match token of Constant(Uint64(1)) but got err: {e})"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Uint64(1));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

    #[test]
    fn test_sized_LLU_suffix_min_int64() {
        let s = Span::from("1LLU a");

        let (rest, loc_token) = match int_constant(s) {
            Ok((r, t)) => (r, t),
            Err(e) => panic!("expected to match token of Constant(Uint64(1)) but got err: {e})"),
        };

        let token = loc_token.token;

        let exp_token = Constant(Uint64(1));

        assert_eq!(&" a", rest.fragment());
        assert_eq!(exp_token, token);
    }

    #[test]
    fn test_trailing_junk_after_constant() {
        let s = Span::from("123abc a");

        match int_constant(s) {
            Ok((r, t)) => panic!("expected not to match token but matched: ({r}, {t:?})"),
            Err(e) => (),
        }
    }

}
