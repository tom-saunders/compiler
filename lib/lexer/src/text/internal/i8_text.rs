use std::{cell::RefCell, iter::Peekable, str::Chars};

use crate::{LocationState, Token};

use super::TextState;

pub fn get_implementation<'iter>(
    iter: Peekable<Chars<'iter>>,
) -> Box<dyn TextState<Ch = i8> + 'iter> {
    Box::new(I8Text::new(iter))
}

struct I8Text<'iter> {
    iter: RefCell<Peekable<Chars<'iter>>>,
    consumed: RefCell<String>,
    seen_error: RefCell<bool>,
    output: RefCell<Vec<i8>>,
}

impl<'iter> I8Text<'iter> {
    fn new(iter: Peekable<Chars<'iter>>) -> I8Text<'iter> {
        I8Text {
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
                eprintln!(
                    "{}:{}:{} - error - empty char literal",
                    location.f(),
                    location.l(),
                    location.c()
                );
                self.emit_unknown()
            }
            1 => Token::CharLit(self.output.borrow()[0] as i32),
            _ => {
                eprintln!(
                    "{}:{}:{} - warn - multi-char char literal",
                    location.f(),
                    location.l(),
                    location.c()
                );
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
                eprintln!(
                    "{}:{}:{} - warn - octal escape sequence out of range: {e}",
                    location.f(),
                    location.l(),
                    location.c()
                );
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
            }
        }
    }

    fn push_hex_value(&self, location: &dyn LocationState, hexs: String) {
        if !hexs.is_ascii() {
            panic!(
                "{}:{}:{} - FATAL - hex digit string must be ascii",
                location.f(),
                location.l(),
                location.c()
            );
        }
        match hexs.len() {
            0 => {
                eprintln!(
                    "{}:{}:{} - error - hex escape with no following hex digits",
                    location.f(),
                    location.l(),
                    location.c()
                );
                self.report_error()
            }
            1 | 2 => {
                let uval = u8::from_str_radix(&hexs, 16)
                    .expect("We only matched 1 or 2 hex chars, this should parse to a u8");
                self.push_u8(uval)
            }
            n => {
                eprintln!(
                    "{}:{}:{} - warn - hex escape sequence out of range",
                    location.f(),
                    location.l(),
                    location.c()
                );
                let uval = u8::from_str_radix(&hexs[n - 2..], 16)
                    .expect("We're only looking at 2 hex chars, this should parse to a u8");
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

    fn chars_consumed(&self) -> usize {
        self.consumed.borrow().chars().count()
    }
}
