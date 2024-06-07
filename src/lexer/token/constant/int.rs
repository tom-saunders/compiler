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
use nom::character::complete::one_of;
use nom::character::complete::satisfy;
use nom::combinator::map;
use nom::combinator::not;
use nom::combinator::opt;
use nom::combinator::peek;
use nom::combinator::recognize;
use nom::error;
use nom::error::Error;
use nom::error::ErrorKind;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::IResult;

fn is_nonzero_digit(c: char) -> bool {
    ('1'..='9').contains(&c)
}

enum SuffixType {
    UnsignedLong,
    Long,
    Unsigned,
    None,
}

fn int_suffix_unsigned(input: Span) -> IResult<Span, ()> {
    map(one_of("uU"), |_| ())(input)
}

fn int_suffix_long(input: Span) -> IResult<Span, ()> {
    map(alt((
        pair(tag("l"), opt(tag("l"))),
        pair(tag("L"), opt(tag("L"))),
    )), |_| ())(input)
}

fn int_suffix(input: Span) -> IResult<Span, SuffixType> {
    match int_suffix_unsigned(input) {
        Ok((r, _)) => match int_suffix_long(r) {
            Ok((s, _)) => Ok((s, SuffixType::UnsignedLong)),
            Err(_) => Ok((r, SuffixType::Unsigned)),
        },
        Err(_) => match int_suffix_long(input) {
            Ok((t, _)) => match int_suffix_unsigned(t) {
                Ok((u, _)) => Ok((u, SuffixType::UnsignedLong)),
                Err(_) => Ok((t, SuffixType::Long)),
            },
            Err(_) => Ok((input, SuffixType::None)),
        },
    }
}

fn int_constant_dec(input: Span) -> IResult<Span, LocatedToken> {
    let (rest, (matched, suffix)) = error::context(
        "int_constant_dec",
        pair(
            recognize(pair(satisfy(is_nonzero_digit), digit0)),
            int_suffix
        ),
    )(input)?;



    match suffix {
        SuffixType::None => {
            match matched.fragment().parse::<i64>() {
                Ok(value) => Ok((rest, LocatedToken::of(Location::from(&matched), int32_or_int64(value)))),
                Err(_) => Err(nom::Err::Error(Error::new(input, ErrorKind::TooLarge)))
            }
        },
        SuffixType::Long => {
            match matched.fragment().parse::<i64>() {
                Ok(value) => Ok((rest, LocatedToken::of(Location::from(&matched), int64(value)))),
                Err(_) => Err(nom::Err::Error(Error::new(input, ErrorKind::TooLarge)))
            }
        },
        SuffixType::Unsigned => {
            match matched.fragment().parse::<u64>() {
                Ok(value) => Ok((rest, LocatedToken::of(Location::from(&matched), uint32_or_uint64(value)))),
                Err(_) => Err(nom::Err::Error(Error::new(input, ErrorKind::TooLarge)))
            }
        },
        SuffixType::UnsignedLong => {
            match matched.fragment().parse::<u64>() {
                Ok(value) => Ok((rest, LocatedToken::of(Location::from(&matched), uint64(value)))),
                Err(_) => Err(nom::Err::Error(Error::new(input, ErrorKind::TooLarge)))
            }
        },
    }
}

fn int_constant_hex(input: Span) -> IResult<Span, LocatedToken> {
    let (rest, (matched, suffix)) = error::context(
        "int_constant_hex",
        pair(
            preceded(tag("0x"), hex_digit1),
            int_suffix
        )
    )(input)?;

    let value = u64::from_str_radix(matched.fragment(), 16).map_err(|_| nom::Err::Error(Error::new(input, ErrorKind::TooLarge)))?;
    match suffix {
        SuffixType::None => Ok((rest, LocatedToken::of(Location::from(&matched), int32_or_uint32_or_int64_or_uint64(value)))),
        SuffixType::Long => Ok((rest, LocatedToken::of(Location::from(&matched), int64_or_uint64(value)))),
        SuffixType::Unsigned => Ok((rest, LocatedToken::of(Location::from(&matched), uint32_or_uint64(value)))),
        SuffixType::UnsignedLong => Ok((rest, LocatedToken::of(Location::from(&matched), uint64(value)))),
    }
}

