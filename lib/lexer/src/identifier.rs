use std::iter::Peekable;
use std::str::Chars;

use crate::Token;
use crate::Token::Unknown;
use crate::LexStruct;

struct IdentState<'state>{
    input: &'state str,
    file_name: &'state str,
    file_line: u32,
    column: usize,
}

pub fn consume_identifier<'state> (lstate: &'state LexStruct) -> Result<(Token, usize), ()> {
    let this_line = match lstate.input().find('\n') {
        Some(n) => &lstate.input()[..n],
        None => &lstate.input(),
    };

    let state = IdentState{input: &this_line, file_name: lstate.file_name(), file_line: lstate.file_line(), column: lstate.column()};

    consume_identifier_inner(&state)
}

macro_rules! unknown {
    ($st: expr, $cons: expr) => {
        Ok((Unknown($st.input[..$cons].to_string()), $cons))
    };
}
fn consume_identifier_inner(state: &IdentState) -> Result<(Token, usize), ()> {
    let mut consumed: usize = 0;
    let mut output: String = String::new();
    let mut seen_err: bool = false;
    let mut char_peek = state.input.chars().peekable();

    match char_peek.peek() {
        Some('a' ..= 'z' | 'A' ..= 'Z' | '_' | '\\') => (),
        _ => {
            eprintln!("{}:{}:{} - warn - this isn't an identifier", state.file_name, state.file_line, state.column);
            return Err(());
        }
    };

    match char_peek.next() {
        Some(c @ ('a' ..= 'z' | 'A' ..= 'Z' | '_')) => {
            consumed += 1;
            output.push(c)
        },
        Some('\\') => {
            consumed += 1;
            match char_peek.peek() {
                Some('u') => consume_identifier_universal_short(state, &mut char_peek, &mut consumed, &mut output, &mut seen_err),
                Some('U') => consume_identifier_universal_long(state, &mut char_peek, &mut consumed, &mut output, &mut seen_err),
                _ => {
                    eprintln!("{}:{}:{} - error - unexpected escape after \\ in identifier", state.file_name, state.file_line, state.column);
                    return unknown!(state, consumed)
                }
            }
        },
        _ => {
            panic!("{}:{}:{} - FATAL - We just peeked to check this wasn't the case", state.file_name, state.file_line, state.column)
        }
    };

    loop {
        match char_peek.peek() {
            Some('a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_' | '\\') => consumed += 1,
            _ => {
                return keyword_or_literal(state, consumed, output)
            },
        }
        match char_peek.next() {
            Some(c @ ('a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '_')) => output.push(c),
            Some('\\') => {
                match char_peek.peek() {
                    Some('u') => consume_identifier_universal_short(state, &mut char_peek, &mut consumed, &mut output, &mut seen_err),
                    Some('U') => consume_identifier_universal_long(state, &mut char_peek, &mut consumed, &mut output, &mut seen_err),
                    _ => {
                        eprintln!("{}:{}:{} - error - unexpected escape after \\ in identifier", state.file_name, state.file_line, state.column);
                        return unknown!(state, consumed)
                    }
                }
            }
            _ => {
                panic!("{}:{}:{} - FATAL - We just peeked to check this wasn't the case", state.file_name, state.file_line, state.column)
            }
        }
        todo!()
    }
}

fn keyword_or_literal(state: &IdentState, consumed: usize, output: String) -> Result<(Token, usize), ()> {
    match output.as_str() {
        "auto" => Ok((Token::KwAuto, consumed)),
        "break" => Ok((Token::KwBreak, consumed)),
        "case" => Ok((Token::KwCase, consumed)),
        "char" => Ok((Token::KwChar, consumed)),
        "const" => Ok((Token::KwConst, consumed)),
        "continue" => Ok((Token::KwContinue, consumed)),
        "default" => Ok((Token::KwDefault, consumed)),
        "do" => Ok((Token::KwDo, consumed)),
        "double" => Ok((Token::KwDouble, consumed)),
        "else" => Ok((Token::KwElse, consumed)),
        "enum" => Ok((Token::KwEnum, consumed)),
        "extern" => Ok((Token::KwExtern, consumed)),
        "float" => Ok((Token::KwFloat, consumed)),
        "for" => Ok((Token::KwFor, consumed)),
        "goto" => Ok((Token::KwGoto, consumed)),
        "if" => Ok((Token::KwIf, consumed)),
        "inline" => Ok((Token::KwInline, consumed)),
        "int" => Ok((Token::KwInt, consumed)),
        "long" => Ok((Token::KwLong, consumed)),
        "register" => Ok((Token::KwRegister, consumed)),
        "restrict" => Ok((Token::KwRestrict, consumed)),
        "return" => Ok((Token::KwReturn, consumed)),
        "short" => Ok((Token::KwShort, consumed)),
        "signed" => Ok((Token::KwSigned, consumed)),
        "sizeof" => Ok((Token::KwSizeof, consumed)),
        "static" => Ok((Token::KwStatic, consumed)),
        "struct" => Ok((Token::KwStruct, consumed)),
        "switch" => Ok((Token::KwSwitch, consumed)),
        "typedef" => Ok((Token::KwTypedef, consumed)),
        "union" => Ok((Token::KwUnion, consumed)),
        "unsigned" => Ok((Token::KwUnsigned, consumed)),
        "void" => Ok((Token::KwVoid, consumed)),
        "while" => Ok((Token::KwWhile, consumed)),
        "_Alignas" => Ok((Token::Kw_Alignas, consumed)),
        "_Alignof" => Ok((Token::Kw_Alignof, consumed)),
        "_Atomic" => Ok((Token::Kw_Atomic, consumed)),
        "_Bool" => Ok((Token::Kw_Bool, consumed)),
        "_Complex" => Ok((Token::Kw_Complex, consumed)),
        "_Generic" => Ok((Token::Kw_Generic, consumed)),
        "_Imaginary" => Ok((Token::Kw_Imaginary, consumed)),
        "_Noreturn" => Ok((Token::Kw_Noreturn, consumed)),
        "_Static_assert" => Ok((Token::Kw_Static_assert, consumed)),
        "_Thread_local" => Ok((Token::Kw_Thread_local, consumed)),
        _ => Ok((Token::Identifier(output), consumed)),
    }
}

fn consume_identifier_universal_short(state: &IdentState, char_peek: &mut Peekable<Chars>, consumed: &mut usize, output: &mut String, seen_err: &mut bool) {
    match char_peek.next() {
        Some('u') => {
            *consumed += 1;
            ()
        },
        _ => panic!("{}:{}:{} - FATAL - this isn't a long universal escape", state.file_name, state.file_line, state.column),
    };
    let mut num_hex: usize = 0;
    let mut hexs = String::new();

    while num_hex < 4 && match char_peek.peek() {
        Some('0' ..= '9' | 'a' ..= 'f' | 'A' ..= 'F') => true,
        _ => false,
    } {
        let c = char_peek.next().expect("We just peeked to check");
        hexs.push(c);
        *consumed += 1;
        num_hex += 1;
    }

    match num_hex {
        4 => {
            // good
            let uval = u32::from_str_radix(&hexs, 16).expect("We've just scanned for four hex chars of input");

            let meets_universal_constraints = match uval {
                0x24 | 0x40 | 0x60 => true,
                0 ..= 0x9f => false,
                0xd800 ..= 0xdfff => false,
                _ => true,
            };
            if ! meets_universal_constraints {
                eprintln!("{}:{}:{} - error - invalid universal character name \\u{}", state.file_name, state.file_line, state.column, hexs);
                *seen_err = true;
                return
            }

            let meets_identifier_constraints = match uval {
                0x0300 ..= 0x036f => false,
                0x1dc0 ..= 0x1dff => false,
                0x20d0 ..= 0x20ff => false,
                0xfe20 ..= 0xfe2f => false,
                _ => true,
            };
            if ! meets_identifier_constraints {
                eprintln!("{}:{}:{} - error - universal character name \\u{} is not valid in identifiers", state.file_name, state.file_line, state.column, hexs);
                *seen_err = true;
                return
            }

            match char::from_u32(uval) {
                Some(c) => {
                    output.push(c);
                },
                None => {
                    eprintln!("{}:{}:{} - error - universal character name \\u{} does not map to a char", state.file_name, state.file_line, state.column, hexs);
                    *seen_err = true
                },
            }
        }
        n => {
            eprintln!("{}:{}:{} - error - incomplete universal character name \\u{}", state.file_name, state.file_line, state.column, hexs);
            *seen_err = true
        },
    }
    todo!()
}

fn consume_identifier_universal_long(state: &IdentState, char_peek: &mut Peekable<Chars>, consumed: &mut usize, output: &mut String, seen_err: &mut bool) {
    match char_peek.next() {
        Some('U') => {
            *consumed += 1;
            ()
        },
        _ => panic!("{}:{}:{} - FATAL - this isn't a long universal escape", state.file_name, state.file_line, state.column),
    };
    let mut num_hex: usize = 0;
    let mut hexs = String::new();

    while num_hex < 8 && match char_peek.peek() {
        Some('0' ..= '9' | 'a' ..= 'f' | 'A' ..= 'F') => true,
        _ => false,
    } {
        let c = char_peek.next().expect("We just peeked to check");
        hexs.push(c);
        *consumed += 1;
        num_hex += 1;
    }

    match num_hex {
        8 => {
            // good
            let uval = u32::from_str_radix(&hexs, 16).expect("We've just scanned for eight hex chars of input");

            let meets_universal_constraints = match uval {
                0x24 | 0x40 | 0x60 => true,
                0 ..= 0x9f => false,
                0xd800 ..= 0xdfff => false,
                _ => true,
            };
            if ! meets_universal_constraints {
                eprintln!("{}:{}:{} - error - invalid universal character name \\U{}", state.file_name, state.file_line, state.column, hexs);
                *seen_err = true;
                return
            }

            let meets_identifier_constraints = match uval {
                0x0300 ..= 0x036f => false,
                0x1dc0 ..= 0x1dff => false,
                0x20d0 ..= 0x20ff => false,
                0xfe20 ..= 0xfe2f => false,
                _ => true,
            };
            if ! meets_identifier_constraints {
                eprintln!("{}:{}:{} - error - universal character name \\U{} is not valid in identifiers", state.file_name, state.file_line, state.column, hexs);
                *seen_err = true;
                return
            }

            match char::from_u32(uval) {
                Some(c) => {
                    output.push(c);
                },
                None => {
                    eprintln!("{}:{}:{} - error - universal character name \\U{} does not map to a char", state.file_name, state.file_line, state.column, hexs);
                    *seen_err = true
                },
            }
        }
        n => {
            eprintln!("{}:{}:{} - error - incomplete universal character name \\U{}", state.file_name, state.file_line, state.column, hexs);
            *seen_err = true
        },
    }
    todo!()
}

#[cfg(test)]
mod test{
    use super::IdentState;

    use crate::Token::Identifier;
    use crate::Token::Unknown;
}