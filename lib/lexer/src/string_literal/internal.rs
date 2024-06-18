#[cfg(test)]
mod tests;

use crate::char_literal::CharLiteral;
use crate::text::char_escape;
use crate::text::char_escape::CharEsc;
use crate::text::TextState;
use crate::LocationState;
use crate::Token;

pub trait StringLiteral<C> {
    fn consume_string_literal(&self) -> Token;
}

pub fn string_literal_impl<'iter, C>(
    location: &'iter dyn LocationState,
    text: &'iter dyn TextState<Ch = C>,
    char_escape: &'iter dyn CharEsc,
) -> Box<dyn StringLiteral<C> + 'iter> {
    Box::new(StringLiteralImpl::new(location, text, char_escape))
}

struct StringLiteralImpl<'iter, C> {
    location: &'iter dyn LocationState,
    text: &'iter dyn TextState<Ch = C>,
    char_escape: &'iter dyn CharEsc,
}

impl<'iter, C: 'iter> StringLiteralImpl<'iter, C> {
    fn new(
        location: &'iter dyn LocationState,
        text: &'iter dyn TextState<Ch = C>,
        char_escape: &'iter dyn CharEsc,
    ) -> StringLiteralImpl<'iter, C> {
        StringLiteralImpl {
            location,
            text,
            char_escape,
        }
    }
}

impl<'iter, C> StringLiteral<C> for StringLiteralImpl<'iter, C> {
    fn consume_string_literal(&self) -> Token {
        match self.text.peek() {
            Some('\"') => {
                self.text.next();
            }
            _ => panic!(
                "{}:{}:{} - FATAL - this isn't a string literal",
                self.location.f(),
                self.location.l(),
                self.location.c(),
            ),
        }
        loop {
            match self.text.peek() {
                Some('\n') | None => {
                    eprintln!(
                        "{}:{}:{} - error - unterminated string literal",
                        self.location.f(),
                        self.location.l(),
                        self.location.c()
                    );
                    break self.text.emit_unknown()
                }
                _ => (),
            }
            match self.text.next() {
                Some('\n') | None => panic!(
                    "{}:{}:{} - FATAL - We should have handled this in the match block above",
                    self.location.f(),
                    self.location.l(),
                    self.location.c()
                ),
                Some('\"') => {
                    if self.text.seen_error() {
                        eprintln!("{}:{}:{} - warn - seen error in processing, returning Unknown token not StringLit", self.location.f(), self.location.l(), self.location.c());
                        break self.text.emit_unknown();
                    }
                    break self.text.emit_string_lit();
                }
                Some('\\') => self.char_escape.consume_char_escape(),
                Some(c) => self.text.push_char(c),
            }
        }
    }
}
