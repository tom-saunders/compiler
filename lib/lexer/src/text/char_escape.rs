use crate::{
    text::hex_escape::HexEsc, text::oct_escape::OctEsc, text::TextState, text::universal_char::UnivEsc, LocationState,
};

pub trait CharEsc {
    fn consume_char_escape(&self);
}

pub fn char_esc_impl<'iter, C>(
    location: &'iter dyn LocationState,
    text: &'iter dyn TextState<Ch = C>,
    hex: &'iter dyn HexEsc,
    oct: &'iter dyn OctEsc,
    univ: &'iter dyn UnivEsc,
) -> Box<dyn CharEsc + 'iter> {
    Box::new(CharEscImpl::new(location, text, hex, oct, univ))
}

struct CharEscImpl<'iter, C> {
    location: &'iter dyn LocationState,
    text: &'iter dyn TextState<Ch = C>,
    hex: &'iter dyn HexEsc,
    oct: &'iter dyn OctEsc,
    univ: &'iter dyn UnivEsc,
}

impl<'iter, C: 'iter> CharEscImpl<'iter, C> {
    fn new(
        location: &'iter dyn LocationState,
        text: &'iter dyn TextState<Ch = C>,
        hex: &'iter dyn HexEsc,
        oct: &'iter dyn OctEsc,
        univ: &'iter dyn UnivEsc,
    ) -> CharEscImpl<'iter, C> {
        CharEscImpl {
            location,
            text,
            hex,
            oct,
            univ,
        }
    }
}

impl<'iter, C> CharEsc for CharEscImpl<'iter, C> {
    fn consume_char_escape(&self) {
        macro_rules! emit_escape {
            ($o: literal) => {{
                self.text.next();
                self.text.push_u8($o);
            }};
        }

        match self.text.peek() {
            Some('0'..='7') => self.oct.consume_oct_escape(),
            Some('x') => self.hex.consume_hex_escape(),
            Some('u') => self.univ.consume_universal_short(),
            Some('U') => self.univ.consume_universal_long(),
            Some('\'') => emit_escape!(b'\''),
            Some('\\') => emit_escape!(b'\\'),
            Some('\"') => emit_escape!(b'\"'),
            Some('?') => emit_escape!(b'?'),
            Some('a') => emit_escape!(0x07),
            Some('b') => emit_escape!(0x08),
            Some('f') => emit_escape!(0x0c),
            Some('n') => emit_escape!(b'\n'),
            Some('r') => emit_escape!(b'\r'),
            Some('t') => emit_escape!(b'\t'),
            Some('v') => emit_escape!(0x0b),
            None => (),
            _ => eprintln!(
                "{}:{}:{} - warn - unknown escape in char literal",
                self.location.f(),
                self.location.l(),
                self.location.c()
            ),
        }
    }
}
