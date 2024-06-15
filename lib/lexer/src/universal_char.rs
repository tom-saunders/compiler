use crate::LocationState;
use crate::text::TextState;

pub trait UnivEsc<'input> {
    fn consume_universal_short(& self);
    fn consume_universal_long(& self);
    fn consume_universal_short_identifier(& self);
    fn consume_universal_long_identifier(& self);
}

pub struct UnivEscImpl<'input, C> {
    location: &'input dyn LocationState<'input>,
    text: &'input dyn TextState<'input, Ch = C>,
}

impl<'input, C: 'input> UnivEscImpl<'input, C> {
    pub fn new(
            location: &'input dyn LocationState<'input>,
            text: &'input dyn TextState<'input, Ch = C>,
    ) -> UnivEscImpl<'input, C> {
        UnivEscImpl{location, text}
    }

    fn no_restrict(_: u32) -> bool {
        true
    }

    fn identifier_restrict(u: u32) -> bool {
        match u {
            0x0300 ..= 0x036f => false,
            0x1dc0 ..= 0x1dff => false,
            0x20d0 ..= 0x20ff => false,
            0xfe20 ..= 0xfe2f => false,
            _ => true,
        }
    }

    fn consume_universal<F>(&self, restrict: F, esc_char: &str, exp_chars: usize)
    where F: Fn(u32) -> bool {

        let mut num_hex: usize = 0;
        let mut hexs = String::new();

        while num_hex < exp_chars && match self.text.peek() {
            Some('0' ..= '9' | 'a' ..= 'f' | 'A' ..= 'F') => true,
            _ => false,
        } {
            let c = self.text.next().expect("We just peeked to check");
            hexs.push(c);
            num_hex += 1;
        }

        if num_hex == exp_chars {
            let uval = u32::from_str_radix(&hexs, 16).expect("We've just scanned for exp_char hex chars of input");

            let meets_universal_constraints = match uval {
                0x24 | 0x40 | 0x60 => true,
                0 ..= 0x9f => false,
                0xd800 ..= 0xdfff => false,
                _ => true,
            };
            if ! meets_universal_constraints {
                eprintln!("{}:{}:{} - error - invalid universal character name \\{}{}", self.location.f(), self.location.l(), self.location.c(), esc_char, hexs);
                self.text.report_error();
                return
            }

            let meets_restrict = restrict(uval);
            if ! meets_restrict {
                eprintln!("{}:{}:{} - error - universal character name \\{}{} does not meet the restrictions applied", self.location.f(), self.location.l(), self.location.c(), esc_char, hexs);
                self.text.report_error();
                return
            }

            match char::from_u32(uval) {
                Some(c) => self.text.push_char(c),
                None => {
                    eprintln!("{}:{}:{} - error - universal character name \\{}{} does not map to a char", self.location.f(), self.location.l(), self.location.c(), esc_char, hexs);
                    self.text.report_error()
                },
            }
        } else {
            eprintln!("{}:{}:{} - error - incomplete universal character name \\{}{}", self.location.f(), self.location.l(), self.location.c(), esc_char, hexs);
            self.text.report_error()
        }
    }
}

impl<'input, C> UnivEsc<'input> for UnivEscImpl<'input, C> {
    fn consume_universal_short(&self) {
            match self.text.peek() {
                Some('u') => self.text.next(),
                _ => panic!("{}:{}:{} - FATAL - this isn't a short universal escape", self.location.f(), self.location.l(), self.location.c()),
            };
        self.consume_universal(Self::no_restrict, "u", 4)
    }

    fn consume_universal_long(&self) {
            match self.text.peek() {
                Some('U') => self.text.next(),
                _ => panic!("{}:{}:{} - FATAL - this isn't a long universal escape", self.location.f(), self.location.l(), self.location.c()),
            };
        self.consume_universal(Self::no_restrict, "U", 8)
    }

     fn consume_universal_short_identifier(&self) {
            match self.text.peek() {
                Some('u') => self.text.next(),
                _ => panic!("{}:{}:{} - FATAL - this isn't a short universal escape", self.location.f(), self.location.l(), self.location.c()),
            };
        self.consume_universal(Self::identifier_restrict, "u", 4)
    }

    fn consume_universal_long_identifier(&self) {
            match self.text.peek() {
                Some('U') => self.text.next(),
                _ => panic!("{}:{}:{} - FATAL - this isn't a long universal escape", self.location.f(), self.location.l(), self.location.c()),
            };
        self.consume_universal(Self::identifier_restrict, "U", 8)
    }
}