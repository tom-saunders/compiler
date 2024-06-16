use crate::char_escape::CharEsc;
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
        CharLiteralImpl{location, text, char_escape}
    }
}

impl<'input, C> CharLiteral<C> for CharLiteralImpl<'input, C> {
    fn consume_char_literal(&self) -> Token {
        match self.text.peek() {
            Some('\'') => {
                self.text.next();
            }
            _ => panic!("{}:{}:{} - FATAL - this isn't a char literal", self.location.f(), self.location.l(), self.location.c()),
        }

        loop {
            match self.text.peek() {
                Some('\n') | None => {
                    eprintln!("{}:{}:{} - error - unterminated char literal", self.location.f(), self.location.l(), self.location.c());
                    break self.text.emit_unknown()
                },
                _ => (),
            }
            match self.text.next() {
                Some('\n') | None => panic!("{}:{}:{} - FATAL - We should have handled this in the match block above", self.location.f(), self.location.l(), self.location.c()),
                Some('\'') => {
                    if self.text.seen_error() {
                        eprintln!("{}:{}:{} - warn - seen error in processing, returning Unknown token not CharLit", self.location.f(), self.location.l(), self.location.c());
                        break self.text.emit_unknown()
                    }
                    break self.text.emit_char_lit(self.location)
                },
                Some('\\') => self.char_escape.consume_char_escape(),
                Some(c) => self.text.push_char(c),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Borrow;

    use crate::char_escape::char_esc_impl;
    use crate::hex_escape::hex_esc_impl;
    use crate::oct_escape::oct_esc_impl;
    use crate::text::text_state_impl_i16;
    use crate::text::text_state_impl_i32;
    use crate::text::text_state_impl_i8;
    use crate::universal_char::univ_esc_impl;
    use crate::Token;
    use crate::Token::CharLit;
    use crate::Token::CharLit_u;
    use crate::Token::CharLit_U;
    use crate::LocationState;
    use crate::text::{self, TextState};
    use crate::char_escape::{self, CharEsc};
    use crate::hex_escape::{self, HexEsc};
    use crate::oct_escape::{self, OctEsc};
    use crate::universal_char::{self, UnivEsc};

    use super::char_literal_impl;
    use super::CharLiteral;

    struct TestLocation;
    impl LocationState for TestLocation {
        fn f(&self) -> &str {
            "TEST"
        }

        fn l(&self) -> u32 {
            1
        }

        fn c(&self) -> usize {
            1
        }
    }

    fn actual_i8(input: &str) -> Token {
        let location = Box::new(TestLocation);
        let text = text_state_impl_i8(input.chars().peekable());

        let hex_escape = hex_esc_impl::<i8>(location.as_ref(), text.as_ref());
        let oct_escape = oct_esc_impl::<i8>(location.as_ref(), text.as_ref());
        let univ_escape = univ_esc_impl::<i8>(location.as_ref(), text.as_ref());

        let char_escape = char_esc_impl(location.as_ref(), text.as_ref(), hex_escape.as_ref(), oct_escape.as_ref(), univ_escape.as_ref());

        let char_lit = char_literal_impl(location.as_ref(), text.as_ref(), char_escape.as_ref());
        char_lit.consume_char_literal()
    }

    fn actual_i16(input: &str) -> Token {
        let location = Box::new(TestLocation);
        let text = text_state_impl_i16(input.chars().peekable());

        let hex_escape = hex_esc_impl::<i16>(location.as_ref(), text.as_ref());
        let oct_escape = oct_esc_impl::<i16>(location.as_ref(), text.as_ref());
        let univ_escape = univ_esc_impl::<i16>(location.as_ref(), text.as_ref());

        let char_escape = char_esc_impl(location.as_ref(), text.as_ref(), hex_escape.as_ref(), oct_escape.as_ref(), univ_escape.as_ref());

        let char_lit = char_literal_impl(location.as_ref(), text.as_ref(), char_escape.as_ref());
        char_lit.consume_char_literal()
    }

    fn actual_i32(input: &str) -> Token {
        let location = Box::new(TestLocation);
        let text = text_state_impl_i32(input.chars().peekable());

        let hex_escape = hex_esc_impl::<i32>(location.as_ref(), text.as_ref());
        let oct_escape = oct_esc_impl::<i32>(location.as_ref(), text.as_ref());
        let univ_escape = univ_esc_impl::<i32>(location.as_ref(), text.as_ref());

        let char_escape = char_esc_impl(location.as_ref(), text.as_ref(), hex_escape.as_ref(), oct_escape.as_ref(), univ_escape.as_ref());

        let char_lit = char_literal_impl(location.as_ref(), text.as_ref(), char_escape.as_ref());
        char_lit.consume_char_literal()
    }

    fn unknown_and_actual_i8(input: &str) -> (Token, Token) {
        let expected = Token::Unknown(input.to_string());
        let actual = actual_i8(input);

        (expected, actual)
    }

    fn unknown_and_actual_i16(input: &str) -> (Token, Token) {
        let expected = Token::Unknown(input.to_string());
        let actual = actual_i16(input);

        (expected, actual)
    }

    fn unknown_and_actual_i32(input: &str) -> (Token, Token) {
        let expected = Token::Unknown(input.to_string());
        let actual = actual_i32(input);

        (expected, actual)
    }

    #[test]
    fn test_i8_char_literal_empty() {
        let (expected, actual) = unknown_and_actual_i8("''");

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_i16_char_literal_empty() {
        let (expected, actual) = unknown_and_actual_i16("''");

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_i32_char_literal_empty() {
        let (expected, actual) = unknown_and_actual_i32("''");

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_i8_char_literals_unterminated_eoi() {
        let (expected, actual) = unknown_and_actual_i8("'");

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_i16_char_literals_unterminated_eoi() {
        let (expected, actual) = unknown_and_actual_i16("'");

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_i32_char_literals_unterminated_eoi() {
        let (expected, actual) = unknown_and_actual_i32("'");

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_i8_char_literals_unterminated_eol() {
        let expected = Token::Unknown("'".to_string());
        let actual = actual_i8("'\n");

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_i16_char_literals_unterminated_eol() {
        let expected = Token::Unknown("'".to_string());
        let actual = actual_i16("'\n");

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_i32_char_literals_unterminated_eol() {
        let expected = Token::Unknown("'".to_string());
        let actual = actual_i32("'\n");

        assert_eq!(expected, actual)
    }
}

#[cfg(none)]
mod oldtest {
    macro_rules! state_and_unk {
        ($input: literal) => {
            (CharState{input: $input, file_name: "", file_line: 1, column: 1}, Unknown($input.to_string()), $input.len())
        };
    }

    macro_rules! state_and_exp {
        ($input: literal, $exp: expr) => {
            (CharState{input: $input, file_name: "", file_line: 1, column: 1}, CharLit($exp as i32), $input.len())
        };
    }

    #[test]
    fn test_char_literals_unterminated() {
        let (state, exp_t, exp_n) = state_and_unk!("'");

        let (t, n) = consume_char_literal_inner(&state)
            .expect("expect Unknown token");

        assert_eq!(exp_t, t);
        assert_eq!(exp_n, n);
    }

    #[test]
    fn test_char_literals_empty() {
        let (state, exp_t, exp_n) = state_and_unk!("''");

        let (t, n) = consume_char_literal_inner(&state)
            .expect("expect Unknown token");

        assert_eq!(exp_t, t);
        assert_eq!(exp_n, n);
    }

    #[test]
    fn test_char_literals_one_char() {
        let (state, exp_t, exp_n) = state_and_exp!("'a'", 'a');

        let (t, n) = consume_char_literal_inner(&state)
            .expect("expect CharLit token");

        assert_eq!(exp_t, t);
        assert_eq!(exp_n, n);
    }

    #[test]
    fn test_char_literals_two_char_not_escape() {
        let (state, exp_t, exp_n) = state_and_exp!("'ab'", (('a' as i32) << 8) + 'b' as i32);

        let (t, n) = consume_char_literal_inner(&state)
            .expect("expect CharLit token");

        assert_eq!(exp_t, t);
        assert_eq!(exp_n, n);
    }

    #[test]
    fn test_char_literals_two_char_valid_escapes() {
        let test_data = vec![
            state_and_exp!(r"'\0'", 0x00),
            state_and_exp!(r"'\1'", 0x01),
            state_and_exp!(r"'\2'", 0x02),
            state_and_exp!(r"'\3'", 0x03),
            state_and_exp!(r"'\4'", 0x04),
            state_and_exp!(r"'\5'", 0x05),
            state_and_exp!(r"'\6'", 0x06),
            state_and_exp!(r"'\7'", 0x07),

            state_and_exp!(r"'\''", b'\''),
            state_and_exp!(r#"'\"'"#, b'\"'),
            state_and_exp!(r"'\?'", b'?'),
            state_and_exp!(r"'\\'", b'\\'),

            state_and_exp!(r"'\a'", 0x07),
            state_and_exp!(r"'\b'", 0x08),
            state_and_exp!(r"'\f'", 0x0c),
            state_and_exp!(r"'\n'", b'\n'),
            state_and_exp!(r"'\r'", b'\r'),
            state_and_exp!(r"'\t'", b'\t'),
            state_and_exp!(r"'\v'", 0x0b),
        ];

        let mut exp_t_n: Vec<(Token, usize)> = vec![];
        let mut act_t_n: Vec<(Token, usize)> = vec![];
        for (s, t, n) in test_data {
            exp_t_n.push((t, n));
            act_t_n.push(consume_char_literal_inner(&s).expect("All of the inputs should lex"));
        }

        assert_eq!(exp_t_n, act_t_n);
    }


    #[test]
    fn test_char_literals_two_char_invalid_escapes() {
        let test_data = vec![
            state_and_exp!(r"'\8'", '8'),
            state_and_exp!(r"'\c'", 'c'),
            state_and_exp!(r"'\s'", 's'),
        ];

        let mut exp_t_n: Vec<(Token, usize)> = vec![];
        let mut act_t_n: Vec<(Token, usize)> = vec![];
        for (s, t, n) in test_data {
            exp_t_n.push((t, n));
            act_t_n.push(consume_char_literal_inner(&s).expect("All of the inputs should lex"));
        }

        assert_eq!(exp_t_n, act_t_n);
    }


    #[test]
    fn test_char_literals_two_char_hex_escape_with_no_value() {
        let test_data = vec![
            state_and_unk!(r"'\xM'"),
        ];

        let mut exp_t_n: Vec<(Token, usize)> = vec![];
        let mut act_t_n: Vec<(Token, usize)> = vec![];
        for (s, t, n) in test_data {
            exp_t_n.push((t, n));
            act_t_n.push(consume_char_literal_inner(&s).expect("All of the inputs should lex"));
        }

        assert_eq!(exp_t_n, act_t_n);
    }

    #[test]
    fn test_char_literals_three_char_valid_escapes() {
        let test_data = vec![
            state_and_exp!(r"'\00'", 0x00),
            state_and_exp!(r"'\61'", '1'),
            state_and_exp!(r"'\x0'", 0x00),
            state_and_exp!(r"'\xa'", 0x0a),
            state_and_exp!(r"'\xF'", 0x0f),
        ];

        let mut exp_t_n: Vec<(Token, usize)> = vec![];
        let mut act_t_n: Vec<(Token, usize)> = vec![];
        for (s, t, n) in test_data {
            exp_t_n.push((t, n));
            act_t_n.push(consume_char_literal_inner(&s).expect("All of the inputs should lex"));
        }

        assert_eq!(exp_t_n, act_t_n);
    }

    #[test]
    fn test_char_literals_three_char_not_escape() {
        let (state, exp_t, exp_n) = state_and_exp!("'abc'", (('a' as i32) << 16) + (('b' as i32) << 8) + 'c' as i32);

        let (t, n) = consume_char_literal_inner(&state)
            .expect("expect CharLit token");

        assert_eq!(exp_t, t);
        assert_eq!(exp_n, n)
    }

    #[test]
    fn test_char_literals_three_char_misleading_escapes() {
        let test_data = vec![
            // this is handled as [ ' \1 8 ' ]
            state_and_exp!(r"'\18'", 0x0138),
            // this is handled as [ ' \a b ' ]
            state_and_exp!(r"'\ab'", 0x0762),
            // this is handled as [ ' \xa h ' ]
        ];

        let mut exp_t_n: Vec<(Token, usize)> = vec![];
        let mut act_t_n: Vec<(Token, usize)> = vec![];

        for (s, t, n) in test_data {
            exp_t_n.push((t, n));
            act_t_n.push(consume_char_literal_inner(&s).expect("All of the inputs should lex"));
        }

        assert_eq!(exp_t_n, act_t_n);
    }

    #[test]
    fn test_char_literals_four_char_valid_escapes() {
        let test_data = vec![
            state_and_exp!(r"'\000'", 0x00),
            state_and_exp!(r"'\061'", '1'),
            state_and_exp!(r"'\x00'", 0x00),
            state_and_exp!(r"'\x0a'", 0x0a),
            state_and_exp!(r"'\x0F'", 0x0f),
            state_and_exp!(r"'\x70'", 0x70),
            state_and_exp!(r"'\xa0'", 0xffffffa0u32 as i32),
            state_and_exp!(r"'\xF0'", 0xfffffff0u32 as i32),
        ];

        let mut exp_t_n: Vec<(Token, usize)> = vec![];
        let mut act_t_n: Vec<(Token, usize)> = vec![];
        for (s, t, n) in test_data {
            exp_t_n.push((t, n));
            act_t_n.push(consume_char_literal_inner(&s).expect("All of the inputs should lex"));
        }

        assert_eq!(exp_t_n, act_t_n);
    }

    #[test]
    fn test_char_literals_four_char_not_escape() {
        let (state, exp_t, exp_n) = state_and_exp!("'abcd'", (('a' as i32) << 24) + (('b' as i32) << 16) + (('c' as i32) << 8) + 'd' as i32);

        let (t, n) = consume_char_literal_inner(&state)
            .expect("expect CharLit token");

        assert_eq!(exp_t, t);
        assert_eq!(exp_n, n)
    }

    #[test]
    fn test_char_literals_four_char_misleading_escapes() {
        let test_data = vec![
            // this is handled as [ ' \11 8 ' ]
            state_and_exp!(r"'\118'", 0x0738),
            // this is handled as [ ' \a b c ' ]
            state_and_exp!(r"'\abc'", 0x076263),
            // this is handled as [ ' \xa h ' ]
            state_and_exp!(r"'\xah'", 0x0a68),
            // this isn't misleading per se, but it's possibly unexpected if you don't realize the value is handled as an i32
            // 0xff is -1i8 so this is sign extended to 0xffffffff (-1 as i32) not 0x000000ff (255 as i32)
            state_and_exp!(r"'\xff'", 0xffffffffu32),
        ];

        let mut exp_t_n: Vec<(Token, usize)> = vec![];
        let mut act_t_n: Vec<(Token, usize)> = vec![];

        for (s, t, n) in test_data {
            exp_t_n.push((t, n));
            act_t_n.push(consume_char_literal_inner(&s).expect("All of the inputs should lex"));
        }

        assert_eq!(exp_t_n, act_t_n);
    }

    #[test]
    fn test_char_literals_five_char_valid_escapes() {
        let test_data = vec![
            // this is handled as [ ' \000 0 ' ]
            state_and_exp!(r"'\0000'", 0x0030),
            // this is handled as [ ' \006 1 ' ]
            state_and_exp!(r"'\0061'", 0x0631),
            state_and_exp!(r"'\x000'", 0x00),
            state_and_exp!(r"'\x00a'", 0x0a),
            state_and_exp!(r"'\x00F'", 0x0f),
            state_and_exp!(r"'\x070'", 0x70),
        ];

        let mut exp_t_n: Vec<(Token, usize)> = vec![];
        let mut act_t_n: Vec<(Token, usize)> = vec![];
        for (s, t, n) in test_data {
            exp_t_n.push((t, n));
            act_t_n.push(consume_char_literal_inner(&s).expect("All of the inputs should lex"));
        }

        assert_eq!(exp_t_n, act_t_n);
    }

    #[test]
    fn test_char_literals_five_char_misleading_escapes() {
        let test_data = vec![
            // this is handled as [ ' \xa h h ' ]
            state_and_exp!(r"'\xahh'", 0x0a6868),
            // any hex sequence is truncated to the lowest 8 bits
            state_and_exp!(r"'\x100'", 0x00),
            state_and_exp!(r"'\x10a'", 0x0a),
            state_and_exp!(r"'\x10F'", 0x0f),
            state_and_exp!(r"'\x170'", 0x70),
            state_and_exp!(r"'\x1a0'", 0xffffffa0u32 as i32),
            state_and_exp!(r"'\x1F0'", 0xfffffff0u32 as i32),
            // this is possibly unexpected based on the behaviour when only two hex digits are provided
            state_and_exp!(r"'\7\xf'", 0x0000070f),
        ];

        let mut exp_t_n: Vec<(Token, usize)> = vec![];
        let mut act_t_n: Vec<(Token, usize)> = vec![];

        for (s, t, n) in test_data {
            exp_t_n.push((t, n));
            act_t_n.push(consume_char_literal_inner(&s).expect("All of the inputs should lex"));
        }

        assert_eq!(exp_t_n, act_t_n);
    }

    #[test]
    fn test_char_literals_five_char_not_escape() {
        // the first char overflows out of the i32 value space so we only end up with the final four
        let (state, exp_t, exp_n) = state_and_exp!("'abcde'", (('b' as i32) << 24) + (('c' as i32) << 16) + (('d' as i32) << 8) + 'e' as i32);

        let (t, n) = consume_char_literal_inner(&state)
            .expect("expect CharLit token");

        assert_eq!(exp_t, t);
        assert_eq!(exp_n, n)
    }

    #[test]
    fn test_char_literals_universal_short_valid() {
        let test_data = vec![
            state_and_unk!(r"'\u0000'"),
            state_and_unk!(r"'\u0023'"),
            state_and_exp!(r"'\u0024'", '$'),
            state_and_unk!(r"'\u0025'"),
            state_and_unk!(r"'\u003f'"),
            state_and_exp!(r"'\u0040'", '@'),
            state_and_unk!(r"'\u0041'"),
            state_and_unk!(r"'\u005f'"),
            state_and_exp!(r"'\u0060'", '`'),
            state_and_unk!(r"'\u0061'"),
            state_and_unk!(r"'\u009f'"),
            state_and_exp!(r"'\u00a0'", 0xc2a0),
            state_and_exp!(r"'\uabcd'", 0xeaaf8d),
            state_and_exp!(r"'\uABCD'", 0xeaaf8d),
            state_and_exp!(r"'\ud7ff'", 0xed9fbf),
            state_and_unk!(r"'\ud800'"),
            state_and_unk!(r"'\udfff'"),
            state_and_exp!(r"'\ue000'", 0xee8080),
            state_and_exp!(r"'\uef00'", 0xeebc80),
            state_and_exp!(r"'\uEF00'", 0xeebc80),
            state_and_exp!(r"'\uffff'", 0xefbfbf),
        ];

        let mut exp_t_n: Vec<(Token, usize)> = vec![];
        let mut act_t_n: Vec<(Token, usize)> = vec![];

        for (s, t, n) in test_data {
            exp_t_n.push((t, n));
            act_t_n.push(consume_char_literal_inner(&s).expect("All of the inputs should lex"));
        }

        assert_eq!(exp_t_n, act_t_n);
    }

    #[test]
    fn test_char_literals_universal_short_invalid() {
        let test_data = vec![
            state_and_unk!(r"'\u'"),
            // this would be invalid even if it were accepted as a shorter input
            state_and_unk!(r"'\u0'"),
            state_and_unk!(r"'\ua0'"),
            state_and_unk!(r"'\u0a0'"),
            state_and_exp!(r"'\u00a0'", 0xc2a0),
        ];

        let mut exp_t_n: Vec<(Token, usize)> = vec![];
        let mut act_t_n: Vec<(Token, usize)> = vec![];

        for (s, t, n) in test_data {
            exp_t_n.push((t, n));
            act_t_n.push(consume_char_literal_inner(&s).expect("All of the inputs should lex"));
        }

        assert_eq!(exp_t_n, act_t_n);
    }

    #[test]
    fn test_char_literals_universal_long_valid() {
        let test_data = vec![
            state_and_unk!(r"'\U00000000'"),
            state_and_unk!(r"'\U00000023'"),
            state_and_exp!(r"'\U00000024'", '$'),
            state_and_unk!(r"'\U00000025'"),
            state_and_unk!(r"'\U0000003f'"),
            state_and_exp!(r"'\U00000040'", '@'),
            state_and_unk!(r"'\U00000041'"),
            state_and_unk!(r"'\U0000005f'"),
            state_and_exp!(r"'\U00000060'", '`'),
            state_and_unk!(r"'\U00000061'"),
            state_and_unk!(r"'\U0000009f'"),
            state_and_exp!(r"'\U000000a0'", 0xc2a0),
            state_and_exp!(r"'\U0000abcd'", 0xeaaf8d),
            state_and_exp!(r"'\U0000ABCD'", 0xeaaf8d),
            state_and_exp!(r"'\U0000d7ff'", 0xed9fbf),
            state_and_unk!(r"'\U0000d800'"),
            state_and_unk!(r"'\U0000dfff'"),
            state_and_exp!(r"'\U0000ef00'", 0xeebc80),
            state_and_exp!(r"'\U0000EF00'", 0xeebc80),
            state_and_exp!(r"'\U0000e000'", 0xee8080),
            state_and_exp!(r"'\U0010FFFF'", 0xf48fbfbfu32),
            state_and_unk!(r"'\U00110000'"),
        ];

        let mut exp_t_n: Vec<(Token, usize)> = vec![];
        let mut act_t_n: Vec<(Token, usize)> = vec![];

        for (s, t, n) in test_data {
            exp_t_n.push((t, n));
            act_t_n.push(consume_char_literal_inner(&s).expect("All of the inputs should lex"));
        }

        assert_eq!(exp_t_n, act_t_n);
    }

    #[test]
    fn test_char_literals_universal_long_invalid() {
        let test_data = vec![
            state_and_unk!(r"'\U'"),
            // this would be invalid even if it were accepted as a shorter input
            state_and_unk!(r"'\U0'"),
            state_and_unk!(r"'\Ua0'"),
            state_and_unk!(r"'\U00a0'"),
            state_and_unk!(r"'\U000a0'"),
            state_and_unk!(r"'\U0000a0'"),
            state_and_unk!(r"'\U00000a0'"),
            state_and_exp!(r"'\U000000a0'", 0xc2a0),
        ];

        let mut exp_t_n: Vec<(Token, usize)> = vec![];
        let mut act_t_n: Vec<(Token, usize)> = vec![];

        for (s, t, n) in test_data {
            exp_t_n.push((t, n));
            act_t_n.push(consume_char_literal_inner(&s).expect("All of the inputs should lex"));
        }

        assert_eq!(exp_t_n, act_t_n);
    }
}