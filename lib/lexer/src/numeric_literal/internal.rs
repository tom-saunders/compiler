#[cfg(test)]
mod tests;

use std::cell::RefCell;
use std::fmt::Debug;
use std::iter::Peekable;
use std::num::ParseIntError;
use std::str::Chars;
use std::str::FromStr;

use crate::Token;
use crate::LocationState;

pub trait NumericLiteral {
    fn consume_numeric_literal(&self) -> Token;
}

pub trait NumericState {
    fn peek(&self) -> Option<char>;
    fn next(&self) -> Option<char>;

    fn emit_unknown(&self) -> Token;

    fn report_error(&self);
    fn seen_error(&self) -> bool;

    fn chars_consumed(&self) -> usize;
}

pub fn numeric_literal_impl<'iter>(
    location: &'iter dyn LocationState,
    numeric: &'iter dyn NumericState,
) -> Box<dyn NumericLiteral + 'iter> {
    Box::new(NumericLiteralImpl::new(location, numeric))
}

pub fn numeric_state_impl<'iter>(
    iter: Peekable<Chars<'iter>>,
) -> Box<dyn NumericState + 'iter> {
    Box::new(NumericStateImpl::new(iter,))
}

struct NumericLiteralImpl<'iter> {
    location: &'iter dyn LocationState,
    numeric: &'iter dyn NumericState,
    debug: bool,
}

impl<'iter> NumericLiteralImpl<'iter> {
    fn new(
        location: &'iter dyn LocationState,
        numeric: &'iter dyn NumericState,
    ) -> NumericLiteralImpl<'iter> {
        NumericLiteralImpl{location, numeric, debug: true}
    }
}

fn parse_dec_int_no_suffix(loc: &dyn LocationState, seen: &str) -> Token {
    let parsed = u128::from_str_radix(seen, 10);
    match parsed {
        Ok(u) => {
            println!("parsed ok: {u}");
            if (i64::MAX as u128) < u {
                eprintln!("{}:{}:{} - warn - value outside range of i64 will be truncated", loc.f(), loc.l(), loc.c());
            }
            let u = u as i64;
            if (i32::MIN as i64 <= u) && (u <= i32::MAX as i64) {
                println!("IntLitI32 as {} <= {} <= {}", i32::MIN, u, i32::MAX);
                Token::IntLitI32(u as i32)
            } else {
                println!("IntLitI64 as {} < {} || {} < {}", u, i32::MIN, i32::MAX, u);
                Token::IntLitI64(u)
            }
        }
        Err(e) => {
            eprintln!("{}:{}:{} - error - value {} cannot be parsed as a u128?: {}", loc.f(), loc.l(), loc.c(), seen, e);
            Token::Unknown(seen.to_string())
        }
    }
}

fn parse_dec_int_u_suffix(loc: &dyn LocationState, seen: &str, u: &str) -> Token {
    let parsed = u128::from_str_radix(seen, 10);
    match parsed {
        Ok(u) => {
            if (u64::MAX as u128) < u {
                eprintln!("{}:{}:{} - warn - value outside range of u64 will be truncated", loc.f(), loc.l(), loc.c());
            }
            let u = u as u64;

            if u <= u32::MAX as u64 {
                Token::IntLitU32(u as u32)
            } else{
                Token::IntLitU64(u as u64)
            }
        }
        Err(e) => {
            eprintln!("{}:{}:{} - error - value {} cannot be parsed as a u128?: {}", loc.f(), loc.l(), loc.c(), seen, e);
            let value = seen.to_string() + u;
            Token::Unknown(value)
        }
    }
}

fn parse_dec_int_l_suffix(loc: &dyn LocationState, seen: &str, l: &str) -> Token {
    let parsed = u128::from_str_radix(seen, 10);
    match parsed {
        Ok(u) => {
            if (i64::MAX as u128) < u {
                eprintln!("{}:{}:{} - warn - value outside range of i64 will be truncated", loc.f(), loc.l(), loc.c());
            }
            let u = u as i64;
            Token::IntLitI64(u as i64)
        }
        Err(e) => {
            eprintln!("{}:{}:{} - error - value {} cannot be parsed as a u128?: {}", loc.f(), loc.l(), loc.c(), seen, e);
            let value = seen.to_string() + l;
            Token::Unknown(value)
        }
    }
}

fn parse_dec_int_lu_suffix(loc: &dyn LocationState, seen: &str, suff_1: &str, suff_2: &str) -> Token {
    let parsed = u128::from_str_radix(seen, 10);
    match parsed {
        Ok(u) => {
            if (u64::MAX as u128) < u {
                eprintln!("{}:{}:{} - warn - value outside range of u64 will be truncated", loc.f(), loc.l(), loc.c());
            }
            let u = u as u64;
            Token::IntLitU64(u)
        }
        Err(e) => {
            eprintln!("{}:{}:{} - error - value {} cannot be parsed as a u128?: {}", loc.f(), loc.l(), loc.c(), seen, e);
            let value = seen.to_string() + suff_1 + suff_1;
            Token::Unknown(value)
        }
    }
}

fn int_hex_or_oct_no_suffix(loc: &dyn LocationState, pre: &str, seen: &str, parsed: Result<u128, ParseIntError>) -> Token {
    match parsed {
        Ok(u) => {
            if (u64::MAX as u128) < u {
                eprintln!("{}:{}:{} - warn - value outside range of u64 will be truncated", loc.f(), loc.l(), loc.c());
            }
            let u = u as u64;
            if u <= i32::MAX as u64 {
                Token::IntLitI32(u as i32)
            } else if u <= u32::MAX as u64 {
                Token::IntLitU32(u as u32)
            } else if u <= i64::MAX as u64 {
                Token::IntLitI64(u as i64)
            } else {
                Token::IntLitU64(u as u64)
            }
        }
        Err(e) => {
            eprintln!("{}:{}:{} - error - value {} cannot be parsed as a u128?: {}", loc.f(), loc.l(), loc.c(), seen, e);
            let value = pre.to_string() + seen;
            Token::Unknown(value)
        }
    }
}

