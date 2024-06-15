use crate::{hex_escape::HexEsc, oct_escape::OctEsc, text::TextState, universal_char::UnivEsc, LocationState};


pub trait CharEsc<'input> {
    fn consume_char_escape(&self);
}

pub struct CharEscImpl<'input, C> {
    location: &'input dyn LocationState<'input>,
    text: &'input dyn TextState<'input, Ch = C>,
    hex: &'input dyn HexEsc<'input>,
    oct: &'input dyn OctEsc<'input>,
    univ: &'input dyn UnivEsc<'input>,
}

impl<'input, C: 'input> CharEscImpl<'input, C> {
    fn new(
        location: &'input dyn LocationState<'input>,
        text: &'input dyn TextState<'input, Ch = C>,
        hex: &'input dyn HexEsc<'input>,
        oct: &'input dyn OctEsc<'input>,
        univ: &'input dyn UnivEsc<'input>,
    ) -> CharEscImpl<'input, C> {
        CharEscImpl{location, text, hex, oct, univ}
    }
}

impl<'input, C> CharEsc<'input> for CharEscImpl<'input, C> {
    fn consume_char_escape(&self) {
        macro_rules! emit_escape {
            ($o: literal) => {
                {
                    self.text.next();
                    self.text.push_u8($o);
                }
            }
        }


        match self.text.peek() {
            Some('0' ..= '7') => self.oct.consume_oct_escape(),
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
            _ => eprintln!("{}:{}:{} - warn - unknown escape in char literal", self.location.f(), self.location.l(), self.location.c()),
        }
    }
}

#[cfg(test)]
mod test {

    use super::CharEsc;
    use super::CharEscImpl;

    use crate::text::TextState;
    use crate::LocationState;
    use crate::text::I8Text;
    use crate::hex_escape::HexEscImpl;
    use crate::oct_escape::OctEscImpl;
    use crate::universal_char::UnivEscImpl;

    struct Dummy{}

    impl Dummy {
        pub fn new() -> Self{
            Dummy{}
        }
    }
    impl<'input> LocationState<'input> for Dummy {
        fn f(&self) -> &'input str {
            return "DUMMY"
        }

        fn l(&self) -> u32 {
            return 1;
        }

        fn c(&self) -> usize {
            return 1;
        }
    }

    #[test]
    fn do_char_escape() {

        let input: &str = "U000000a0";
        let chars = input.chars();
        let iter = chars.peekable();
        let l = Dummy::new();
        let t = I8Text::new(iter);
        let h = HexEscImpl::new(&l, &t);
        let o = OctEscImpl::new(&l, &t);
        let u = UnivEscImpl::new(&l, &t);

        let cei: CharEscImpl<i8> = CharEscImpl{location: &l, text: &t, hex: &h, oct: &o, univ: &u};

        cei.consume_char_escape();

        print!("t.get_output(): {:?}", t.get_output())
    }
}
