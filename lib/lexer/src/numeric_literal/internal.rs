#[cfg(test)]
mod tests;

use std::cell::RefCell;
use std::iter::Peekable;
use std::str::Chars;

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
}

impl<'iter> NumericLiteralImpl<'iter> {
    fn new(
        location: &'iter dyn LocationState,
        numeric: &'iter dyn NumericState,
    ) -> NumericLiteralImpl<'iter> {
        NumericLiteralImpl{location, numeric}
    }

    fn parse_dec_int_no_suffix(&self, seen: String) -> Token {
        let parsed = u128::from_str_radix(&seen, 10);
        match parsed {
            Ok(u) => {
                if (i64::MAX as u128) < u {
                    eprintln!("{}:{}:{} - warn - value outside range of i64 will be truncated", self.location.f(), self.location.l(), self.location.c());
                }
                let u = u as i64;
                if (i32::MIN as i64 <= u) && (u <= i32::MAX as i64) {
                    Token::IntLitI32(u as i32)
                } else {
                    Token::IntLitI64(u)
                }
            }
            Err(e) => {
                eprintln!("{}:{}:{} - error - value {} cannot be parsed as a u128?: {}", self.location.f(), self.location.l(), self.location.c(), seen, e);
                Token::Unknown(seen)
            }
        }
    }

    fn parse_dec_int_u_suffix(&self, seen: String) -> Token {
        let parsed = u128::from_str_radix(&seen, 10);
        match parsed {
            Ok(u) => {
                if (u64::MAX as u128) < u {
                    eprintln!("{}:{}:{} - warn - value outside range of u64 will be truncated", self.location.f(), self.location.l(), self.location.c());
                }
                let u = u as u64;

                if u <= u32::MAX as u64 {
                    Token::IntLitU32(u as u32)
                } else{
                    Token::IntLitU64(u as u64)
                }
            }
            Err(e) => {
                eprintln!("{}:{}:{} - error - value {} cannot be parsed as a u128?: {}", self.location.f(), self.location.l(), self.location.c(), seen, e);
                Token::Unknown(seen)
            }
        }
    }

    fn parse_dec_int_l_suffix(&self, seen: String) -> Token {
        let parsed = u128::from_str_radix(&seen, 10);
        match parsed {
            Ok(u) => {
                if (i64::MAX as u128) < u {
                    eprintln!("{}:{}:{} - warn - value outside range of i64 will be truncated", self.location.f(), self.location.l(), self.location.c());
                }
                let u = u as i64;
                Token::IntLitI64(u as i64)
            }
            Err(e) => {
                eprintln!("{}:{}:{} - error - value {} cannot be parsed as a u128?: {}", self.location.f(), self.location.l(), self.location.c(), seen, e);
                Token::Unknown(seen)
            }
        }
    }

    fn parse_dec_int_lu_suffix(&self, seen: String) -> Token {
        let parsed = u128::from_str_radix(&seen, 10);
        match parsed {
            Ok(u) => {
                if (u64::MAX as u128) < u {
                    eprintln!("{}:{}:{} - warn - value outside range of u64 will be truncated", self.location.f(), self.location.l(), self.location.c());
                }
                let u = u as u64;
                Token::IntLitU64(u)
            }
            Err(e) => {
                eprintln!("{}:{}:{} - error - value {} cannot be parsed as a u128?: {}", self.location.f(), self.location.l(), self.location.c(), seen, e);
                Token::Unknown(seen)
            }
        }
    }

    fn parse_oct_int_no_suffix(&self, seen: String) -> Token {
        let parsed = u128::from_str_radix(&seen, 8);
        match parsed {
            Ok(u) => {
                if (u64::MAX as u128) < u {
                    eprintln!("{}:{}:{} - warn - value outside range of u64 will be truncated", self.location.f(), self.location.l(), self.location.c());
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
                eprintln!("{}:{}:{} - error - value {} cannot be parsed as a u128?: {}", self.location.f(), self.location.l(), self.location.c(), seen, e);
                Token::Unknown(seen)
            }
        }
    }

    fn parse_oct_int_l_suffix(&self, seen: String) -> Token {
        let parsed = u128::from_str_radix(&seen, 8);
        match parsed {
            Ok(u) => {
                if (u64::MAX as u128) < u {
                    eprintln!("{}:{}:{} - warn - value outside range of u64 will be truncated", self.location.f(), self.location.l(), self.location.c());
                }
                let u = u as u64;

                if u <= i64::MAX as u64 {
                    Token::IntLitI64(u as i64)
                } else {
                    Token::IntLitU64(u)
                }
            }
            Err(e) => {
                eprintln!("{}:{}:{} - error - value {} cannot be parsed as a u128?: {}", self.location.f(), self.location.l(), self.location.c(), seen, e);
                Token::Unknown(seen)
            }
        }
    }

    fn parse_oct_int_u_suffix(&self, seen: String) -> Token {        let parsed = u128::from_str_radix(&seen, 8);
        match parsed {
            Ok(u) => {
                if (u64::MAX as u128) < u {
                    eprintln!("{}:{}:{} - warn - value outside range of u64 will be truncated", self.location.f(), self.location.l(), self.location.c());
                }
                let u = u as u64;
                if u <= u32::MAX as u64 {
                    Token::IntLitU32(u as u32)
                } else {
                    Token::IntLitU64(u)
                }
            }
            Err(e) => {
                eprintln!("{}:{}:{} - error - value {} cannot be parsed as a u128?: {}", self.location.f(), self.location.l(), self.location.c(), seen, e);
                Token::Unknown(seen)
            }
        }
    }

    fn parse_oct_int_lu_suffix(&self, seen: String) -> Token {        let parsed = u128::from_str_radix(&seen, 8);
        match parsed {
            Ok(u) => {
                if (u64::MAX as u128) < u {
                    eprintln!("{}:{}:{} - warn - value outside range of u64 will be truncated", self.location.f(), self.location.l(), self.location.c());
                }
                let u = u as u64;
                Token::IntLitU64(u as u64)
            }
            Err(e) => {
                eprintln!("{}:{}:{} - error - value {} cannot be parsed as a u128?: {}", self.location.f(), self.location.l(), self.location.c(), seen, e);
                Token::Unknown(seen)
            }
        }
    }
}

