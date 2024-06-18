use crate::LocationState;

pub struct TestLocation;
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