use crate::{text::TextState, LocationState};

pub trait HexEsc<'input> {
    fn consume_hex_escape(&self);
}

pub struct HexEscImpl<'input, C> {
    location: &'input dyn LocationState<'input>,
    text: &'input dyn TextState<'input, Ch = C>,
}

impl<'input, C: 'input> HexEscImpl<'input, C> {
    pub fn new(
            location: &'input dyn LocationState<'input>,
            text: &'input dyn TextState<'input, Ch = C>,
    ) -> HexEscImpl<'input, C> {
        HexEscImpl{location, text}
    }
}

impl<'input, C> HexEsc<'input> for HexEscImpl<'input, C> {
    fn consume_hex_escape(&self) {
        match self.text.peek() {
            Some('x') => self.text.next(),
            _ => panic!("{}:{}:{} - FATAL - this isn't a hex escape", self.location.f(), self.location.l(), self.location.c()),
        };

        let mut hexs = String::new();

        while match self.text.peek() {
            Some('0' ..= '9' | 'a' ..= 'f' | 'A' ..= 'F') => true,
            _ => false,
        } {
            let c = self.text.next().expect("We just peeked to check");
            hexs.push(c);
        }

        self.text.push_hex_value(self.location, hexs)
    }
}
