mod char_literal;
mod tokens;
use core::hash;
use std::{ops::Index, rc::Rc};

pub use tokens::Token;

use Token::*;

use nom_locate;

type Span<'a> = nom_locate::LocatedSpan<&'a str>;

#[derive(Debug, PartialEq, Clone)]
pub struct LocatedToken<'a> {
    locations: Vec<Location<'a>>,
    token: Token,
}

impl<'a> LocatedToken<'a> {
    pub fn locations(&'a self) -> &'a Vec<Location<'a>> {
        &self.locations
    }

    pub fn current_location(&'a self) -> &Location<'a>{
        &self.locations.last().expect("must have a location")
    }

    pub fn token(&'a self) -> &Token {
        &self.token
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Location<'a> {
    line: u32,
    column: usize,
    file: &'a str,
    input: &'a str,
}

mod state {
    use super::LocatedToken;
    use super::Location;
    use super::Token;
    use super::process_linemarker;
#[derive(Debug, PartialEq, Clone)]
pub struct LexState<'a> {
    input: &'a str,
    column: usize,
    file_line: u32,
    file_name: &'a str,
    seen_error: bool,
    located_tokens: Vec<LocatedToken<'a>>,
    file_hist: Vec<Location<'a>>,
}

impl<'a> LexState<'a> {

    pub fn new(input: &'a str) -> Self {
        // we assume that the input starts with a gcc preprocessor linemarker (e.g.):
        // ^# 0 "some_input.c"$
        // see: https://gcc.gnu.org/onlinedocs/cpp/Preprocessor-Output.html
        //
        let eol = input.find('\n').or(Some(0)).unwrap();
        let linemarker = &input[..=eol];

        let ((file_line, file_name, _), skip) = match process_linemarker(linemarker) {
            Err(e) => ((0, "TOPLEVEL", (false, false, false, false)), 0),
            Ok(o) => (o, linemarker.len() + 1),
        };


        let input = &input[skip..];
        let column = 1usize;
        let located_tokens: Vec<LocatedToken> = vec![];
        let file_hist = vec![Location{line: file_line, column, file: file_name, input: linemarker}];

        LexState{input, column, file_line, file_name, seen_error: false, located_tokens, file_hist}
    }

    pub fn len(&self) -> usize {
        self.input.len()
    }

    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }

    pub fn peek(&self) -> char {
        self.input.chars().nth(0).expect("Expect at least one char in input")
    }

    pub fn skip(&mut self, n: usize) {
        self.input = &self.input[n..];
    }

    pub fn peek_nth(&self, n: usize) -> Option<char> {
        self.input.chars().nth(n)
    }

    pub fn consume(&mut self, n: usize, token: Token) {
        let mut locations = self.file_hist.clone();
        locations.push(Location{line: self.file_line, column: self.column, file: &self.file_name, input: &self.input[..n]});
        self.located_tokens.push(LocatedToken{locations, token});


        self.input = &self.input[n..];
        self.column += n;
    }

    pub fn error(&mut self, n: usize) {
        self.seen_error = true;
        let mut locations = self.file_hist.clone();
        locations.push(Location{line: self.file_line, column: self.column, file: &self.file_name, input: &self.input[..n]});
        self.located_tokens.push(LocatedToken{locations, token: Token::Unknown(self.input[..n].to_string())});


        self.input = &self.input[n..];
        self.column += n;
    }

    pub fn newline(&mut self) {
        self.file_line += 1;
        self.column = 1;
        self.input = &self.input[1..];
    }

    pub fn enter_file(&mut self, file: &'a str, line: u32, skip: usize) {
        self.file_hist.push(Location{line: self.file_line, column: self.column, file: &self.file_name, input: &self.input[..skip]});

        self.file_name = file;
        self.file_line = line;
        self.column = 1;
        self.input = &self.input[skip..];
    }

    pub fn exit_file(&mut self, file: &'a str, line: u32, skip: usize) {
        self.file_hist.pop().expect("Expect at least one file");

        self.file_name = file;
        self.file_line = line;
        self.column = 1;
        self.input = &self.input[skip..];
    }

    pub fn move_in_file(&mut self, line: u32, skip: usize) {
        self.file_line = line;
        self.column = 1;
        self.input = &self.input[skip..];
    }

    pub fn input(&self) -> &'a str {
        &self.input
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn file_line(&self) -> u32 {
        self.file_line
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn seen_error(&self) -> bool {
        self.seen_error
    }

    pub fn tokens(&self) -> Vec<LocatedToken<'a>> {
        self.located_tokens.clone()
    }

}
}

