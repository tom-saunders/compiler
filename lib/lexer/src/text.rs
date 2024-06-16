use std::{cell::RefCell, iter::Peekable, str::Chars};

use crate::{LocationState, Token};

pub trait TextState {
    type Ch;
    fn peek(&self) -> Option<char>;
    fn next(&self) -> Option<char>;

    fn emit_unknown(&self) -> Token;
    fn emit_char_lit(&self, location: &dyn LocationState) -> Token;
    fn emit_string_lit(&self) -> Token;

    fn push_char(&self, c: char);
    fn push_c(&self, c: Self::Ch);
    fn push_u8(&self, u: u8);

    fn push_oct_value(&self, location: &dyn LocationState, octs: String);
    fn push_hex_value(&self, location: &dyn LocationState, hexs: String);

    fn get_output(&self) -> Vec<Self::Ch>;

    fn report_error(&self);

    fn seen_error(&self) -> bool;
}

pub fn text_state_impl_i8<'iter>(
    iter: Peekable<Chars<'iter>>,
) -> Box<dyn TextState<Ch = i8> + 'iter>{
    Box::new(I8Text::new(iter))
}

pub fn text_state_impl_i16<'iter>(
    iter: Peekable<Chars<'iter>>,
) -> Box<dyn TextState<Ch = i16> + 'iter> {
    Box::new(I16Text::new(iter))
}

pub fn text_state_impl_i32<'iter>(
    iter: Peekable<Chars<'iter>>,
) -> Box<dyn TextState<Ch = i32> + 'iter> {
    Box::new(I32Text::new(iter))
}

struct I8Text<'iter> {
    iter: RefCell<Peekable<Chars<'iter>>>,
    consumed: RefCell<String>,
    seen_error: RefCell<bool>,
    output: RefCell<Vec<i8>>,
}

struct I16Text<'iter> {
    iter: RefCell<Peekable<Chars<'iter>>>,
    consumed: RefCell<String>,
    seen_error: RefCell<bool>,
    output: RefCell<Vec<i16>>,
}

struct I32Text<'iter> {
    iter: RefCell<Peekable<Chars<'iter>>>,
    consumed: RefCell<String>,
    seen_error: RefCell<bool>,
    output: RefCell<Vec<i32>>,
}

impl<'iter> I8Text<'iter> {
    fn new(iter: Peekable<Chars<'iter>>) -> I8Text<'iter> {
        I8Text{
            iter: RefCell::new(iter),
            consumed: RefCell::new(String::new()),
            seen_error: RefCell::new(false),
            output: RefCell::new(vec![]),
        }
    }
}

impl<'iter> I16Text<'iter> {
    fn new(iter: Peekable<Chars<'iter>>) -> I16Text<'iter> {
        I16Text{
            iter: RefCell::new(iter),
            consumed: RefCell::new(String::new()),
            seen_error: RefCell::new(false),
            output: RefCell::new(vec![]),
        }
    }
}

impl<'iter> I32Text<'iter> {
    fn new(iter: Peekable<Chars<'iter>>) -> I32Text<'iter> {
        I32Text{
            iter: RefCell::new(iter),
            consumed: RefCell::new(String::new()),
            seen_error: RefCell::new(false),
            output: RefCell::new(vec![]),
        }
    }
}

impl<'iter> TextState for I8Text<'iter> {
    type Ch = i8;

    fn peek(&self) -> Option<char> {
        match self.iter.borrow_mut().peek() {
            None => None,
            Some(c) => Some(*c),
        }
    }

    fn next(&self) -> Option<char> {
        let r = self.iter.borrow_mut().next();
        match r {
            Some(c) => self.consumed.borrow_mut().push(c),
            _ => (),
        }
        r
    }

    fn emit_unknown(&self) -> Token {
        Token::Unknown(self.consumed.borrow().clone())
    }

    fn emit_char_lit(&self, location: &dyn LocationState) -> Token {
        match self.output.borrow().len() {
            0 => {
                eprintln!("{}:{}:{} - error - empty char literal", location.f(), location.l(), location.c());
                self.emit_unknown()
            },
            1 => {
                Token::CharLit(self.output.borrow()[0] as i32)
            },
            _ => {
                eprintln!("{}:{}:{} - warn - multi-char char literal", location.f(), location.l(), location.c());
                let mut u: u32 = 0;
                for o in self.output.borrow().iter() {
                    (u, _) = u.overflowing_shl(8);
                    u |= 0x000000ff & (*o as u32);
                }
                let v = u as i32;
                Token::CharLit(v)
            }
        }
    }

