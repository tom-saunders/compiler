#[cfg(test)]
mod tests;

use crate::text::char_escape::CharEsc;
use crate::text::TextState;
use crate::LocationState;
use crate::Token;

pub trait CharLiteral<C> {
    fn consume_char_literal(&self) -> Token;
}

pub fn char_literal_impl<'iter, C>(
    location: &'iter dyn LocationState,
    text: &'iter dyn TextState<Ch = C>,
    char_escape: &'iter dyn CharEsc,
) -> Box<dyn CharLiteral<C> + 'iter> {
    Box::new(CharLiteralImpl::new(location, text, char_escape))
}

struct CharLiteralImpl<'iter, C> {
    location: &'iter dyn LocationState,
    text: &'iter dyn TextState<Ch = C>,
    char_escape: &'iter dyn CharEsc,
}

impl<'iter, C: 'iter> CharLiteralImpl<'iter, C> {
    fn new(
        location: &'iter dyn LocationState,
        text: &'iter dyn TextState<Ch = C>,
        char_escape: &'iter dyn CharEsc,
    ) -> CharLiteralImpl<'iter, C> {
        CharLiteralImpl {
            location,
            text,
            char_escape,
        }
    }
}

impl<'input, C> CharLiteral<C> for CharLiteralImpl<'input, C> {
    fn consume_char_literal(&self) -> Token {
        match self.text.peek() {
            Some('\'') => {
                self.text.next();
            }
            _ => panic!(
                "{}:{}:{} - FATAL - this isn't a char literal",
                self.location.f(),
                self.location.l(),
                self.location.c()
            ),
        }

        loop {
            match self.text.peek() {
                Some('\n') | None => {
                    eprintln!(
                        "{}:{}:{} - error - unterminated char literal",
                        self.location.f(),
                        self.location.l(),
                        self.location.c()
                    );
                    break self.text.emit_unknown();
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
                Some('\'') => {
                    if self.text.seen_error() {
                        eprintln!("{}:{}:{} - warn - seen error in processing, returning Unknown token not CharLit", self.location.f(), self.location.l(), self.location.c());
                        break self.text.emit_unknown();
                    }
                    break self.text.emit_char_lit(self.location);
                }
                Some('\\') => self.char_escape.consume_char_escape(),
                Some(c) => self.text.push_char(c),
            }
        }
    }
}