fn int_hex_or_oct_l_suffix(loc: &dyn LocationState, pre: &str, seen: &str, l: &str, parsed: Result<u128, ParseIntError>) -> Token {
    match parsed {
        Ok(u) => {
            if (u64::MAX as u128) < u {
                eprintln!("{}:{}:{} - warn - value outside range of u64 will be truncated", loc.f(), loc.l(), loc.c());
            }
            let u = u as u64;

            if u <= i64::MAX as u64 {
                Token::IntLitI64(u as i64)
            } else {
                Token::IntLitU64(u)
            }
        }
        Err(e) => {
            eprintln!("{}:{}:{} - error - value {} cannot be parsed as a u128?: {}", loc.f(), loc.l(), loc.c(), seen, e);
            let value = pre.to_string() + seen + l;
            Token::Unknown(value)
        }
    }
}

fn int_hex_or_oct_lu_suffix(loc: &dyn LocationState, pre: &str, seen: &str, suff_1: &str, suff_2: &str, parsed: Result<u128, ParseIntError>) -> Token {
    match parsed {
        Ok(u) => {
            if (u64::MAX as u128) < u {
                eprintln!("{}:{}:{} - warn - value outside range of u64 will be truncated", loc.f(), loc.l(), loc.c());
            }
            let u = u as u64;
            Token::IntLitU64(u as u64)
        }
        Err(e) => {
            eprintln!("{}:{}:{} - error - value {} cannot be parsed as a u128?: {}", loc.f(), loc.l(), loc.c(), seen, e);
            let value = pre.to_string() + seen + suff_1 + suff_2;
            Token::Unknown(value)
        }
    }
}

fn int_hex_or_oct_u_suffix(loc: &dyn LocationState, pre: &str, seen: &str, u: &str, parsed: Result<u128, ParseIntError>) -> Token {
    match parsed {
        Ok(u) => {
            if (u64::MAX as u128) < u {
                eprintln!("{}:{}:{} - warn - value outside range of u64 will be truncated", loc.f(), loc.l(), loc.c());
            }
            let u = u as u64;
            if u <= u32::MAX as u64 {
                Token::IntLitU32(u as u32)
            } else {
                Token::IntLitU64(u)
            }
        }
        Err(e) => {
            eprintln!("{}:{}:{} - error - value {} cannot be parsed as a u128?: {}", loc.f(), loc.l(), loc.c(), seen, e);
            let value = pre.to_string() + seen + u;
            Token::Unknown(value)
        }
    }
}

fn parse_hex_int_no_suffix(loc: &dyn LocationState, pre: &str, seen: &str) -> Token {
    let parsed = u128::from_str_radix(seen, 16);
    int_hex_or_oct_no_suffix(loc, pre, seen, parsed)
}

fn parse_hex_int_l_suffix(loc: &dyn LocationState, pre: &str, seen: &str, l: &str) -> Token {
    let parsed = u128::from_str_radix(seen, 16);
    int_hex_or_oct_l_suffix(loc, pre, seen, l, parsed)
}

fn parse_hex_int_u_suffix(loc: &dyn LocationState, pre: &str, seen: &str, u: &str) -> Token {
    let parsed = u128::from_str_radix(seen, 16);
    int_hex_or_oct_u_suffix(loc, pre, seen, u, parsed)

}

fn parse_hex_int_lu_suffix(loc: &dyn LocationState, pre: &str, seen: &str, suff_1: &str, suff_2: &str) -> Token {
    let parsed = u128::from_str_radix(seen, 16);
    int_hex_or_oct_lu_suffix(loc, pre, seen, suff_1, suff_2, parsed)
}

fn parse_oct_int_no_suffix(loc: &dyn LocationState, seen: &str) -> Token {
    let parsed = u128::from_str_radix(seen, 8);
    int_hex_or_oct_no_suffix(loc, "", seen, parsed)
}

fn parse_oct_int_l_suffix(loc: &dyn LocationState, seen: &str, l: &str) -> Token {
    let parsed = u128::from_str_radix(seen, 8);
    int_hex_or_oct_l_suffix(loc, "", seen, l, parsed)
}

fn parse_oct_int_u_suffix(loc: &dyn LocationState, seen: &str, u: &str) -> Token {
    let parsed = u128::from_str_radix(seen, 8);
    int_hex_or_oct_u_suffix(loc, "", seen, u, parsed)
}

fn parse_oct_int_lu_suffix(loc: &dyn LocationState, seen: &str, suff_1: &str, suff_2: &str) -> Token {
    let parsed = u128::from_str_radix(seen, 8);
    int_hex_or_oct_lu_suffix(loc, "", seen, suff_1, suff_2, parsed)
}

fn parse_dec_float_no_suffix(loc: &dyn LocationState, seen: &str, e: &str, exp: &str) -> Token {
    let value = seen.to_string() + e + exp;
    let parsed = f64::from_str(&value);
    match parsed {
        Ok(val) => Token::FloatLit64(val),
        Err(e) => {
            eprintln!("{}:{}:{} - warn - unable to convert dec_float to f64: seen:[{}] e:[{}] exp:[{}]", loc.f(), loc.l(), loc.c(), seen, e, exp);
            Token::Unknown(value)
        }
    }
}

fn parse_dec_float_f_suffix(loc: &dyn LocationState, seen: &str, e: &str, exp: &str, f: &str) -> Token {
    let value = seen.to_string() + e + exp;
    let parsed = f32::from_str(&value);
    match parsed {
        Ok(val) => Token::FloatLit32(val),
        Err(e) => {
            eprintln!("{}:{}:{} - warn - unable to convert dec_float to f32: seen:[{}] e:[{}] exp:[{}]", loc.f(), loc.l(), loc.c(), seen, e, exp);
            Token::Unknown(value)
        }
    }
}