use state::LexState;

fn process_linemarker(linemarker: &str) -> Result<(u32, &str, (bool, bool, bool, bool)), String> {
    let processing = linemarker.strip_prefix("# ").ok_or(format!("Expected linemarker to start [# ] but got [{linemarker}]"))?;
    let line_end = processing.find(' ').ok_or(format!("Expected to find whitespace in processing: [{processing}]"))?;
    let line_str = &processing[..line_end];
    let line: u32 = line_str.parse().or_else(|_| Err(format!("Expected to parse u32 from line_str: {line_str}")))?;

    let processing = &processing[line_end..];
    let quote_start = processing.find('"').ok_or(format!("Expected to find open quote in linemarker: [{processing}]"))?;
    let processing = &processing[quote_start + 1..];
    let quote_end = processing.rfind('"').ok_or(format!("Expected to find close quote in linemarker: [{processing}]"))?;
    let file = &processing[..quote_end];
    let processing = &processing[quote_end + 1..];

    let (one, processing) = match processing.strip_prefix(" 1") {
        Some(p) => (true, p),
        None => (false, processing),
    };
    let (two, processing) = match processing.strip_prefix(" 2") {
        Some(p) => (true, p),
        None => (false, processing),
    };
    let (three, processing) = match processing.strip_prefix(" 3") {
        Some(p) => (true, p),
        None => (false, processing),
    };
    let (four, processing) = match processing.strip_prefix(" 4") {
        Some(p) => (true, p),
        None => (false, processing),
    };

    if processing.len() > 2 {
        eprintln!("Unexpected trailing input on linemarker: [{processing}]");
    }
    Ok((line, file, (one, two, three, four)))
}

