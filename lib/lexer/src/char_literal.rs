use std::iter::Peekable;
use std::str::Chars;

use crate::LexState;
use crate::Token;
use crate::Token::CharLit;
use crate::Token::Unknown;

struct CharState<'state>{
    input: &'state str,
    file_name: &'state str,
    file_line: u32,
    column: usize,
}

pub fn consume_char_literal<'state> (lstate: &'state LexState) -> Result<(Token, usize), ()> {

    let this_line = match lstate.input().find('\n') {
        Some(n) => &lstate.input()[..n],
        None => &lstate.input(),
    };

    let state = CharState{input: &this_line, file_name: lstate.file_name(), file_line: lstate.file_line(), column: lstate.column()};

    consume_char_literal_inner(&state)
}

macro_rules! unknown {
    ($st: expr, $cons: expr) => {
        Ok((Unknown($st.input[..$cons].to_string()), $cons))
    };
}

fn consume_char_literal_inner(state: &CharState) -> Result<(Token, usize), ()> {
    let mut consumed: usize = 0;
    let mut output: Vec<i8> = vec![];
    let mut seen_err: bool = false;
    let mut char_peek = state.input.chars().peekable();


    match char_peek.peek() {
        Some('\'') => {
            char_peek.next();
            consumed += 1
        }
        _ => {
            eprintln!("{}:{}:{} - warn - this isn't a char literal", state.file_name, state.file_line, state.column);
            return Err(())
        }
    }
    loop {
        match char_peek.peek() {
            Some(_) => consumed += 1,
            None => ()
        }
        match char_peek.next() {
            Some('\'') => {
                let len = output.len();
                match len  {
                    0 => {
                        eprintln!("{}:{}:{} - error - empty char literal", state.file_name, state.file_line, state.column);
                        return unknown!(state, consumed)
                    },
                    1 => {
                        let val = output[0] as i32;
                        let lit = CharLit(val);

                        if seen_err {
                            eprintln!("{}:{}:{} - warn - seen error in processing, returning Unknown token not CharLit", state.file_name, state.file_line, state.column);
                            return unknown!(state, consumed)
                        }
                        return Ok((lit, consumed))
                    },
                    2 ..= 4 => {
                        eprintln!("{}:{}:{} - warn - multi-character char literal", state.file_name, state.file_line, state.column);
                        let val = output.iter().fold(0, |acc: i32, &u| {
                            let v: i32 = u as i32;
                            let (shifted_acc, _) = (acc as u32).overflowing_shl(8);
                            let masked_shift = shifted_acc & 0xffffff00;
                            let masked_v = v & 0x000000ff;
                            (masked_shift | masked_v as u32) as i32
                        });

                        let lit = CharLit(val);


                        if seen_err {
                            eprintln!("{}:{}:{} - warn - seen error in processing, returning Unknown token not CharLit", state.file_name, state.file_line, state.column);
                            return unknown!(state, consumed)
                        }
                        return Ok((lit, consumed))
                    },
                    5 .. => {
                        eprintln!("{}:{}:{} - warn - multi-character with {} characters exceeds int size of four bytes", state.file_name, state.file_line, state.column, len);
                        let val = output.iter().fold(0, |acc: i32, &u| {
                            let v: i32 = u.try_into().expect("Single i8 should always convert to an i32");
                            let (shifted_acc, _) = (acc as u32).overflowing_shl(8);
                            let masked_shift = shifted_acc & 0xffffff00;
                            let masked_v = v & 0x000000ff;
                            (masked_shift | (masked_v as u32)) as i32
                        });

                        let lit = CharLit(val);

                        if seen_err {
                            eprintln!("{}:{}:{} - warn - seen error in processing, returning Unknown token not CharLit", state.file_name, state.file_line, state.column);
                            return unknown!(state, consumed)
                        }
                        return Ok((lit, consumed))
                    }
                }
            },
            Some('\\') => consume_char_literal_escape(state, &mut char_peek, &mut consumed, &mut output, &mut seen_err),
            Some(c) => {
                let mut bytes = [0; 4];
                c.encode_utf8(&mut bytes);
                for i in 0..c.len_utf8() {
                    let v = bytes[i] as i8;
                    output.push(v)
                }
            },
            None => {
                eprintln!("{}:{}:{} - error - unterminated char literal", state.file_name, state.file_line, state.column);
                return unknown!(state, consumed)
            },
        }
    }
}