fn int_constant_oct(input: Span) -> IResult<Span, LocatedToken> {
    let (rest, (matched, suffix)) = error::context(
        "int_constant_oct",
         pair(recognize(pair(tag("0"), oct_digit0)), int_suffix)
        ,
    )(input)?;
    
    let value = u64::from_str_radix(matched.fragment(), 8).map_err(|_| nom::Err::Error(Error::new(input, ErrorKind::TooLarge)))?;
    match suffix {
        SuffixType::None => Ok((rest, LocatedToken::of(Location::from(&matched), int32_or_uint32_or_int64_or_uint64(value)))),
        SuffixType::Long => Ok((rest, LocatedToken::of(Location::from(&matched), int64_or_uint64(value)))),
        SuffixType::Unsigned => Ok((rest, LocatedToken::of(Location::from(&matched), uint32_or_uint64(value)))),
        SuffixType::UnsignedLong => Ok((rest, LocatedToken::of(Location::from(&matched), uint64(value)))),
    }
}

/// decimal literal with no suffix
fn int32_or_int64(v64: i64) -> Token {
    match v64 {
        0..=0x7fff_ffff => Constant(Int32(i32::try_from(v64).unwrap())),
        0x8000_0000..=0x7fff_ffff_ffff_ffff => Constant(Int64(v64)),
        _ => panic!("should never get negative values in int32_or_int64"),
    }
}

/// nondecimal literal with no suffix
fn int32_or_uint32_or_int64_or_uint64(v64: u64) -> Token {
    match v64 {
        0..=0x7fff_ffff => Constant(Int32(i32::try_from(v64).unwrap())),
        0x8000_0000..=0xffff_ffff => Constant(Uint32(u32::try_from(v64).unwrap())),
        0x1_0000_0000..=0x7fff_ffff_ffff_ffff => Constant(Int64(i64::try_from(v64).unwrap())),
        _ => Constant(Uint64(v64)),
    }
}

/// any literal with only u suffix
fn uint32_or_uint64(v64: u64) -> Token {
    match v64 {
        0..=0xffff_ffff => Constant(Uint32(u32::try_from(v64).unwrap())),
        _ => Constant(Uint64(v64)),
    }
}

/// decimal literal with only l suffix
fn int64(v64: i64) -> Token {
    match v64 {
        0..=0x7fff_ffff_ffff_ffff => Constant(Int64(v64)),
        _ => panic!("should never get negative values in int64"),
    }
}

/// nondecimal literal with only l suffix
fn int64_or_uint64(v64: u64) -> Token {
    match v64 {
        0..=0x7fff_ffff_ffff_ffff => Constant(Int64(i64::try_from(v64).unwrap())),
        _ => Constant(Uint64(v64)),
    }
}