fn parse_dec_float_l_suffix(loc: &dyn LocationState, seen: &str, e: &str, exp: &str, l: &str) -> Token {
    eprintln!("{}:{}:{} - warn - unimplemented dec_float_l: seen:[{}] e:[{}] exp:[{}] l[{}]", loc.f(), loc.l(), loc.c(), seen, e, exp, l);
    let value = seen.to_string() + e + exp + l;
    Token::Unknown(value)
}

fn parse_hex_float_no_suffix(loc: &dyn LocationState, pre: &str, seen: &str, p: &str, exp: &str) -> Token {
    eprintln!("{}:{}:{} - warn - unimplemented hex_float: pre:[{}] seen:[{}] p:[{}] exp:[{}]", loc.f(), loc.l(), loc.c(), pre, seen, p, exp);
    let value = pre.to_string() + seen + p + exp;
    Token::Unknown(value)
}

fn parse_hex_float_f_suffix(loc: &dyn LocationState, pre: &str, seen: &str, p: &str, exp: &str, f: &str) -> Token {
    eprintln!("{}:{}:{} - warn - unimplemented hex_float_f: pre:[{}] seen:[{}] p:[{}] exp:[{}] f[{}]", loc.f(), loc.l(), loc.c(), pre, seen, p, exp, f);
    let value = pre.to_string() + seen + p + exp + f;
    Token::Unknown(value)
}

fn parse_hex_float_l_suffix(loc: &dyn LocationState, pre: &str, seen: &str, p: &str, exp: &str, l: &str) -> Token {
    eprintln!("{}:{}:{} - warn - unimplemented hex_float_l: pre:[{}] seen:[{}] p:[{}] exp:[{}] l[{}]", loc.f(), loc.l(), loc.c(), pre, seen, p, exp, l);
    let value = pre.to_string() + seen + p + exp + l;
    Token::Unknown(value)
}

trait NumericDfa : Debug {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token>;
}

fn init_dfa<'iter>(loc: &'iter dyn LocationState, peeked: Option<char>) -> Box<dyn NumericDfa + 'iter> {
    match peeked {
        Some(c @ '0') => {
            let seen = c.to_string();
            Box::new(InitZero{seen})
        }
        Some(c @ ('0' ..= '9')) => {
            let seen = String::from(c);
            Box::new(DecInt{seen})
        }
        Some(c @ '.') => {
            let seen = String::from(c);
            Box::new(InitDot{seen})
        }
        _ => panic!("{}:{}:{} - FATAL - this isn't a numeric literal", loc.f(), loc.l(), loc.c()),
    }
}

#[derive(Debug)]
struct InitZero {
    seen: String,
}

#[derive(Debug)]
struct InitDot {
    seen: String,
}

#[derive(Debug)]
struct InitZeroX {
    pref: String,
}

#[derive(Debug)]
struct DecInt {
    seen: String,
}

#[derive(Debug)]
struct DecIntL {
    seen: String,
    suff: String,
}

#[derive(Debug)]
struct DecIntLL {
    seen: String,
    suff: String,
}

#[derive(Debug)]
struct DecIntLLU {
    seen: String,
    suf1: String,
    suf2: String,
}

#[derive(Debug)]
struct DecIntLU {
    seen: String,
    suf1: String,
    suf2: String,
}

#[derive(Debug)]
struct DecIntU {
    seen: String,
    suff: String,
}

#[derive(Debug)]
struct OctInt {
    seen: String,
}

#[derive(Debug)]
struct OctDecInt {
    seen: String,
}

#[derive(Debug)]
struct OctIntL {
    seen: String,
    suff: String,
}

#[derive(Debug)]
struct OctIntLL {
    seen: String,
    suff: String,
}

#[derive(Debug)]
struct OctIntLLU {
    seen: String,
    suf1: String,
    suf2: String,
}

#[derive(Debug)]
struct OctIntLU {
    seen: String,
    suf1: String,
    suf2: String,
}

#[derive(Debug)]
struct OctIntU {
    seen: String,
    suff: String,
}

#[derive(Debug)]
struct HexInt {
    pref: String,
    seen: String,
}

#[derive(Debug)]
struct HexIntL {
    pref: String,
    seen: String,
    suff: String,
}

#[derive(Debug)]
struct HexIntLL {
    pref: String,
    seen: String,
    suff: String,
}

#[derive(Debug)]
struct HexIntLU {
    pref: String,
    seen: String,
    suf1: String,
    suf2: String,
}

#[derive(Debug)]
struct HexIntLLU {
    pref: String,
    seen: String,
    suf1: String,
    suf2: String,
}

#[derive(Debug)]
struct HexIntU {
    pref: String,
    seen: String,
    suff: String,
}

#[derive(Debug)]
struct DecFloat {
    seen: String,
}

#[derive(Debug)]
struct DecFloatF {
    seen: String,
    suff: String,
}

#[derive(Debug)]
struct DecFloatL {
    seen: String,
    suff: String,
}

#[derive(Debug)]
struct DecFloatExp {
    seen: String,
    e: String,
    exp: String,
}

#[derive(Debug)]
struct DecFloatExpF {
    seen: String,
    e: String,
    exp: String,
    suff: String
}

#[derive(Debug)]
struct DecFloatExpL {
    seen: String,
    e: String,
    exp: String,
    suff: String
}

#[derive(Debug)]
struct DecFloatExpSign {
    seen: String,
    e: String,
    exp: String,
}

#[derive(Debug)]
struct DecFloatExp_ {
    seen: String,
    e: String,
}

#[derive(Debug)]
struct HexFloat {
    pref: String,
    seen: String,
}

