use crate::{text::TextState, LocationState};

pub trait HexEsc {
    fn consume_hex_escape(&self);
}

pub fn hex_esc_impl<'iter, C> (
    location: &'iter dyn LocationState,
    text: &'iter dyn TextState<Ch = C>,
) -> Box<dyn HexEsc + 'iter> {
    Box::new(HexEscImpl::new(location, text))
}

struct HexEscImpl<'iter, C> {
    location: &'iter dyn LocationState,
    text: &'iter dyn TextState<Ch = C>,
}

impl<'iter, C: 'iter> HexEscImpl<'iter, C> {
    fn new(
            location: &'iter dyn LocationState,
            text: &'iter dyn TextState<Ch = C>,
    ) -> HexEscImpl<'iter, C> {
        HexEscImpl{location, text}
    }
}

impl<'iter, C> HexEsc for HexEscImpl<'iter, C> {
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
