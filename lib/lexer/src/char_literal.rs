use std::str::Chars;

use crate::LexState;
use crate::Token;
use crate::Token::CharLit;
use crate::Token::Unknown;

pub fn consume_char_literal<'a>(state: &'a LexState, line: &'a mut str) -> (Token<'a>, usize) {
    let mut char_iter = line.chars();
    match char_iter.next() {
        Some('\'') => consume_char_literal_inner(state, char_iter, 1usize, vec![]),
        _ => panic!("this isn't a char literal: [{line}]"),
    }
}

macro_rules! unknown {
    ($st: expr, $cons: expr) => {
        (Unknown(&$st.input()[..$cons]), $cons)
    };
}

fn consume_char_literal_inner<'a>(state: &'a LexState, mut char_iter: Chars, consumed: usize, output: Vec<i8>) -> (Token<'a>, usize) {
    match char_iter.next() {
        Some('\'') => {
            let len = output.len();
            match len  {
                0 => {
                    eprintln!("{}:{}:{} - error - empty char literal", state.file_name(), state.file_line(), state.column());
                    unknown!(state, consumed + 1)
                },
                1 => {
                    let val = output[0].try_into().expect("Single u8 should always convert to an i32");
                    let lit = CharLit(val);
                    (lit, consumed + 1)
                },
                2 ..= 4 => {
                    eprintln!("{}:{}:{} - warn - multi-character char literal", state.file_name(), state.file_line(), state.column());
                    let val = output.iter().fold(0, |acc: i32, &u| {
                        let v: i32 = u.try_into().expect("Single u8 should always convert to an i32");
                        let (shifted_acc, _) = acc.overflowing_shl(8);
                        let masked_shift = shifted_acc & 0xffffff00;
                        let masked_v = v & 0x000000ff;
                        masked_shift | masked_v
                    });

                    let lit = CharLit(val);

                    (lit, consumed + 1)
                },
                5 .. => {
                    eprintln!("{}:{}:{} - warn - multi-character with {} characters exceeds int size of four bytes", state.file_name(), state.file_line(), state.column(), len);
                    let val = output.iter().fold(0, |acc: i32, &u| {
                        let v: i32 = u.try_into().expect("Single u8 should always convert to an i32");
                        let (shifted_acc, _) = acc.overflowing_shl(8);
                        let masked_shift = shifted_acc & 0xffffff00;
                        let masked_v = v & 0x000000ff;
                        masked_shift | masked_v
                    });

                    let lit = CharLit(val);
                    (lit, consumed + 1)
                }
            }
        },
        Some('\\') => consume_char_literal_escape(state, char_iter, consumed + 1, output),
        Some(c) => {
            todo!("normal char")
        },
        None => {
            eprintln!("{}:{}:{} - error - unterminated char literal", state.file_name(), state.file_line(), state.column());
            unknown!(state, consumed)
        },
    }
}

fn consume_char_literal_escape<'a>(state: &'a LexState, mut char_iter: Chars, consumed: usize, output: Vec<i8>) -> (Token<'a>, usize) {
    match char_iter.next() {
        Some('0'..='3') => {
            todo!("oct_low")
        },
        Some('4'..='7') => {
            todo!("oct_hi")
        },
        Some('x') => {
            todo!("hex")
        }
        None => {
            eprintln!("{}:{}:{} - error - unterminated char literal", state.file_name(), state.file_line(), state.column());
            unknown!(state, consumed)
        },
        _ => {
            eprintln!("{}:{}:{} - warn - unknown escape in char literal", state.file_name(), state.file_line(), state.column());
            consume_char_literal_inner(state, char_iter, consumed + 1, output)
        }
    }
}