#[derive(Debug)]
struct HexFloatF {
    pref: String,
    seen: String,
    suff: String,
}

#[derive(Debug)]
struct HexFloatL {
    pref: String,
    seen: String,
    suff: String,
}

#[derive(Debug)]
struct HexFloatExp_ {
    pref: String,
    seen: String,
    p: String,
}

#[derive(Debug)]
struct HexFloatExpSign {
    pref: String,
    seen: String,
    p: String,
    exp: String,
}

#[derive(Debug)]
struct HexFloatExp {
    pref: String,
    seen: String,
    p: String,
    exp: String,
}

#[derive(Debug)]
struct HexFloatExpF {
    pref: String,
    seen: String,
    p: String,
    exp: String,
    suff: String
}

#[derive(Debug)]
struct HexFloatExpL {
    pref: String,
    seen: String,
    p: String,
    exp: String,
    suff: String
}

#[derive(Debug)]
struct Unkn {
    seen: String,
    suff: String,
}

impl NumericDfa for InitZero {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('0' ..= '7')) => {
                let mut next_seen = self.seen.clone();
                next_seen.push(c);
                Ok(Box::new(OctInt{seen: next_seen}))
            }
            Some(c @ ('8' | '9')) => {
                let mut next_seen = self.seen.clone();
                next_seen.push(c);
                Ok(Box::new(OctDecInt{seen: next_seen}))
            }
            Some(c @ '.') => {
                let mut next_seen = self.seen.clone();
                next_seen.push(c);
                Ok(Box::new(DecFloat{seen: next_seen}))
            }
            Some(c @ ('l' | 'L')) => {
                let seen = self.seen.clone();
                let suff = String::from(c);
                Ok(Box::new(OctIntL{seen, suff}))
            }
            Some(c @ ('u' | 'U')) => {
                let seen = self.seen.clone();
                let suff = String::from(c);
                Ok(Box::new(OctIntU{seen, suff}))
            }
            Some(c @ ('x' | 'X')) => {
                let mut next_seen = self.seen.clone();
                next_seen.push(c);
                Ok(Box::new(InitZeroX{pref: next_seen}))
            }
             Some(c @ ('a' ..= 'z' | 'A' ..= 'Z'| '_' )) => {
                let suff = String::from(c);
                Ok(Box::new(Unkn{seen: self.seen.clone(), suff}))
            }
            _ => {
                Err(parse_oct_int_no_suffix(loc, &self.seen))
            }
        }
    }
}

impl NumericDfa for InitDot {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('0' ..= '9')) => {
                let mut next_seen = self.seen.clone();
                next_seen.push(c);
                Ok(Box::new(DecFloat{seen: next_seen}))
            }
            Some(c @ ('a' ..= 'z' | 'A' ..= 'Z' | '_' | '.')) => {
                let seen = self.seen.clone();
                let suff = String::from(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                // this is a panic as it should really be a Token::Dot and never have entered here
                panic!("{}:{}:{} - FATAL - found '.' with no trailing digits in numeric_literal", loc.f(), loc.l(), loc.c());
            }
        }
    }
}

impl NumericDfa for InitZeroX {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('0' ..= '9' | 'a' ..= 'f' | 'A' ..= 'F')) => {
                let pref = self.pref.clone();
                let seen = String::from(c);
                Ok(Box::new(HexInt{pref, seen}))
            }
            Some(c @ '.') => {
                let pref = self.pref.clone();
                let seen = String::from(c);
                Ok(Box::new(HexFloat{pref, seen}))
            }
            Some(c @ ('a' ..= 'z' | 'A' ..= 'Z' | '_')) => {
                let seen = self.pref[..1].to_string();
                let mut suff = self.pref[1..].to_string();
                suff.push(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                eprintln!("{}:{}:{} - error - found hex prefix with no digits following", loc.f(), loc.l(), loc.c());
                let value = self.pref.clone();
                Err(Token::Unknown(value))
            }
        }
    }
}

impl NumericDfa for DecInt {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('0' ..= '9')) => {
                let mut seen = self.seen.clone();
                seen.push(c);
                Ok(Box::new(DecInt{seen}))
            }
            Some(c @ ('e' | 'E')) => {
                let seen = self.seen.clone();
                let e = String::from(c);
                Ok(Box::new(DecFloatExp_{seen, e}))
            }
            Some(c @ ('l' | 'L')) => {
                let seen = self.seen.clone();
                let suff = String::from(c);
                Ok(Box::new(DecIntL{seen, suff}))
            }
            Some(c @ ('u' | 'U')) => {
                let seen = self.seen.clone();
                let suff = String::from(c);
                Ok(Box::new(DecIntU{seen, suff}))
            }
            Some(c @ '.') => {
                let mut seen = self.seen.clone();
                seen.push(c);
                Ok(Box::new(DecFloat{seen}))
            }
            Some(c @ ('a' ..= 'z' | 'A' ..= 'Z' | '_')) => {
                let seen = self.seen.clone();
                let suff = String::from(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                Err(parse_dec_int_no_suffix(loc, &self.seen))
            }
        }
    }
}

impl NumericDfa for DecIntL {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('l' | 'L')) => {
                let previous = self.suff.chars().next().expect("There should always be a single char in suffix");
                if previous == c {
                    let seen = self.seen.clone();
                    let mut suff = self.suff.clone();
                    suff.push(c);
                    Ok(Box::new(DecIntLL{seen, suff}))
                } else {
                    let seen = self.seen.clone();
                    let mut suff = self.suff.clone();
                    suff.push(c);
                    Ok(Box::new(Unkn{seen, suff}))
                }
            }
            Some(c @ ('u' | 'U')) => {
                let seen = self.seen.clone();
                let suf1 = self.suff.clone();
                let suf2 = String::from(c);
                Ok(Box::new(DecIntLU{seen, suf1, suf2}))
            }
            Some(c @ ('0' ..= '9' |'a' ..= 'z' | 'A' ..= 'Z' | '_' | '.')) => {
                let seen = self.seen.clone();
                let mut suff = self.suff.clone();
                suff.push(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                Err(parse_dec_int_l_suffix(loc, &self.seen, &self.suff))
            }
        }
    }
}

