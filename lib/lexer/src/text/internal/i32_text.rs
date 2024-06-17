use std::{cell::RefCell, iter::Peekable, str::Chars};

use crate::{LocationState, Token};

use super::TextState;

pub fn get_implementation<'iter>(
    iter: Peekable<Chars<'iter>>,
) -> Box<dyn TextState<Ch = i32> + 'iter> {
    Box::new(I32Text::new(iter))
}

struct I32Text<'iter> {
    iter: RefCell<Peekable<Chars<'iter>>>,
    consumed: RefCell<String>,
    seen_error: RefCell<bool>,
    output: RefCell<Vec<i32>>,
}

impl<'iter> I32Text<'iter> {
    fn new(iter: Peekable<Chars<'iter>>) -> I32Text<'iter> {
        I32Text {
            iter: RefCell::new(iter),
            consumed: RefCell::new(String::new()),
            seen_error: RefCell::new(false),
            output: RefCell::new(vec![]),
        }
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
                eprintln!(
                    "{}:{}:{} - error - empty char literal",
                    location.f(),
                    location.l(),
                    location.c()
                );
                self.emit_unknown()
            }
            1 => Token::CharLit_U(self.output.borrow()[0]),
            _ => {
                eprintln!(
                    "{}:{}:{} - warn - multi-char char literal",
                    location.f(),
                    location.l(),
                    location.c()
                );
                let v = *self
                    .output
                    .borrow()
                    .last()
                    .expect("We just checked there is at least one value in the vector");
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
        let u32val = u32::from_str_radix(&octs, 8)
            .expect("We are looking at 1 ..= 3 ot digits, should parse to a u32");
        self.push_c(u32val as i32)
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
            1..=8 => {
                let u32val = u32::from_str_radix(&hexs, 16)
                    .expect("We only matched 1 ..= 8 hex chars, this should parse to a u32");
                self.push_c(u32val as i32)
            }
            n => {
                eprintln!(
                    "{}:{}:{} - warn - hex escape sequence out of range",
                    location.f(),
                    location.l(),
                    location.c()
                );
                let u32val = u32::from_str_radix(&hexs[n - 8..], 16)
                    .expect("We're only looking at 4 hex chars, this should parse to a u16");
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

    fn chars_consumed(&self) -> usize {
        self.consumed.borrow().chars().count()
    }
}