/// literal with both u and l suffix
fn uint64(v64: u64) -> Token {
    Constant(Uint64(v64))
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

    use crate::lexer::ConstantType::Int32;
    use crate::lexer::ConstantType::Int64;
    use crate::lexer::ConstantType::Uint32;
    use crate::lexer::ConstantType::Uint64;
    use crate::lexer::Span;
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
            Ok((r, t)) => panic!("Expected failure to match token but matched ({r}, {t:?}"),
            Err(_) => (),
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
    fn test_dec_above_range() {
        // int64_max is     9223372036854775807 (i.e. 2^63 -1)
        let s = Span::from("9223372036854775808 a");

        match int_constant_dec(s) {
            Ok((r, t)) => panic!("Expected match failure as constant value is too large but matched ({r}, {t:?}"),
            Err(_) => (),
        }
    }

    #[test]
    fn test_dec_ul_suffix_above_range() {
        // uint64_max is    18446744073709551615 (i.e. 2^64 -1)
        let s = Span::from("18446744073709551616 a");

        match int_constant_dec(s) {
            Ok((r, t)) => panic!("Expected match failure as constant value is too large but matched ({r}, {t:?}"),
            Err(_) => (),
        }
    }

    #[test]
    fn test_hex_above_range() {
        // uint64_max is    0x0FFFFFFFFFFFFFFFF (i.e. 2^64 -1)
        let s = Span::from("0x10000000000000000 a");

        match int_constant_hex(s) {
            Ok((r, t)) => panic!("Expected match failure as constant value is too large but matched ({r}, {t:?}"),
            Err(_) => (),
        }
    }

    #[test]
    fn test_oct_above_range() {
        // uint64_max is    01777777777777777777777 (i.e. 2^64 -1)
        let s = Span::from("02000000000000000000000 a");

        match int_constant_oct(s) {
            Ok((r, t)) => panic!("Expected match failure as constant value is too large but matched ({r}, {t:?}"),
            Err(_) => (),
        }
    }

    #[test]
    fn test_dec_no_suffix_min_int32() {
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
    fn test_dec_no_suffix_max_int32() {
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
    fn test_no_suffix_dec_min_int64() {
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
    fn test_dec_no_suffix_max_int64() {
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
    fn test_oct_no_suffix_min_int32() {
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
    fn test_oct_no_suffix_max_int32() {
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
    fn test_oct_no_suffix_min_uint32() {
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
    fn test_oct_no_suffix_max_uint32() {
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
    fn test_oct_no_suffix_min_int64() {
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
    fn test_oct_no_suffix_max_int64() {
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
    fn test_oct_no_suffix_min_uint64() {
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
    fn test_oct_no_suffix_max_uint64() {
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
    fn test_hex_no_suffix_min_int32() {
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
    fn test_hex_no_suffix_max_int32() {
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
    fn test_hex_no_suffix_min_uint32() {
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
    fn test_hex_no_suffix_max_uint32() {
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
    fn test_hex_no_suffix_min_int64() {
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
    fn test_hex_no_suffix_max_int64() {
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
    fn test_hex_no_suffix_min_uint64() {
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
    fn test_hex_no_suffix_max_uint64() {
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
    fn test_dec_u_suffix_min_uint32() {
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
    #[allow(non_snake_case)]
    fn test_dec_U_suffix_min_uint32() {
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
    fn test_dec_l_suffix_min_int64() {
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
    #[allow(non_snake_case)]
    fn test_dec_L_suffix_min_int64() {
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
    fn test_dec_ll_suffix_min_int64() {
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
    #[allow(non_snake_case)]
    fn test_dec_LL_suffix_min_int64() {
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
    #[allow(non_snake_case)]
    fn test_dec_lL_suffix_fails() {
        let s = Span::from("1lL a");

        match int_constant(s) {
            Ok((r, t)) => panic!("expected not to match token of Constant(Int64(1)) but got ({r}, {t:?})"),
            Err(_) => (),
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_dec_Ll_suffix_fails() {
        let s = Span::from("1Ll a");

        match int_constant(s) {
            Ok((r, t)) => panic!("expected not to match token of Constant(Int64(1)) but got ({r}, {t:?})"),
            Err(_) => (),
        }
    }

    #[test]
    fn test_dec_ul_suffix_min_uint64() {
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
    fn test_dec_lu_suffix_min_uint64() {
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
    #[allow(non_snake_case)]
    fn test_dec_Ul_suffix_min_uint64() {
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
    #[allow(non_snake_case)]
    fn test_dec_lU_suffix_min_uint64() {
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
    #[allow(non_snake_case)]
    fn test_dec_uL_suffix_min_uint64() {
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
    #[allow(non_snake_case)]
    fn test_dec_Lu_suffix_min_uint64() {
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
    #[allow(non_snake_case)]
    fn test_dec_UL_suffix_min_uint64() {
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
    #[allow(non_snake_case)]
    fn test_dec_LU_suffix_min_uint64() {
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
    fn test_dec_ull_suffix_min_uint64() {
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
    fn test_dec_llu_suffix_min_uint64() {
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
    #[allow(non_snake_case)]
    fn test_dec_Ull_suffix_min_uint64() {
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
    #[allow(non_snake_case)]
    fn test_dec_llU_suffix_min_uint64() {
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
    #[allow(non_snake_case)]
    fn test_dec_uLL_suffix_min_uint64() {
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
    #[allow(non_snake_case)]
    fn test_dec_LLu_suffix_min_uint64() {
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
    #[allow(non_snake_case)]
    fn test_dec_ULL_suffix_min_uint64() {
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
    #[allow(non_snake_case)]
    fn test_dec_LLU_suffix_min_uint64() {
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
            Err(_) => (),
        }
    }

    #[test]
    fn test_dec_ulu_suffix_rejected() {
        let s = Span::from("1ulu a");

        match int_constant(s) {
            Ok((r, t)) => panic!("expected not to match token but matched: ({r}, {t:?})"),
            Err(_) => (),
        }
    }

    #[test]
    fn test_dec_lul_suffix_rejected() {
        let s = Span::from("1lul a");

        match int_constant(s) {
            Ok((r, t)) => panic!("expected not to match token but matched: ({r}, {t:?})"),
            Err(_) => (),
        }
    }

    #[test]
    fn test_hex_lul_suffix_rejected() {
        let s = Span::from("0x1lul a");

        match int_constant(s) {
            Ok((r, t)) => panic!("expected not to match token but matched: ({r}, {t:?})"),
            Err(_) => (),
        }
    }

    #[test]
    fn test_oct_lul_suffix_rejected() {
        let s = Span::from("01lul a");

        match int_constant(s) {
            Ok((r, t)) => panic!("expected not to match token but matched: ({r}, {t:?})"),
            Err(_) => (),
        }
    }

}