fn consume_char_literal_escape(state: &CharState, char_peek: &mut Peekable<Chars>, consumed: &mut usize, output: &mut Vec<i8>, seen_err: &mut bool) {

    macro_rules! emit_escape {
        ($o: literal) => {
            {
                output.push($o as i8);
                char_peek.next();
                *consumed += 1;
            }
        };
    }

    match char_peek.peek() {
        Some('0' ..= '7') => consume_char_literal_escape_oct(state, char_peek, consumed, output, seen_err),
        Some('x') => consume_char_literal_escape_hex(state, char_peek, consumed, output, seen_err),
        Some('\'') => emit_escape!('\''),
        Some('\\') => emit_escape!('\\'),
        Some('\"') => emit_escape!('\"'),
        Some('?') => emit_escape!('?'),
        Some('a') => emit_escape!(0x07),
        Some('b') => emit_escape!(0x08),
        Some('f') => emit_escape!(0x0c),
        Some('n') => emit_escape!('\n'),
        Some('r') => emit_escape!('\r'),
        Some('t') => emit_escape!('\t'),
        Some('v') => emit_escape!(0x0b),
        None => (),
        _ => eprintln!("{}:{}:{} - warn - unknown escape in char literal", state.file_name, state.file_line, state.column),
    }
}

fn consume_char_literal_escape_oct(state: &CharState, char_peek: &mut Peekable<Chars>, consumed: &mut usize, output: &mut Vec<i8>, _seen_err: &mut bool) {
    let mut num_octs = 0;
    let mut octs = String::new();
    while num_octs < 3 && match char_peek.peek() {
        Some('0' ..= '7') => true,
        _ => false,
    } {
        let c = char_peek.next().expect("We just peeked to check");
        octs.push(c);
        *consumed += 1;
        num_octs += 1;
    }
    assert!(num_octs != 0, "{}:{}:{} - FATAL - we found no oct chars?", state.file_name, state.file_line, state.column);

    match i8::from_str_radix(&octs, 8) {
        Ok(i) => output.push(i),
        Err(e) => {
            eprintln!("{}:{}:{} - warn - octal escape sequence out of range: {e}", state.file_name, state.file_line, state.column);
            unsafe {
                // we reduce the leading octal char by 4 which should bring the value into the i8 range
                let oct_bytes = octs.as_bytes_mut();
                oct_bytes[0] -= 4;
            };
            match u8::from_str_radix(&octs, 8) {
                Ok(uval) => {
                    let val = uval as i8;
                    output.push(val)
                },
                Err(e) => panic!("{}:{}:{} - FATAL - We messed up and couldn't parse the new String oct either: {e}", state.file_name, state.file_line, state.column),
            }
        },
    }
}

fn consume_char_literal_escape_hex(state: &CharState, char_peek: &mut Peekable<Chars>, consumed: &mut usize, output: &mut Vec<i8>, seen_err: &mut bool) {
    match char_peek.next() {
        Some('x') => {
            *consumed += 1;
            ()
        },
        _ => panic!("{}:{}:{} - FATAL - this isn't a hex escape", state.file_name, state.file_line, state.column),
    };

    let mut num_hex: usize = 0;
    let mut hexs = String::new();

    while match char_peek.peek() {
        Some('0' ..= '9' | 'a' ..= 'f' | 'A' ..= 'F') => true,
        _ => false,
    } {
        let c = char_peek.next().expect("We just peeked to check");
        hexs.push(c);
        *consumed += 1;
        num_hex += 1;
    }

    match num_hex {
        0 => {
            eprintln!("{}:{}:{} - error - hex escape with no following hex digits", state.file_name, state.file_line, state.column);
            *seen_err = true
        },
        1 | 2 => {
            let uval = u8::from_str_radix(&hexs, 16).expect("We only matched 1 or 2 hex chars, this should parse to a u8");
            let val = uval as i8;
            output.push(val)
        }
        n => {
            eprintln!("{}:{}:{} - warn - hex escape sequence out of range", state.file_name, state.file_line, state.column);
            let uval = u8::from_str_radix(&hexs[n-2..] ,16).expect("We're only looking at 2 hex chars, this should parse to a u8");
            let val = uval as i8;
            output.push(val)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Token;
    use crate::Token::CharLit;
    use crate::Token::Unknown;

    use super::consume_char_literal_inner;
    use super::CharState;

    macro_rules! state_and_unkn {
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
        let (state, exp_t, exp_n) = state_and_unkn!("'");

        let (t, n) = consume_char_literal_inner(&state)
            .expect("expect Unknown token");

        assert_eq!(exp_t, t);
        assert_eq!(exp_n, n);
    }

    #[test]
    fn test_char_literals_empty() {
        let (state, exp_t, exp_n) = state_and_unkn!("''");

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
            state_and_unkn!(r"'\xM'"),
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
}