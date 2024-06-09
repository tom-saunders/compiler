#[derive(Debug, PartialEq, Clone)]
pub struct LocatedToken {
    location: NestedLocation,
    token: Token,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NestedLocation {
    locations: Vec<Location>,
}

impl NestedLocation {
    fn get_chain(&self) -> &Vec<Location> {
        todo!()
    }

    fn get_top(&self) -> &Location {
        todo!()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Location {
    line: usize,
    col: usize,
    file: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {}

pub fn lex(_input: &str) -> Result<Vec<LocatedToken>, ()> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_test() {
        todo!()
    }
}
