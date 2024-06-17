
mod i16;
mod i32;
mod i8;

use crate::LocationState;

use super::char_literal_impl;

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