fn lex_char_literal<'a>(state: &'a mut LexState) {
    match char_literal::consume_char_literal(state) {
        Ok((t, n)) => state.consume(n, t),
        Err(()) => panic!("{}:{}:{} - FATAL - we haven't found a character literal: {}", state.file_name(), state.file_name(), state.column(), &state.input()[..10]),
    }
/*
    let this_line = match state.input().find('\n') {
        Some(n) => &state.input()[..n],
        None => &state.input(),
    };

    struct CharLitEndMatcher {
        last: char,
        second_last: char,
    }
    impl CharLitEndMatcher {
        fn do_match(&mut self, c: char) -> bool {
            let m = match (c, self.last, self.second_last) {
                ('\'', '\\', '\\') => true,
                ('\'', '\\', _) => false,
                ('\'', _, _) => true,
                _ => false,
            };
            self.second_last = self.last;
            self.last = c;
            m
        }
    }

    // setting last to '\\' here cheeses the fact that the first char is a ' :)
    // this doesn't technically handle '\\\' correctly - it should be an unterminated char literal
    let mut clem = CharLitEndMatcher{last: '\\', second_last: ' '};
    match this_line.find(|c| clem.do_match(c)) {
        None => {
            eprintln!("{}:{}:{} - error - unterminated char literal", state.file_name(), state.file_line(), state.column());
            state.error(this_line.len())
        },
        Some(1) => {
            eprintln!("{}:{}:{} - error - empty char literal", state.file_name(), state.file_line(), state.column());
            state.error(2)
        },
        Some(2) => {
            let c = this_line.chars().nth(1).expect("If the terminating ' is at idx 2 then getting idx 1 should work");
            let c_u8 = c.try_into().expect("Input should be constrained to ASCII");
            state.consume(3, CharLit(c_u8))
        },
        Some(3) => {
            let mut chars = this_line.chars();
            // could be an escape or a multichar
            match chars.nth(1).expect("Within valid &str") {
                '\\' => {
                    // ok so far
                    match chars.next().expect("Within valid &str") {
                        '0'..='7' => {
                            let v = u8::from_str_radix(&this_line[2..3], 8).expect("we know this value should only contain octal valid chars");
                            state.consume(4, CharLit(v))
                        }
                        '\'' => state.consume(4, CharLit(b'\'')),
                        '\"' => state.consume(4, CharLit(b'\"')),
                        '?' => state.consume(4, CharLit(b'?')),
                        '\\' => state.consume(4, CharLit(b'\\')),
                        'a' => state.consume(4, CharLit(0x07)),
                        'b' => state.consume(4, CharLit(0x08)),
                        'f' => state.consume(4, CharLit(0x0c)),
                        'n' => state.consume(4, CharLit(b'\n')),
                        'r' => state.consume(4, CharLit(b'\r')),
                        't' => state.consume(4, CharLit(b'\t')),
                        'v' => state.consume(4, CharLit(0x0b)),
                        'x' => {
                            eprintln!("{}:{}:{} - error - hex escape used with no following hex digits in char literal", state.file_name(), state.file_line(), state.column());
                            state.error(4)
                        }
                        _ => {
                            eprintln!("{}:{}:{} - error - unknown escape within char literal", state.file_name(), state.file_line(), state.column());
                            state.error(4)
                        },
                    }
                },
                _ => {
                    eprintln!("{}:{}:{} - error - multi-character char literal", state.file_name(), state.file_line(), state.column());
                    state.error(4)
                }
            }
        },
        Some(4) => {
            // either an octal escape \01 or a hex escape \xa or a multi-character char literal
            let mut chars = this_line.chars();
            match chars.nth(1).expect("Within valid &str") {
                '\\' => {
                    // escape
                    match chars.next().expect("Within valid &str") {
                        '0'..='7' => {
                            match chars.next().expect("Within valid &str") {
                                '0'..='7' => {
                                    let v = u8::from_str_radix(&this_line[2..4], 8).expect("we know this value should only contain octal valid chars");
                                    state.consume(5, CharLit(v))
                                },
                                _ => {
                                    eprintln!("{}:{}:{} - error - unexpected escape or multi-character char literal", state.file_name(), state.file_line(), state.column());
                                    state.error(5)
                                }
                            }
                        },
                        'x' => {
                            match chars.next().expect("Within valid &str") {
                                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                                    let v = u8::from_str_radix(&this_line[3..4], 16).expect("we know this value should only contain hex valid chars");
                                    state.consume(5, CharLit(v))
                                }
                                _ => {
                                    eprintln!("{}:{}:{} - error - hex escape used with no following hex digits in char literal", state.file_name(), state.file_line(), state.column());
                                    state.error(5)
                                }
                            }
                        },
                        _ => {
                            eprintln!("{}:{}:{} - error - unexpected escape or multi-character char literal", state.file_name(), state.file_line(), state.column());
                            state.error(5)
                        }
                    }
                },
                _ => {
                    eprintln!("{}:{}:{} - error - multi-character char literal", state.file_name(), state.file_line(), state.column());
                    state.error(5)
                },
            }
        },
        Some(5) => {
            let mut chars = this_line.chars();
            // either an octal escape \012 or a hex escape \xab or a multi-character char literal
            match chars.nth(1).expect("Within valid &str") {
                '\\' => {
                    // escape
                    match chars.next().expect("Within valid &str") {
                        '0'..='3' => {
                            match chars.next().expect("Within valid &str") {
                                '0'..='7' => {
                                    match chars.next().expect("Within valid &str") {
                                        '0'..='7' => {
                                            let v = u8::from_str_radix(&this_line[2..5], 8).expect("we know this value should only contain octal valid chars");
                                            state.consume(6, CharLit(v))
                                        },
                                        _ => {
                                            eprintln!("{}:{}:{} - error - unexpected escape or multi-character char literal", state.file_name(), state.file_line(), state.column());
                                            state.error(6)
                                        }
                                    }
                                },
                                _ => {
                                    eprintln!("{}:{}:{} - error - unexpected escape or multi-character char literal", state.file_name(), state.file_line(), state.column());
                                    state.error(6)
                                }
                            }
                        },
                        'x' => {
                            match chars.next().expect("Within valid &str") {
                                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                                    match chars.next().expect("Within valid &str") {
                                        '0'..='9' | 'a'..='f' | 'A'..='F' => {
                                            let v = u8::from_str_radix(&this_line[3..5], 16).expect("we know this value should only contain hex valid chars");
                                            state.consume(6, CharLit(v))
                                        },
                                        _ => {
                                            eprintln!("{}:{}:{} - error - unexpected escape or multi-character char literal", state.file_name(), state.file_line(), state.column());
                                            state.error(6)
                                        }
                                    }
                                },
                                _ => {
                                    eprintln!("{}:{}:{} - error - hex escape used with no following hex digits in char literal", state.file_name(), state.file_line(), state.column());
                                    state.error(6)
                                }
                            }
                        },
                        _ => {
                            eprintln!("{}:{}:{} - error - unexpected escape or multi-character char literal", state.file_name(), state.file_line(), state.column());
                            state.error(6)
                        },
                    }
                },
                _ => {
                    eprintln!("{}:{}:{} - error - multi-character char literal", state.file_name(), state.file_line(), state.column());
                    state.error(6)
                },
            };
        },
        Some(n) => {
            eprintln!("{}:{}:{} - error - overlong char literal", state.file_name(), state.file_line(), state.column());
            state.error(n + 1)
        },
    }
*/
}