impl NumericDfa for DecIntLL {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('u' | 'U')) => {
                let seen = self.seen.clone();
                let suf1 = self.suff.clone();
                let suf2 = String::from(c);
                Ok(Box::new(DecIntLLU{seen, suf1, suf2}))
            }
            Some(c @ ('0' ..= '9' |'a' ..= 'z' | 'A' ..= 'Z' | '_' | '.')) => {
                let seen = self.seen.clone();
                let mut suff = self.suff.clone();
                suff.push(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                Err(parse_dec_int_l_suffix(loc, &self.seen, &self.suff))
            }
        }
    }
}

impl NumericDfa for DecIntLLU {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('0' ..= '9' |'a' ..= 'z' | 'A' ..= 'Z' | '_' | '.')) => {
                let seen = self.seen.clone();
                let mut suff = self.suf1.clone();
                suff += &self.suf2;
                suff.push(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                Err(parse_dec_int_lu_suffix(loc, &self.seen, &self.suf1, &self.suf2))
            }
        }
    }
}

impl NumericDfa for DecIntLU {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('l' | 'L')) => {
                let previous = self.suf2.chars().next().expect("There should always be a single char in suffix");
                if previous == c {
                    let seen = self.seen.clone();
                    let suf1 = self.suf1.clone();
                    let mut suf2 = self.suf2.clone();
                    suf2.push(c);
                    Ok(Box::new(DecIntLLU{seen, suf1, suf2}))
                } else {
                    let seen = self.seen.clone();
                    let mut suff = self.suf1.clone();
                    suff += &self.suf2;
                    suff.push(c);
                    Ok(Box::new(Unkn{seen, suff}))
                }
            }
            Some(c @ ('0' ..= '9' |'a' ..= 'z' | 'A' ..= 'Z' | '_' | '.')) => {
                let seen = self.seen.clone();
                let mut suff = self.suf1.clone();
                suff += &self.suf2;
                suff.push(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                Err(parse_dec_int_lu_suffix(loc, &self.seen, &self.suf1, &self.suf2))
            }
        }
    }
}

impl NumericDfa for DecIntU {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('l' | 'L')) => {
                let seen = self.seen.clone();
                let suf1 = self.suff.clone();
                let suf2 = String::from(c);
                Ok(Box::new(DecIntLU{seen, suf1, suf2}))
            }
            Some(c @ ('0' ..= '9' |'a' ..= 'z' | 'A' ..= 'Z' | '_' | '.')) => {
                let seen = self.seen.clone();
                let mut suff = self.suff.clone();
                suff.push(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                Err(parse_dec_int_u_suffix(loc, &self.seen, &self.suff))
            }
        }
    }
}

impl NumericDfa for OctDecInt {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ '.') => {
                let mut seen = self.seen.clone();
                seen.push(c);
                Ok(Box::new(DecFloat{seen}))
            }
            Some(c @ ('0' ..= '9' |'a' ..= 'z' | 'A' ..= 'Z' | '_')) => {
                let seen = self.seen.clone();
                let suff = String::from(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                eprintln!("{}:{}:{} - error - found non-octal values in octal integer", loc.f(), loc.l(), loc.c());
                let seen = self.seen.clone();
                Err(Token::Unknown(seen))
            }
        }
    }
}

impl NumericDfa for OctInt {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('0' ..= '7')) => {
                let mut seen = self.seen.clone();
                seen.push(c);
                Ok(Box::new(OctInt{seen}))
            }
            Some(c @ ('8' | '9')) => {
                let mut seen = self.seen.clone();
                seen.push(c);
                Ok(Box::new(OctDecInt{seen}))
            }
            Some(c @ ('e' | 'E')) => {
                let seen = self.seen.clone();
                let e = String::from(c);
                Ok(Box::new(DecFloatExp_{seen, e}))
            }
            Some(c @ ('l' | 'L')) => {
                let seen = self.seen.clone();
                let suff = String::from(c);
                Ok(Box::new(OctIntL{seen, suff}))
            }
            Some(c @ ('u' | 'U')) => {
                let seen = self.seen.clone();
                let suff = String::from(c);
                Ok(Box::new(OctIntU{seen, suff}))
            }
            Some(c @ '.') => {
                let mut seen = self.seen.clone();
                seen.push(c);
                Ok(Box::new(DecFloat{seen}))
            }
            Some(c @ ('a' ..= 'z' | 'A' ..= 'Z' | '_')) => {
                let seen = self.seen.clone();
                let suff = String::from(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                Err(parse_oct_int_no_suffix(loc, &self.seen))
            }
        }
    }
}

impl NumericDfa for OctIntL {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('l' | 'L')) => {
                let seen = self.seen.clone();
                let mut suff = self.suff.clone();
                suff.push(c);

                let prev = self.suff.chars().next().expect("There should always be a single char in suffix");
                if prev == c {
                    Ok(Box::new(OctIntLL{seen, suff}))
                } else {
                    Ok(Box::new(Unkn{seen, suff}))
                }
            }
            Some(c @ ('u' | 'U')) => {
                let seen = self.seen.clone();
                let suf1 = self.suff.clone();
                let suf2 = String::from(c);
                Ok(Box::new(OctIntLU{seen, suf1, suf2}))
            }
            Some(c @ ('0' ..= '9' | 'a' ..= 'z' | 'A' ..= 'Z' | '_' | '.')) => {
                let seen = self.seen.clone();
                let mut suff = self.suff.clone();
                suff.push(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                Err(parse_oct_int_l_suffix(loc, &self.seen, &self.suff))
            }
        }
    }
}

