mod i8_text;
mod i16_text;
mod i32_text;

use std::{cell::RefCell, iter::Peekable, str::Chars};

use crate::{LocationState, Token};

pub trait TextState {
    type Ch;
    fn peek(&self) -> Option<char>;
    fn next(&self) -> Option<char>;

    fn emit_unknown(&self) -> Token;
    fn emit_char_lit(&self, location: &dyn LocationState) -> Token;
    fn emit_string_lit(&self) -> Token;

    fn push_char(&self, c: char);
    fn push_c(&self, c: Self::Ch);
    fn push_u8(&self, u: u8);

    fn push_oct_value(&self, location: &dyn LocationState, octs: String);
    fn push_hex_value(&self, location: &dyn LocationState, hexs: String);

    fn get_output(&self) -> Vec<Self::Ch>;

    fn report_error(&self);
    fn seen_error(&self) -> bool;

    fn chars_consumed(&self) -> usize;
}

pub fn text_state_impl_i8<'iter>(
    iter: Peekable<Chars<'iter>>,
) -> Box<dyn TextState<Ch = i8> + 'iter> {
    i8_text::get_implementation(iter)
}

pub fn text_state_impl_i16<'iter>(
    iter: Peekable<Chars<'iter>>,
) -> Box<dyn TextState<Ch = i16> + 'iter> {
    i16_text::get_implementation(iter)
}

pub fn text_state_impl_i32<'iter>(
    iter: Peekable<Chars<'iter>>,
) -> Box<dyn TextState<Ch = i32> + 'iter> {
    i32_text::get_implementation(iter)
}