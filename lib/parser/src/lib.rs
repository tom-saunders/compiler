use lexer::{self, LocatedToken};

pub type AbstractSyntaxTree = ();

pub fn parse(_tokens: &[LocatedToken]) -> Result<AbstractSyntaxTree, ()> {
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