impl NumericDfa for OctIntLL {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('u' | 'U')) => {
                let seen = self.seen.clone();
                let suf1 = self.suff.clone();
                let suf2 = String::from(c);
                Ok(Box::new(OctIntLLU{seen, suf1, suf2}))
            }
            Some(c @ ('0' ..= '9' | 'a' ..= 'z' | 'A' ..= 'Z' | '_' | '.')) => {
                let seen = self.seen.clone();
                let mut suff = self.suff.clone();
                suff.push(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                Err(parse_oct_int_l_suffix(loc, &self.seen, &self.suff))
            }
        }
    }
}

impl NumericDfa for OctIntLLU {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('0' ..= '9' | 'a' ..= 'z' | 'A' ..= 'Z' | '_' | '.')) => {
                let seen = self.seen.clone();
                let mut suff = self.suf1.clone();
                suff += &self.suf2;
                suff.push(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                Err(parse_oct_int_lu_suffix(loc, &self.seen, &self.suf1, &self.suf2))
            }
        }
    }
}

impl NumericDfa for OctIntLU {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('l' | 'L')) => {
                let seen = self.seen.clone();
                let suf1 = self.suf1.clone();
                let mut suf2 = self.suf2.clone();
                suf2.push(c);

                let prev = self.suf2.chars().next().expect("There should always be a single char in suffix");
                if prev == c {
                    Ok(Box::new(OctIntLLU{seen, suf1, suf2}))
                } else {
                    let suff = suf1 + &suf2;
                    Ok(Box::new(Unkn{seen, suff}))
                }
            }
            Some(c @ ('0' ..= '9' | 'a' ..= 'z' | 'A' ..= 'Z' | '_' | '.')) => {
                let seen = self.seen.clone();
                let mut suff = self.suf1.clone();
                suff += &self.suf2;
                suff.push(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                Err(parse_oct_int_lu_suffix(loc, &self.seen, &self.suf1, &self.suf2))
            }
        }
    }
}

impl NumericDfa for OctIntU {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('l' | 'L')) => {
                let seen = self.seen.clone();
                let suf1 = self.suff.clone();
                let suf2 = String::from(c);
                Ok(Box::new(OctIntLU{seen, suf1, suf2}))
            }
            Some(c @ ('0' ..= '9' | 'a' ..= 'z' | 'A' ..= 'Z' | '_' | '.')) => {
                let seen = self.seen.clone();
                let mut suff = self.suff.clone();
                suff.push(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                Err(parse_oct_int_u_suffix(loc, &self.seen, &self.suff))
            }
        }
    }
}

impl NumericDfa for HexInt {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('0' ..= '9' | 'a' ..= 'f' | 'A' ..= 'F')) => {
                let pref = self.pref.clone();
                let mut seen = self.seen.clone();
                seen.push(c);
                Ok(Box::new(HexInt{pref, seen}))
            }
            Some(c @ ('l' | 'L')) => {
                let pref = self.pref.clone();
                let seen = self.seen.clone();
                let suff = String::from(c);
                Ok(Box::new(HexIntL{pref, seen, suff}))
            }
            Some(c @ ('p' | 'P')) => {
                let pref = self.pref.clone();
                let seen = self.seen.clone();
                let p = String::from(c);
                Ok(Box::new(HexFloatExp_{pref, seen, p}))
            }
            Some(c @ ('u' | 'U')) => {
                let pref = self.pref.clone();
                let seen = self.seen.clone();
                let suff = String::from(c);
                Ok(Box::new(HexIntU{pref, seen, suff}))
            }
            Some(c @ '.') => {
                let pref = self.pref.clone();
                let mut seen = self.seen.clone();
                seen.push(c);
                Ok(Box::new(HexFloat{pref, seen}))
            }
            Some(c @ ('a' ..= 'z' | 'A' ..= 'Z' | '_')) => {
                let mut seen = self.pref.clone();
                seen += &self.seen;
                let suff = String::from(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                Err(parse_hex_int_no_suffix(loc, &self.pref, &self.seen))
            }
        }
    }
}

impl NumericDfa for HexIntL {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('l' | 'L')) => {
                let previous = self.suff.chars().next().expect("There should always be a single char in suffix");
                if previous == c {
                    let pref = self.pref.clone();
                    let seen = self.seen.clone();
                    let mut suff = self.suff.clone();
                    suff.push(c);
                    Ok(Box::new(HexIntLL{pref, seen, suff}))
                } else {
                    let mut seen = self.pref.clone();
                    seen += &self.seen;
                    let mut suff = self.suff.clone();
                    suff.push(c);
                    Ok(Box::new(Unkn{seen, suff}))
                }
            }
            Some(c @ ('u' | 'U')) => {
                let pref = self.pref.clone();
                let seen = self.seen.clone();
                let suf1 = self.suff.clone();
                let suf2 = String::from(c);
                Ok(Box::new(HexIntLU{pref, seen, suf1, suf2}))
            }
            Some(c @ ('0' ..= '9' | 'a' ..= 'z' | 'A' ..= 'Z' | '_' | '.')) => {
                let mut seen = self.pref.clone();
                seen += &self.seen;
                let mut suff = self.suff.clone();
                suff.push(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                Err(parse_hex_int_l_suffix(loc, &self.pref, &self.seen, &self.suff))
            }
        }
    }
}

impl NumericDfa for HexIntLL {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('u' | 'U')) => {
                let pref = self.pref.clone();
                let seen = self.seen.clone();
                let suf1 = self.suff.clone();
                let suf2 = String::from(c);
                Ok(Box::new(HexIntLLU{pref, seen, suf1, suf2}))
            }
            Some(c @ ('0' ..= '9' | 'a' ..= 'z' | 'A' ..= 'Z' | '_' | '.')) => {
                let mut seen = self.pref.clone();
                seen += &self.seen;
                let mut suff = self.suff.clone();
                suff.push(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                Err(parse_hex_int_l_suffix(loc, &self.pref, &self.seen, &self.suff))
            }
        }
    }
}