pub fn lex<'a>(input: &'a str) -> Result<Vec<LocatedToken<'a>>, ()> {
    let mut state = LexState::new(input);
    println!("state: {state:?}");
    while ! state.is_empty() {
        let c = state.peek();

        match c {
            '#' => {
                // either a linemarker (which we can deal with)
                // or a #pragma or an #ident (which we ?ignore?)
                let hashline = match state.input().find('\n') {
                    Some(n) => &state.input()[..n + 1],
                    None => {
                        eprintln!("{}:{}:{} - warn - Found hashline without terminating newline", state.file_name(), state.file_line(), state.column());
                        state.input()
                    },
                };
                match process_linemarker(hashline) {
                    Ok((line, file, (one, two, _, _))) => {
                        if one {
                            println!("enter: {file} at {line}");
                            state.enter_file(file, line, hashline.len());
                        } else if two {
                            println!("exit: from {} into {} at {}", state.file_name(), file, line);
                            state.exit_file(file, line, hashline.len());
                        } else {
                            println!("move: {} at {}", file, line);
                            state.move_in_file(line, hashline.len());
                        }
                    },
                    Err(e) => {
                        eprintln!("{}:{}:{} - warn - Didn't match a linemarker with err: [{}]", state.file_name(), state.file_line(), state.column(), e);
                        eprintln!("{}:{}:{} - warn - linemarker input was: [{}]", state.file_name(), state.file_line(), state.column(), hashline);
                        state.skip(hashline.len());
                    },
                }
            },
            '\n' => {
                state.newline();
            },
            '\t' | '\r' | ' ' => {
                // non-newline whitespace
                let whitespaces = match state.input().find(|c| c != '\t' && c != '\r' && c != ' ') {
                    Some(n) => n,
                    None => {
                        eprintln!("{}:{}:{} - warn trailing whitespace at end of file", state.file_name(), state.file_line(), state.column());
                        state.input().len()
                    },
                };
                state.skip(whitespaces);
            },
            'a'..='z' | 'A'..='Z' | '_' => {
                // identifier or keyword
                todo!()
            },
            '0' => {
                // octal or hex numeric literal
                todo!()
            },
            '1'..='9' => {
                // decimal numeric literal
                todo!()
            },
            '.' => {
                // one of
                // decimal numeric float literal: .0 .1 etc.
                // or . ...
                match state.peek_nth(1) {
                    Some(c) => match c {
                        '0'..='9' => {
                            // decimal float literal
                            todo!()
                        },
                        '.' => {
                            // either two dots or an ellipsis
                            match state.peek_nth(2) {
                                Some(d) => match d {
                                    '0'..='9' => {
                                        // dot then a decimal float literal, but for simplicity emit the dot
                                        // and then go back to the start
                                        state.consume(1, Token::Dot)
                                    },
                                    '.' => {
                                        // ellipsis
                                        state.consume(3, Token::Ellipsis)
                                    }
                                    _ => {
                                        // two dots
                                        state.consume(1, Token::Dot);
                                        state.consume(1, Token::Dot);
                                    }
                                },
                                None => {
                                    // EOF but we still emit
                                    state.consume(1, Token::Dot);
                                    state.consume(1, Token::Dot);
                                }
                            }
                        },
                        _ => state.consume(1, Token::Dot),
                    },
                    None => {
                        // EOF but still emit
                        state.consume(1, Token::Dot)
                    }
                }
            },
            '\'' => {
                // char literal
                lex_char_literal(&mut state);
            }
            '\"' => {
                // string literal
                todo!()
            },
            '&' => {
                // ampersand
                // one of & && &=
                match  state.peek_nth(1) {
                    Some('&') => state.consume(2, Token::AmpAmp),
                    Some('=') => state.consume(2, Token::AmpEql),
                    _ => state.consume(1, Token::Amp),
                }
            },
            '!' => {
                // bang
                // one of ! !=
                match state.peek_nth(1) {
                    Some('=') => state.consume(2, Token::BangEql),
                    _ => state.consume(1, Token::Bang),
                }
            },
            '^' => {
                // caret
                // one of ^ ^=
                match state.peek_nth(1) {
                    Some('=') => state.consume(2, Token::CaretEql),
                    _ => state.consume(1, Token::Caret),
                }
            },
            ':' => {
                // colon
                // one of : :>
                // :> is a compat for ]
                match state.peek_nth(1) {
                    Some('>') => state.consume(2, Token::RSquare),
                    _ => state.consume(1, Token::Colon),
                }
            },
            ',' => {
                // comma
                // always a ,
                state.consume(1, Token::Comma)
            },
            '-' => {
                // dash
                // one of - -- -= ->
                match state.peek_nth(1) {
                    Some('-') => state.consume(2, Token::DashDash),
                    Some('=') => state.consume(2, Token::DashEql),
                    Some('>') => state.consume(2, Token::DashGTh),
                    _ => state.consume(1, Token::Dash),
                }
            },
            '=' => {
                // equal
                // one of = ==
                match state.peek_nth(1) {
                    Some('=') => state.consume(2, Token::EqlEql),
                    _ => state.consume(1, Token::Eql),
                }
            },
            '/' => {
                // fslash
                // one of / /=
                match state.peek_nth(1) {
                    Some('=') => state.consume(2, Token::FSlEql),
                    _ => state.consume(1, Token::FSl),
                }
            },
            '>' => {
                // gthan
                // one of > >= >> >>=
                match state.peek_nth(1) {
                    Some('=') => state.consume(2, Token::GThEql),
                    Some('>') => {
                        match state.peek_nth(2) {
                            Some('=') => state.consume(3, Token::GThGThEql),
                            _ => state.consume(2, Token::GThGTh),
                        }
                    }
                    _ => state.consume(1, Token::GTh),
                }
            },
            '{' => {
                // lbrace
                // always a {
                state.consume(1, Token::LBrace)
            },
            '[' => {
                // lsquare
                // always a [
                state.consume(1, Token::LSquare)
            },
            '(' => {
                // lparen
                // always a (
                state.consume(1, Token::LParen)
            },
            '<' => {
                // lthan
                // one of < <: <= << <<= <%
                // <: is a compat for [
                // <% is a compat for {
                match state.peek_nth(1) {
                    Some(':') => state.consume(2, Token::LSquare),
                    Some('=') => state.consume(2, Token::LThEql),
                    Some('>') => {
                        match state.peek_nth(2) {
                            Some('=') => state.consume(3, Token::LThLThEql),
                            _ => state.consume(2, Token::LThLTh),
                        }
                    }
                    Some('%') => state.consume(2, Token::LBrace),
                    _ => state.consume(1, Token::LTh),
                }
            },
            '%' => {
                // pct
                // one of % %= %>
                // %> is a compat for }
                match state.peek_nth(1) {
                    Some('=') => state.consume(2, Token::PctEql),
                    Some('>') => state.consume(2, Token::RBrace),
                    _ => state.consume(1, Token::Pct),
                }
            },
            '|' => {
                // pipe
                // one of | || |=
                match state.peek_nth(1) {
                    Some('=') => state.consume(2, Token::PipeEql),
                    Some('|') => state.consume(2, Token::PipePipe),
                    _ => state.consume(1, Token::Pipe),
                }
            },
            '+' => {
                // plus
                // one of + += ++
                match state.peek_nth(1) {
                    Some('=') => state.consume(2, Token::PlusEql),
                    Some('+') => state.consume(2, Token::PlusPlus),
                    _ => state.consume(1, Token::Plus),
                }
            },
            '?' => {
                // quest
                // always a ?
                state.consume(1, Token::Question)
            },
            '}' => {
                // rbrace
                // always a ]
                state.consume(1, Token::RBrace)
            },
            ')' => {
                // rparen
                // always a )
                state.consume(1, Token::RParen)
            },
            ']' => {
                // rsquare
                // always a ]
                state.consume(1, Token::RSquare)
            },
            ';' => {
                // semi
                // always a ;
                state.consume(1, Token::Semi)
            },
            '*' => {
                // star
                // one of * *=
                match state.peek_nth(1) {
                    Some('=') => state.consume(2, Token::StarEql),
                    _ => state.consume(1, Token::Plus),
                }
            },
            '~' => {
                // tilde
                // always a ~
                state.consume(1, Token::Tilde)
            },
            _ => {
                println!("unhandled: {}", c);
                state.error(1)
            },
        }
    }

    println!("done");
    Ok(state.tokens())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_return_0() {
        let input = include_str!("asset/return_0.i");

        lex(input);
    }

    #[test]
    fn test_include_a() {
        let input = include_str!("asset/include_a.i");

        lex(input);
    }

}