    fn emit_string_lit(&self) -> Token {
        Token::StringLit(self.output.borrow().clone())
    }

    fn push_char(&self, c: char) {
        let mut bytes = [0; 4];
        c.encode_utf8(&mut bytes);
        for i in 0..c.len_utf8() {
            let v = bytes[i] as i8;
            self.output.borrow_mut().push(v)
        }
    }

    fn push_c(&self, c: i8) {
        self.output.borrow_mut().push(c)
    }

    fn push_u8(&self, u: u8) {
        self.output.borrow_mut().push(u as i8)
    }


    fn push_oct_value(&self, location: &dyn LocationState, mut octs: String) {
        match u8::from_str_radix(&octs, 8) {
            Ok(uval) => self.push_u8(uval),
            Err(e) => {
                eprintln!("{}:{}:{} - warn - octal escape sequence out of range: {e}", location.f(), location.l(), location.c());
                unsafe {
                    // we reduce the leading octal char by 4 which should bring the value into the i8 range
                    let oct_bytes = octs.as_bytes_mut();
                    oct_bytes[0] -= 4;
                };
                match u8::from_str_radix(&octs, 8) {
                    Ok(uval) => {
                        self.push_u8(uval)
                    },
                    Err(e) => panic!("{}:{}:{} - FATAL - We messed up and couldn't parse the new String oct either: {e}", location.f(), location.l(), location.c()),
                }
            },
        }
    }

    fn push_hex_value(&self, location: &dyn LocationState, hexs: String) {
        if ! hexs.is_ascii() {
            panic!("{}:{}:{} - FATAL - hex digit string must be ascii", location.f(), location.l(), location.c());
        }
        match hexs.len() {
            0 => {
                eprintln!("{}:{}:{} - error - hex escape with no following hex digits", location.f(), location.l(), location.c());
                self.report_error()
            },
            1 | 2 => {
                let uval = u8::from_str_radix(&hexs, 16).expect("We only matched 1 or 2 hex chars, this should parse to a u8");
                self.push_u8(uval)
            }
            n => {
                eprintln!("{}:{}:{} - warn - hex escape sequence out of range", location.f(), location.l(), location.c());
                let uval = u8::from_str_radix(&hexs[n-2..] ,16).expect("We're only looking at 2 hex chars, this should parse to a u8");
                self.push_u8(uval)
            }
        }
    }

    fn get_output(&self) -> Vec<Self::Ch> {
        self.output.borrow().clone()
    }

    fn report_error(&self) {
        *self.seen_error.borrow_mut() = true;
    }

    fn seen_error(&self) -> bool {
        *self.seen_error.borrow()
    }
}


impl<'iter> TextState for I16Text<'iter> {
    type Ch = i16;

    fn peek(&self) -> Option<char> {
        match self.iter.borrow_mut().peek() {
            None => None,
            Some(c) => Some(*c),
        }
    }

    fn next(&self) -> Option<char> {
        let r = self.iter.borrow_mut().next();
        match r {
            Some(c) => self.consumed.borrow_mut().push(c),
            _ => (),
        }
        r
    }

    fn emit_unknown(&self) -> Token {
        Token::Unknown(self.consumed.borrow().clone())
    }

    fn emit_char_lit(&self, location: &dyn LocationState) -> Token {
        match self.output.borrow().len() {
            0 => {
                eprintln!("{}:{}:{} - error - empty char literal", location.f(), location.l(), location.c());
                self.emit_unknown()
            },
            1 => {
                Token::CharLit_u(self.output.borrow()[0] as i32)
            },
            _ => {
                eprintln!("{}:{}:{} - warn - multi-char char literal", location.f(), location.l(), location.c());
                let mut u: u32 = 0;
                for o in self.output.borrow().iter() {
                    (u, _) = u.overflowing_shl(16);
                    u |= 0x0000ffff & (*o as u32);
                }
                let v = u as i32;
                Token::CharLit_u(v)
            }
        }
    }

    fn emit_string_lit(&self) -> Token {
        Token::StringLit_u(self.output.borrow().clone())
    }

    fn push_char(&self, c: char) {
        let mut surrogates = [0u16; 2];
        c.encode_utf16(&mut surrogates);
        for i in 0..c.len_utf16() {
            let v = surrogates[i] as i16;
            self.output.borrow_mut().push(v)
        }
    }

