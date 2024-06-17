use std::{cell::RefCell, iter::Peekable, str::Chars};

use crate::{LocationState, Token};

use super::TextState;

pub fn get_implementation<'iter>(
    iter: Peekable<Chars<'iter>>,
) -> Box<dyn TextState<Ch = i16> + 'iter> {
    Box::new(I16Text::new(iter))
}

struct I16Text<'iter> {
    iter: RefCell<Peekable<Chars<'iter>>>,
    consumed: RefCell<String>,
    seen_error: RefCell<bool>,
    output: RefCell<Vec<i16>>,
}

impl<'iter> I16Text<'iter> {
    fn new(iter: Peekable<Chars<'iter>>) -> I16Text<'iter> {
        I16Text {
            iter: RefCell::new(iter),
            consumed: RefCell::new(String::new()),
            seen_error: RefCell::new(false),
            output: RefCell::new(vec![]),
        }
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
                eprintln!(
                    "{}:{}:{} - error - empty char literal",
                    location.f(),
                    location.l(),
                    location.c()
                );
                self.emit_unknown()
            }
            1 => {
                let v = self.output.borrow()[0];
                Token::CharLit_u((v as i32)  & 0x0000ffff)
            },
            _ => {
                eprintln!(
                    "{}:{}:{} - warn - multi-char char literal",
                    location.f(),
                    location.l(),
                    location.c()
                );
                let v: i16 = *self
                    .output
                    .borrow()
                    .last()
                    .expect("We just checked there is at least one value in the vector");
                Token::CharLit_u((v as i32) & 0x0000ffff)
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
        let u16val = u16::from_str_radix(&octs, 8)
            .expect("We are looking at 1 ..= 3 ot digits, should parse to a u16");
        self.push_c(u16val as i16)
    }

    fn push_hex_value(&self, location: &dyn LocationState, hexs: String) {
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
            1..=4 => {
                let u16val = u16::from_str_radix(&hexs, 16)
                    .expect("We only matched 1 ..= 4 hex chars, this should parse to a u16");
                self.push_c(u16val as i16)
            }
            n => {
                eprintln!(
                    "{}:{}:{} - warn - hex escape sequence out of range",
                    location.f(),
                    location.l(),
                    location.c()
                );
                let u16val = u16::from_str_radix(&hexs[n - 4..], 16)
                    .expect("We're only looking at 4 hex chars, this should parse to a u16");
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

    fn chars_consumed(&self) -> usize {
        self.consumed.borrow().chars().count()
    }
}