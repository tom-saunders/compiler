use crate::{text::TextState, LocationState};

pub trait OctEsc {
    fn consume_oct_escape(&self);
}

pub fn oct_esc_impl<'iter, C>(
    location: &'iter dyn LocationState,
    text: &'iter dyn TextState<Ch = C>,
) -> Box<dyn OctEsc + 'iter> {
    Box::new(OctEscImpl::new(location, text))
}

struct OctEscImpl<'iter, C> {
    location: &'iter dyn LocationState,
    text: &'iter dyn TextState<Ch = C>,
}

impl<'iter, C: 'iter> OctEscImpl<'iter, C> {
    fn new(
        location: &'iter dyn LocationState,
        text: &'iter dyn TextState<Ch = C>,
    ) -> OctEscImpl<'iter, C> {
        OctEscImpl{location, text}
    }
}

impl<'iter, C> OctEsc for OctEscImpl<'iter, C> {
    fn consume_oct_escape(& self) {
        let mut num_octs = 0;
        let mut octs = String::new();

        while num_octs < 3 && match self.text.peek() {
            Some('0' ..= '7') => true,
            _ => false,
        } {
            let c = self.text.next().expect("We just peeked to check");
            octs.push(c);
            num_octs += 1;
        }
        if num_octs == 0 {
            panic!("{}:{}:{} - FATAL - this isn't an oct escape", self.location.f(), self.location.l(), self.location.c());
        }

        self.text.push_oct_value(self.location, octs)
    }
}