impl NumericDfa for HexIntLLU {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('0' ..= '9' | 'a' ..= 'z' | 'A' ..= 'Z' | '_' | '.')) => {
                let mut seen = self.pref.clone();
                seen += &self.seen;
                let mut suff = self.suf1.clone();
                suff += &self.suf2;
                suff.push(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                Err(parse_hex_int_lu_suffix(loc, &self.pref, &self.seen, &self.suf1, &self.suf2))
            }
        }
    }
}

impl NumericDfa for HexIntLU {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('l' | 'L')) => {
                let previous = self.suf2.chars().next().expect("There should always be a single char in suffix");
                if previous == c {
                    let pref = self.pref.clone();
                    let seen = self.seen.clone();
                    let suf1 = self.suf1.clone();
                    let mut suf2 = self.suf2.clone();
                    suf2.push(c);
                    Ok(Box::new(HexIntLLU{pref, seen, suf1, suf2}))
                } else {
                    let mut seen = self.pref.clone();
                    seen += &self.seen;
                    let mut suff = self.suf1.clone();
                    suff += &self.suf2;
                    suff.push(c);
                    Ok(Box::new(Unkn{seen, suff}))
                }
            }
            Some(c @ ('0' ..= '9' | 'a' ..= 'z' | 'A' ..= 'Z' | '_' | '.')) => {
                let mut seen = self.pref.clone();
                seen += &self.seen;
                let mut suff = self.suf1.clone();
                suff += &self.suf2;
                suff.push(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                Err(parse_hex_int_lu_suffix(loc, &self.pref, &self.seen, &self.suf1, &self.suf2))
            }
        }
    }
}

impl NumericDfa for HexIntU {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('l' | 'L')) => {
                let pref = self.pref.clone();
                let seen = self.seen.clone();
                let suf1 = self.suff.clone();
                let suf2 = String::from(c);
                Ok(Box::new(HexIntLU{pref, seen, suf1, suf2}))
            }
            Some(c @ ('0' ..= '9' | 'a' ..= 'z' | 'A' ..= 'Z' | '_' | '.')) => {
                let seen = self.seen.clone();
                let mut suff = self.suff.clone();
                suff.push(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                Err(parse_hex_int_u_suffix(loc, &self.pref, &self.seen, &self.suff))
            }
        }
    }
}

impl NumericDfa for DecFloat {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('0' ..= '9')) => {
                let mut seen = self.seen.clone();
                seen.push(c);
                Ok(Box::new(DecFloat{seen}))
            }
            Some(c @ ('e' | 'E')) => {
                let seen = self.seen.clone();
                let e = String::from(c);
                Ok(Box::new(DecFloatExp_{seen, e}))
            }
            Some(c @ ('f' | 'F')) => {
                let seen = self.seen.clone();
                let suff = String::from(c);
                Ok(Box::new(DecFloatF{seen, suff}))
            }
            Some(c @ ('l' | 'L')) => {
                let seen = self.seen.clone();
                let suff = String::from(c);
                Ok(Box::new(DecFloatL{seen, suff}))
            }
            Some(c @ ('a' ..= 'z' | 'A' ..= 'Z' | '_' | '.')) => {
                let seen = self.seen.clone();
                let suff = String::from(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                Err(parse_dec_float_no_suffix(loc, &self.seen, "", ""))
            }
        }
    }
}

impl NumericDfa for DecFloatExp {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('0' ..= '9')) => {
                let seen = self.seen.clone();
                let e = self.e.clone();
                let mut exp = self.exp.clone();
                exp.push(c);
                Ok(Box::new(DecFloatExp{seen, e, exp}))
            }
            Some(c @ ('f' | 'F')) => {
                let seen = self.seen.clone();
                let e = self.e.clone();
                let exp = self.exp.clone();
                let suff = String::from(c);
                Ok(Box::new(DecFloatExpF{seen, e, exp, suff}))
            }
            Some(c @ ('l' | 'L')) => {
                let seen = self.seen.clone();
                let e = self.e.clone();
                let exp = self.exp.clone();
                let suff = String::from(c);
                Ok(Box::new(DecFloatExpL{seen, e, exp, suff}))
            }
            Some(c @ ('a' ..= 'z' | 'A' ..= 'Z' | '_' | '.')) => {
                let mut seen = self.seen.clone();
                seen += &self.e;
                seen += &self.exp;
                let suff = String::from(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                Err(parse_dec_float_no_suffix(loc, &self.seen, &self.e, &self.exp))
            }
        }
    }
}

impl NumericDfa for DecFloatExpF {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('0' ..= '9' |'a' ..= 'z' | 'A' ..= 'Z' | '_' | '.')) => {
                let mut seen = self.seen.clone();
                seen += &self.e;
                seen += &self.exp;
                let mut suff = self.suff.clone();
                suff.push(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                Err(parse_dec_float_f_suffix(loc, &self.seen, &self.e, &self.exp, &self.suff))
            }
        }
    }
}

impl NumericDfa for DecFloatExpL {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('0' ..= '9' |'a' ..= 'z' | 'A' ..= 'Z' | '_' | '.')) => {
                let mut seen = self.seen.clone();
                seen += &self.e;
                seen += &self.exp;
                let mut suff = self.suff.clone();
                suff.push(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                Err(parse_dec_float_l_suffix(loc, &self.seen, &self.e, &self.exp, &self.suff))
            }
        }
    }
}

