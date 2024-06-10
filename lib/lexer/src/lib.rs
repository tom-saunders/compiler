mod tokens;
use std::rc::Rc;

use nom::{bytes::complete::{is_not, tag, take}, character::complete::{multispace1, one_of}, combinator::{consumed, not}, multi::many0, sequence::{delimited, pair, preceded, tuple}};
pub use tokens::Token;

use Token::*;

use nom_locate;

type Span<'a> = nom_locate::LocatedSpan<&'a str>;

#[derive(Debug, PartialEq, Clone)]
pub struct LocatedToken<'a> {
    locations: Vec<Location<'a>>,
    token: Token<'a>,
}

impl<'a> LocatedToken<'a> {
    pub fn locations(&'a self) -> &'a Vec<Location<'a>> {
        &self.locations
    }

    pub fn current_location(&'a self) -> &Location<'a>{
        &self.locations.last().expect("must have a location")
    }

    pub fn token(&'a self) -> &Token<'a> {
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

#[derive(Debug, PartialEq, Clone)]
struct LexState<'a> {
    input: &'a str,
    column: usize,
    file_line: u32,
    file_name: &'a str,
    seen_error: bool,
    located_tokens: Vec<LocatedToken<'a>>,
    file_hist: Vec<Location<'a>>,
}

impl<'a> LexState<'a> {
    fn new(input: &'a str) -> Self {
        // we assume that the input starts with a gcc preprocessor linemarker (e.g.):
        // ^# 0 "some_input.c"$
        // see: https://gcc.gnu.org/onlinedocs/cpp/Preprocessor-Output.html 
        // 
        let eol = input.find('\n').expect("We assume at least one complete line");
        let linemarker = &input[..=eol];
        println!("linemarker: [{linemarker}]");

        let (file_line, file_name, _) = match process_linemarker(linemarker) {
            Err(e) => panic!("Error reading first input line as a linemarker: {e}"),
            Ok(o) => o,
        };

        
        let input = &input[eol + 1..];
        let column = 1usize;
        let located_tokens: Vec<LocatedToken> = vec![];
        let file_hist = vec![Location{line: file_line, column, file: file_name, input: linemarker}];
        
        LexState{input, column, file_line, file_name, seen_error: false, located_tokens, file_hist}
    }

    fn is_empty(&self) -> bool {
        self.input.is_empty()
    }

    fn peek(&self) -> char {
        self.input.chars().nth(0).expect("Expect at least one char in input")
    }

    fn peek_nth(&self, n: usize) -> Option<char> {
        self.input.chars().nth(n)
    }

    fn consume(&mut self, n: usize, token: Token<'a>) {
        
        let mut locations = self.file_hist.clone();
        locations.push(Location{line: self.file_line, column: self.column, file: &self.file_name, input: &self.input[..n]});
        self.located_tokens.push(LocatedToken{locations, token});

        
        self.input = &self.input[n..];
        self.column += n;
    }

    fn newline(&mut self) {
        self.file_line += 1;
        self.column = 1;
        self.input = &self.input[1..];
    }
    
    fn enter_file(&mut self, file: &'a str, line: u32, skip: usize) {
        self.file_hist.push(Location{line: self.file_line, column: self.column, file: &self.file_name, input: &self.input[..skip]});

        self.file_name = file;
        self.file_line = line;
        self.column = 1;
        self.input = &self.input[skip..];
    }

    fn exit_file(&mut self, file: &'a str, line: u32, skip: usize) {
        let location = self.file_hist.pop().expect("Expect at least one file");

        self.file_name = file;
        self.file_line = line;
        self.column = 1;
        self.input = &self.input[skip..];

    }
}

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
    Ok((line, file, (one, two, three, four)))
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
                todo!()
            },
            '\n' => {
                state.newline();
            },
            '\t' | '\r' | ' ' => {
                // non-newline whitespace
                todo!()
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
                // decimal numeric float literal: .0 .1
                // or . ... 
                todo!()
            },
            '\'' => {
                // char literal
                todo!()
            }
            '\"' => {
                // string literal
                todo!()
            },
            '&' => {
                // ampersand
                // one of & && &=
                todo!()
            },
            '!' => {
                // bang
                // one of ! !=
                todo!()
            },
            '^' => {
                // caret
                // one of ^ ^=
                todo!()
            },
            ',' => {
                // comma
                // always a ,
                todo!()
            },
            ':' => {
                // colon 
                // one of : :>
                // :> is a compat for ]
                todo!()
            },
            '-' => {
                // dash
                // one of - -- -= ->
                todo!()
            },
            '=' => {
                // equal
                // one of = ==
                todo!()
            },
            '/' => {
                // fslash
                // one of / /=
                todo!()
            },
            '>' => {
                // gthan
                // one of > >= >> >>=
                todo!()
            },
            '{' => {
                // lbrace
                // always a { 
                todo!()
            },
            '[' => {
                // lsquare
                // always a [
                todo!()
            },
            '(' => {
                // lparen
                // always a ( 
                todo!()
            },
            '<' => {
                // lthan
                // one of < <: <= << <<= <%
                // <: is a compat for [
                // <% is a compat for {
                todo!()
            },
            '%' => {
                // pct
                // one of % %= %>
                // %> is a compat for }
                todo!()
            },
            '|' => {
                // pipe
                // one of | || |=
                todo!()
            },
            '+' => {
                // plus
                // one of + += ++
                todo!()
            },
            '?' => {
                // quest
                // always a ? 
                todo!()
            },
            ']' => {
                // rbrace
                // always a ] 
                todo!()
            },
            ')' => {
                // rparen
                // always a )
                todo!()
            },
            ']' => {
                // rsquare
                // always a ]
                todo!()
            },
            ';' => {
                // semi
                // always a ; 
                todo!()
            },
            '*' => {
                // star
                // one of * *=
                todo!()
            },
            '~' => {
                // tilde
                // always a ~ 
                todo!()
            },
            _ => {
                println!("unhandled: {}", c);
            },
        }
    }

    println!("done");
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_test() {
        let input = include_str!("asset/return_0.i");

        lex(input);
    }
}
