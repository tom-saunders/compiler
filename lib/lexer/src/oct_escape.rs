use crate::{text::TextState, LocationState};

pub trait OctEsc<'input> {
    fn consume_oct_escape(&self);
}

pub struct OctEscImpl<'input, C> {
    location: &'input dyn LocationState<'input>,
    text: &'input dyn TextState<'input, Ch = C>,
}

impl<'input, C: 'input> OctEscImpl<'input, C> {
    pub fn new(
        location: &'input dyn LocationState<'input>,
        text: &'input dyn TextState<'input, Ch = C>,
    ) -> OctEscImpl<'input, C> {
        OctEscImpl{location, text}
    }
}

impl<'input, C> OctEsc<'input> for OctEscImpl<'input, C> {
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
