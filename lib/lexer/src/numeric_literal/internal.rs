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
}

impl<'iter> NumericLiteral for NumericLiteralImpl<'iter>{
    fn consume_numeric_literal(&self) -> Token {
        todo!()
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