    fn push_c(&self, c: i16) {
        self.output.borrow_mut().push(c)
    }

    fn push_u8(&self, u: u8) {
        self.output.borrow_mut().push(u as i16)
    }

    fn push_oct_value(&self, _location: &dyn LocationState, octs: String) {
        let u16val = u16::from_str_radix(&octs, 8).expect("We are looking at 1 ..= 3 ot digits, should parse to a u16");
        self.push_c(u16val as i16)
    }

    fn push_hex_value(&self, location: &dyn LocationState, hexs: String) {
        match hexs.len() {
            0 => {
                eprintln!("{}:{}:{} - error - hex escape with no following hex digits", location.f(), location.l(), location.c());
                self.report_error()
            },
            1 ..= 4 => {
                let u16val = u16::from_str_radix(&hexs, 16).expect("We only matched 1 ..= 4 hex chars, this should parse to a u16");
                self.push_c(u16val as i16)
            }
            n => {
                eprintln!("{}:{}:{} - warn - hex escape sequence out of range", location.f(), location.l(), location.c());
                let u16val = u16::from_str_radix(&hexs[n-4..] ,16).expect("We're only looking at 4 hex chars, this should parse to a u16");
                self.push_c(u16val as i16)
            }
        }
    }

    fn get_output(&self) -> Vec<Self::Ch> {
        self.output.borrow().clone()
    }

    fn report_error(&self) {
        *self.seen_error.borrow_mut() = true;
    }

    fn seen_error(&self) -> bool {
        *self.seen_error.borrow()
    }
}

impl<'iter> TextState for I32Text<'iter> {
    type Ch = i32;

    fn peek(&self) -> Option<char> {
        match self.iter.borrow_mut().peek() {
            None => None,
            Some(c) => Some(*c),
        }
    }

    fn next(&self) -> Option<char> {
        let r = self.iter.borrow_mut().next();
        match r {
            Some(c) => self.consumed.borrow_mut().push(c),
            _ => (),
        }
        r
    }

    fn emit_unknown(&self) -> Token {
        Token::Unknown(self.consumed.borrow().clone())
    }

    fn emit_char_lit(&self, location: &dyn LocationState) -> Token {
        match self.output.borrow().len() {
            0 => {
                eprintln!("{}:{}:{} - error - empty char literal", location.f(), location.l(), location.c());
                self.emit_unknown()
            },
            1 => {
                Token::CharLit_U(self.output.borrow()[0])
            },
            _ => {
                eprintln!("{}:{}:{} - warn - multi-char char literal", location.f(), location.l(), location.c());
                let v = *self.output.borrow().last().expect("We just checked there is at least one value in the vector");
                Token::CharLit_U(v)
            }
        }
    }

    fn emit_string_lit(&self) -> Token {
        Token::StringLit_U(self.output.borrow().clone())
    }

    fn push_char(&self, c: char) {
        self.output.borrow_mut().push(c as i32)
    }

    fn push_c(&self, c: i32) {
        self.output.borrow_mut().push(c)
    }

    fn push_u8(&self, u: u8) {
        self.output.borrow_mut().push(u as i32)
    }

    fn push_oct_value(&self, _location: &dyn LocationState, octs: String) {
        let u32val = u32::from_str_radix(&octs, 8).expect("We are looking at 1 ..= 3 ot digits, should parse to a u32");
        self.push_c(u32val as i32)
    }

    fn push_hex_value(&self, location: &dyn LocationState, hexs: String) {
        match hexs.len() {
            0 => {
                eprintln!("{}:{}:{} - error - hex escape with no following hex digits", location.f(), location.l(), location.c());
                self.report_error()
            },
            1 ..= 8 => {
                let u32val = u32::from_str_radix(&hexs, 16).expect("We only matched 1 ..= 8 hex chars, this should parse to a u32");
                self.push_c(u32val as i32)
            }
            n => {
                eprintln!("{}:{}:{} - warn - hex escape sequence out of range", location.f(), location.l(), location.c());
                let u32val = u32::from_str_radix(&hexs[n-8..] ,16).expect("We're only looking at 4 hex chars, this should parse to a u16");
                self.push_c(u32val as i32)
            }
        }
    }

    fn get_output(&self) -> Vec<Self::Ch> {
        self.output.borrow().clone()
    }

    fn report_error(&self) {
        *self.seen_error.borrow_mut() = true;
    }

    fn seen_error(&self) -> bool {
        *self.seen_error.borrow()
    }
}