impl NumericDfa for DecFloatExpSign {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('0' ..= '9')) => {
                let seen = self.seen.clone();
                let e = self.e.clone();
                let mut exp = self.exp.clone();
                exp.push(c);
                Ok(Box::new(DecFloatExp{seen, e, exp}))
            }
            Some(c @ ('a' ..= 'z' | 'A' ..= 'Z' | '_' | '.')) => {
                let seen = self.seen.clone();
                let mut suff = self.e.clone();
                suff += &self.exp;
                suff.push(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                eprintln!("{}:{}:{} - error - exponent has no digits", loc.f(), loc.l(), loc.c());
                let mut value = self.seen.clone();
                value += &self.e;
                value += &self.exp;
                Err(Token::Unknown(value))
            }
        }
    }
}

impl NumericDfa for DecFloatExp_ {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('0' ..= '9')) => {
                let seen = self.seen.clone();
                let e = self.e.clone();
                let exp = String::from(c);
                Ok(Box::new(DecFloatExp{seen, e, exp}))
            }
            Some(c @ ('+' | '-')) => {
                let seen = self.seen.clone();
                let e = self.e.clone();
                let exp = String::from(c);
                Ok(Box::new(DecFloatExpSign{seen, e, exp}))
            }
            Some(c @ ('a' ..= 'z' | 'A' ..= 'Z' | '_' | '.')) => {
                let seen = self.seen.clone();
                let mut suff = self.e.clone();
                suff.push(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                eprintln!("{}:{}:{} - error - exponent has no digits", loc.f(), loc.l(), loc.c());
                let mut value = self.seen.clone();
                value += &self.e;
                Err(Token::Unknown(value))
            }
        }
    }
}

impl NumericDfa for DecFloatF {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('0' ..= '9' |'a' ..= 'z' | 'A' ..= 'Z' | '_' | '.')) => {
                let seen = self.seen.clone();
                let mut suff = self.suff.clone();
                suff.push(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                Err(parse_dec_float_f_suffix(loc, &self.seen, "", "", &self.suff))
            }
        }
    }
}

impl NumericDfa for DecFloatL {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('0' ..= '9' |'a' ..= 'z' | 'A' ..= 'Z' | '_' | '.')) => {
                let seen = self.seen.clone();
                let mut suff = self.suff.clone();
                suff.push(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                Err(parse_dec_float_l_suffix(loc, &self.seen, "", "", &self.suff))
            }
        }
    }
}

impl NumericDfa for HexFloat {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        todo!()
    }
}

impl NumericDfa for HexFloatExp {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        todo!()
    }
}

impl NumericDfa for HexFloatExpF {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        todo!()
    }
}

impl NumericDfa for HexFloatExpL {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        todo!()
    }
}

impl NumericDfa for HexFloatExpSign {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        todo!()
    }
}

impl NumericDfa for HexFloatExp_ {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        todo!()
    }
}

impl NumericDfa for HexFloatF {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        todo!()
    }
}

impl NumericDfa for HexFloatL {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        todo!()
    }
}

impl NumericDfa for Unkn {
    fn next(&self, loc: &dyn LocationState, peeked: Option<char>) -> Result<Box<dyn NumericDfa>, Token> {
        match peeked {
            Some(c @ ('0' ..= '9' | 'a' ..= 'z' | 'A' ..= 'Z' | '_' | '.')) => {
                let seen = self.seen.clone();
                let mut suff = self.suff.clone();
                suff.push(c);
                Ok(Box::new(Unkn{seen, suff}))
            }
            _ => {
                eprintln!("{}:{}:{} - error - unexpected suffix [{}] for numeric value {}{}", loc.f(), loc.l(), loc.c(), self.suff, self.seen, self.suff);
                let mut value = self.seen.clone();
                value += &self.suff;
                Err(Token::Unknown(value))
            }
        }
    }
}


impl<'iter> NumericLiteral for NumericLiteralImpl<'iter>{
    fn consume_numeric_literal(&self) -> Token {
        let peeked = self.numeric.peek();
        let mut dfa = init_dfa(self.location, peeked);
        self.numeric.next();

        loop {
            let peeked = self.numeric.peek();
            if self.debug {
                println!("{}:{}:{} - debug - inputs: ({:?}, {:?})", self.location.f(), self.location.l(), self.location.c(), dfa, peeked);
            }
            let dfa_or_token = dfa.next(self.location, peeked);
            match dfa_or_token {
                Ok(next_dfa) => {
                    self.numeric.next();
                    dfa = next_dfa
                }
                Err(token) => break token,
            }
        }
    }
}

struct NumericStateImpl<'iter> {
    iter: RefCell<Peekable<Chars<'iter>>>,
    consumed: RefCell<String>,
    seen_error: RefCell<bool>,
}

impl<'iter> NumericStateImpl<'iter> {
    fn new(
        iter: Peekable<Chars<'iter>>,
    ) -> NumericStateImpl<'iter> {
        NumericStateImpl{
            iter: RefCell::new(iter),
            consumed: RefCell::new(String::new()),
            seen_error: RefCell::new(false),
        }
    }
}

impl<'iter> NumericState for NumericStateImpl<'iter> {
    fn peek(&self) -> Option<char> {
        match self.iter.borrow_mut().peek() {
            None => None,
            Some(c) => Some(*c),
        }
    }

    fn next(&self) -> Option<char> {
        let r =  self.iter.borrow_mut().next();
        match r {
            Some(c) => self.consumed.borrow_mut().push(c),
            _ => ()
        };
        r
    }

    fn emit_unknown(&self) -> Token {
        Token::Unknown(self.consumed.borrow().clone())
    }

    fn report_error(&self) {
        *self.seen_error.borrow_mut() = true;
    }

    fn seen_error(&self) -> bool {
        *self.seen_error.borrow()
    }

    fn chars_consumed(&self) -> usize {
        self.consumed.borrow().chars().count()
    }
}