#[derive(Debug)]
enum NumericDfa {
    InitZero(String),
    InitDot(String),
    InitZeroX(String),
    OctInt(String),
    OctIntL(String, String),
    OctIntLL(String, String),
    OctIntU(String, String),
    OctIntLU(String, String, String),
    OctIntLLU(String, String, String),
    OctDecInt(String),
    DecInt(String),
    DecIntL(String, String),
    DecIntLL(String, String),
    DecIntU(String, String),
    DecIntLU(String, String, String),
    DecIntLLU(String, String, String),
    HexInt(String),
    HexIntL(String, String),
    HexIntLL(String, String),
    HexIntU(String, String),
    HexIntLU(String, String, String),
    HexIntLLU(String, String, String),
    DecFloat(String),
    DecFloatF(String, String),
    DecFloatL(String, String),
    DecFloatExp_(String),
    DecFloatExpSign(String, String),
    DecFloatExp(String, String),
    DecFloatExpF(String, String, String),
    DecFloatExpL(String, String, String),
    HexFloatNoExp(String),
    HexFloatExp_(String),
    HexFloatExpSign(String, String),
    HexFloatExp(String, String),
    HexFloatExpF(String, String, String),
    HexFloatExpL(String, String, String),
    Unkn(String, String),
}

impl<'iter> NumericLiteral for NumericLiteralImpl<'iter>{
    fn consume_numeric_literal(&self) -> Token {
        use NumericDfa::*;
        let mut dfa = match self.numeric.peek() {
            Some('0') => {
                self.numeric.next();
                InitZero("0".to_string())
            }
            Some(c @ ('0' ..= '9')) => {
                self.numeric.next();
                DecInt(String::from(c))
            }
            Some('.') => {
                self.numeric.next();
                InitDot(".".to_string())
            }
            _ => panic!("{}:{}:{} - FATAL - this isn't a numeric literal", self.location.f(), self.location.l(), self.location.c()),
        };

        loop {
            dfa = match (dfa, self.numeric.peek()) {
                (DecInt(mut seen), Some(c @ ('0' ..= '9'))) => {
                    self.numeric.next();
                    seen.push(c);
                    DecInt(seen)
                }
                (DecInt(mut seen), Some(c @ '.')) => {
                    self.numeric.next();
                    seen.push(c);
                    DecFloat(seen)
                }
                (DecInt(mut seen), Some(c @ ('e' | 'E'))) => {
                    self.numeric.next();
                    seen.push(c);
                    DecFloatExp_(seen)
                }
                (DecInt(seen), Some(c @ ('l' | 'L'))) => {
                    self.numeric.next();
                    DecIntL(seen, String::from(c))
                }
                (DecInt(seen), Some(c @ ('u' | 'U'))) => {
                    self.numeric.next();
                    DecIntU(seen, String::from(c))
                }
                (DecInt(seen), Some(c @ ('a' ..= 'z' | 'A' ..= 'A' | '_'))) => {
                    self.numeric.next();
                    let suff = String::from(c);
                    Unkn(seen, suff)
                }
                (DecInt(seen), _) => {
                    break self.parse_dec_int_no_suffix(seen)
                }
                (DecIntL(seen, mut l), Some(c @ ('l' | 'L'))) => {
                    self.numeric.next();
                    if l.chars().next().expect("l should never be an empty string") == c {
                        l.push(c);
                        DecIntLL(seen, l)
                    } else {
                        l.push(c);
                        Unkn(seen, l)
                    }
                }
                (DecIntL(seen, l), Some(c @ ('u' | 'U'))) => {
                    self.numeric.next();
                    DecIntLU(seen, l, String::from(c))
                }
                (DecIntL(seen, mut l), Some(c @ ('a' ..= 'z' | 'A' ..= 'A' | '0' ..= '9' | '_' | '.'))) => {
                    self.numeric.next();
                    l.push(c);
                    Unkn(seen, l)
                }
                (DecIntL(seen, _), _) => {
                    break self.parse_dec_int_l_suffix(seen)
                }
                (DecIntLL(seen, ll), Some(c @ ('u' | 'U'))) => {
                    self.numeric.next();
                    DecIntLLU(seen, ll, String::from(c))
                }
                (DecIntLL(seen, mut ll), Some(c @ ('a' ..= 'z' | 'A' ..= 'A' | '0' ..= '9' | '_' | '.'))) => {
                    self.numeric.next();
                    ll.push(c);
                    Unkn(seen, ll)
                }
                (DecIntLL(seen, _), _) => {
                    break self.parse_dec_int_l_suffix(seen)
                }
                (DecIntLLU(seen, mut first, second), Some(c @ ('a' ..= 'z' | 'A' ..= 'A' | '0' ..= '9' | '_' | '.'))) => {
                    self.numeric.next();
                    first += &second;
                    first.push(c);
                    Unkn(seen, first)
                }
                (DecIntLLU(seen, _, _), _) => {
                    break self.parse_dec_int_lu_suffix(seen)
                }
                (DecIntLU(seen, mut first, mut second), Some(c @ ('l' | 'L'))) => {
                    self.numeric.next();
                    if second.chars().next().expect("second should never be an empty string") == c {
                        second.push(c);
                        DecIntLLU(seen, first, second)
                    } else {
                        first += &second;
                        first.push(c);
                        Unkn(seen, first)
                    }
                }
                (DecIntLU(seen, mut first, second), Some(c @ ('a' ..= 'z' | 'A' ..= 'A' | '0' ..= '9' | '_' | '.'))) => {
                    self.numeric.next();
                    first += &second;
                    first.push(c);
                    Unkn(seen, first)
                }
                (DecIntLU(seen, _, _), _) => {
                    break self.parse_dec_int_lu_suffix(seen)
                }
                (DecIntU(seen, u), Some(c @ ('l' | 'L'))) => {
                    self.numeric.next();
                    DecIntLU(seen, u, String::from(c))
                }
                (DecIntU(seen, mut u), Some(c @ ('a' ..= 'z' | 'A' ..= 'A' | '0' ..= '9' | '_' | '.'))) => {
                    self.numeric.next();
                    u.push(c);
                    Unkn(seen, u)
                }
                (DecIntU(seen, _), _) => {
                    break self.parse_dec_int_u_suffix(seen)
                }
                (InitZero(mut seen), Some(c @ ('0' ..= '7'))) => {
                    self.numeric.next();
                    seen.push(c);
                    OctInt(seen)
                }
                (InitZero(mut seen), Some(c @ ('8' | '9'))) => {
                    self.numeric.next();
                    seen.push(c);
                    OctDecInt(seen)
                }
                (InitZero(mut seen), Some(c @ '.')) => {
                    self.numeric.next();
                    seen.push(c);
                    DecFloat(seen)
                }
                (InitZero( seen), Some(c @ ('l' | 'L'))) => {
                    self.numeric.next();
                    let suff = String::from(c);
                    OctIntL(seen, suff)
                }
                (InitZero( seen), Some(c @ ('u' | 'U'))) => {
                    self.numeric.next();
                    let suff = String::from(c);
                    OctIntU(seen, suff)
                }
                (InitZero(mut seen), Some(c @ ('x' | 'X'))) => {
                    self.numeric.next();
                    seen.push(c);
                    InitZeroX(seen)
                }
                (InitZero(seen), Some(c @ ('a' ..= 'z' | 'A' ..= 'A'))) => {
                    self.numeric.next();
                    let suff = String::from(c);
                    Unkn(seen, suff)
                }
                (InitZero(seen), _) => {
                    break self.parse_oct_int_no_suffix(seen)
                }
                (OctInt(mut seen), Some(c @ ('0' ..= '7'))) => {
                    self.numeric.next();
                    seen.push(c);
                    OctInt(seen)
                }
                (OctInt(mut seen), Some(c @ ('8' | '9'))) => {
                    self.numeric.next();
                    seen.push(c);
                    OctDecInt(seen)
                }
                (OctInt(mut seen), Some(c @ '.')) => {
                    self.numeric.next();
                    seen.push(c);
                    DecFloat(seen)
                }
                (OctInt( seen), Some(c @ ('l' | 'L'))) => {
                    self.numeric.next();
                    let suff = String::from(c);
                    OctIntL(seen, suff)
                }
                (OctInt( seen), Some(c @ ('u' | 'U'))) => {
                    self.numeric.next();
                    let suff = String::from(c);
                    OctIntU(seen, suff)
                }
                (OctInt(seen), Some(c @ ('a' ..= 'z' | 'A' ..= 'A'))) => {
                    self.numeric.next();
                    let suff = String::from(c);
                    Unkn(seen, suff)
                }
                (OctInt(seen), _) => {
                    break self.parse_oct_int_no_suffix(seen)
                }
                (OctIntL(seen, mut l), Some(c @ ('l' | 'L'))) => {
                    self.numeric.next();
                    if l.chars().next().expect("l should never be an empty string") == c {
                        l.push(c);
                        OctIntLL(seen, l)
                    } else {
                        l.push(c);
                        Unkn(seen, l)
                    }
                }
                (OctIntL(seen, l), Some(c @ ('u' | 'U'))) => {
                    self.numeric.next();
                    OctIntLU(seen, l, String::from(c))
                }
                (OctIntL(seen, mut l), Some(c @ ('a' ..= 'z' | 'A' ..= 'A' | '0' ..= '9' | '_' | '.'))) => {
                    self.numeric.next();
                    l.push(c);
                    Unkn(seen, l)
                }
                (OctIntL(seen, _), _) => {
                    break self.parse_oct_int_l_suffix(seen)
                }
                (OctIntLL(seen, ll), Some(c @ ('u' | 'U'))) => {
                    self.numeric.next();
                    OctIntLLU(seen, ll, String::from(c))
                }
                (OctIntLL(seen, mut ll), Some(c @ ('a' ..= 'z' | 'A' ..= 'A' | '0' ..= '9' | '_' | '.'))) => {
                    self.numeric.next();
                    ll.push(c);
                    Unkn(seen, ll)
                }
                (OctIntLL(seen, _), _) => {
                    break self.parse_oct_int_l_suffix(seen)
                }
                (OctIntLLU(seen, mut first, second), Some(c @ ('a' ..= 'z' | 'A' ..= 'A' | '0' ..= '9' | '_' | '.'))) => {
                    self.numeric.next();
                    first += &second;
                    first.push(c);
                    Unkn(seen, first)
                }
                (OctIntLLU(seen, _, _), _) => {
                    break self.parse_oct_int_lu_suffix(seen)
                }
                (OctIntLU(seen, mut first, mut second), Some(c @ ('l' | 'L'))) => {
                    self.numeric.next();
                    if second.chars().next().expect("second should never be an empty string") == c {
                        second.push(c);
                        OctIntLLU(seen, first, second)
                    } else {
                        first += &second;
                        first.push(c);
                        Unkn(seen, first)
                    }
                }
                (OctIntLU(seen, mut first, second), Some(c @ ('a' ..= 'z' | 'A' ..= 'A' | '0' ..= '9' | '_' | '.'))) => {
                    self.numeric.next();
                    first += &second;
                    first.push(c);
                    Unkn(seen, first)
                }
                (OctIntLU(seen, _, _), _) => {
                    break self.parse_oct_int_lu_suffix(seen)
                }
                (OctIntU(seen, u), Some(c @ ('l' | 'L'))) => {
                    self.numeric.next();
                    OctIntLU(seen, u, String::from(c))
                }
                (OctIntU(seen, mut u), Some(c @ ('a' ..= 'z' | 'A' ..= 'A' | '0' ..= '9' | '_' | '.'))) => {
                    self.numeric.next();
                    u.push(c);
                    Unkn(seen, u)
                }
                (OctIntU(seen, _), _) => {
                    break self.parse_oct_int_u_suffix(seen)
                }

                (Unkn(seen, mut suff), Some(c @ ('a' ..= 'z' | 'A' ..= 'A' | '0' ..= '9' | '_' | '.'))) => {
                    self.numeric.next();
                    suff.push(c);
                    Unkn(seen, suff)
                }
                (Unkn(seen, suff), _) => {
                    eprintln!("{}:{}:{} - warn - unprocessable numeric literal: numeric:[{}] suffix:[{}] ", self.location.f(), self.location.l(), self.location.c(), seen, suff);
                    let combined = seen + &suff;
                    break Token::Unknown(combined)
                }
                (s, c) => {
                    panic!("{}:{}:{} - FATAL - Unhandled inputs: ({:?}, {:?})", self.location.f(), self.location.l(), self.location.c(), s, c);
                }
            };
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