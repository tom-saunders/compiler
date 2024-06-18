#[cfg(test)]
mod tests;

use crate::{text::TextState, universal_char::UnivEsc, LocationState, Token};

pub trait Identifier {
    fn consume_identifier(&self) -> Token;
}

pub fn identifier_impl<'iter>(
    location: &'iter dyn LocationState,
    text: &'iter dyn TextState<Ch = i8>,
    univ: &'iter dyn UnivEsc,
) -> Box<dyn Identifier + 'iter> {
    Box::new(IdentifierImpl::new(location, text, univ))
}

struct IdentifierImpl<'iter> {
    location: &'iter dyn LocationState,
    text: &'iter dyn TextState<Ch = i8>,
    univ: &'iter dyn UnivEsc,
}

impl<'iter> IdentifierImpl<'iter> {
    fn new(
        location: &'iter dyn LocationState,
        text: &'iter dyn TextState<Ch = i8>,
        univ: &'iter dyn UnivEsc,
    ) -> IdentifierImpl<'iter> {
        IdentifierImpl {
            location,
            text,
            univ,
        }
    }
}

impl<'iter> Identifier for IdentifierImpl<'iter> {
    fn consume_identifier(&self) -> Token {
        match self.text.peek() {
            Some(c @ ('a'..='z' | 'A'..='Z' | '_')) => {
                self.text.push_char(c);
                self.text.next();
            }
            Some('\\') => {
                self.text.next();
                match self.text.peek() {
                    Some('u') => self.univ.consume_universal_short_identifier(),
                    Some('U') => self.univ.consume_universal_long_identifier(),
                    _ => {
                        eprintln!(
                            "{}:{}:{} - error - stray \\ in program",
                            self.location.f(),
                            self.location.l(),
                            self.location.c()
                        );
                        self.text.report_error()
                    }
                }
            }
            _ => {
                panic!(
                    "{}:{}:{} - FATAL - invalid initial character for identifier",
                    self.location.f(),
                    self.location.l(),
                    self.location.c()
                )
            }
        };
        let ident_i8s = loop {
            match self.text.peek() {
                Some(c @ ('a'..='z' | 'A'..='Z' | '0'..='9' | '_')) => {
                    self.text.push_char(c);
                    self.text.next();
                }
                Some('\\') => {
                    self.text.next();
                    match self.text.peek() {
                        Some('u') => self.univ.consume_universal_short_identifier(),
                        Some('U') => self.univ.consume_universal_long_identifier(),
                        _ => {
                            eprintln!(
                                "{}:{}:{} - error - stray '\' in program",
                                self.location.f(),
                                self.location.l(),
                                self.location.c()
                            );
                            self.text.report_error()
                        }
                    }
                }
                _ => break self.text.get_output(),
            }
        };

        if self.text.seen_error() {
            eprintln!("{}:{}:{} - warn - seen error in processing identifier, returning Unknown token not Identifier", self.location.f(), self.location.l(), self.location.c());
            self.text.emit_unknown()
        } else {
            let ident_u8s = ident_i8s.iter().map(|i| *i as u8).collect();
            let ident = String::from_utf8(ident_u8s).expect("We should only have valid UTF-8 here");
            match ident.as_str() {
                "auto" => Token::KwAuto,
                "break" => Token::KwBreak,
                "case" => Token::KwCase,
                "char" => Token::KwChar,
                "const" => Token::KwConst,
                "continue" => Token::KwContinue,
                "default" => Token::KwDefault,
                "do" => Token::KwDo,
                "double" => Token::KwDouble,
                "else" => Token::KwElse,
                "enum" => Token::KwEnum,
                "extern" => Token::KwExtern,
                "float" => Token::KwFloat,
                "for" => Token::KwFor,
                "goto" => Token::KwGoto,
                "if" => Token::KwIf,
                "inline" => Token::KwInline,
                "int" => Token::KwInt,
                "long" => Token::KwLong,
                "register" => Token::KwRegister,
                "restrict" => Token::KwRestrict,
                "return" => Token::KwReturn,
                "short" => Token::KwShort,
                "signed" => Token::KwSigned,
                "sizeof" => Token::KwSizeof,
                "static" => Token::KwStatic,
                "struct" => Token::KwStruct,
                "switch" => Token::KwSwitch,
                "typedef" => Token::KwTypedef,
                "union" => Token::KwUnion,
                "unsigned" => Token::KwUnsigned,
                "void" => Token::KwVoid,
                "while" => Token::KwWhile,
                "_Alignas" => Token::Kw_Alignas,
                "_Alignof" => Token::Kw_Alignof,
                "_Atomic" => Token::Kw_Atomic,
                "_Bool" => Token::Kw_Bool,
                "_Complex" => Token::Kw_Complex,
                "_Generic" => Token::Kw_Generic,
                "_Imaginary" => Token::Kw_Imaginary,
                "_Noreturn" => Token::Kw_Noreturn,
                "_Static_assert" => Token::Kw_Static_assert,
                "_Thread_local" => Token::Kw_Thread_local,
                _ => Token::Identifier(ident),
            }
        }
    }
}
