use parser::{self, AbstractSyntaxTree};

pub type Generated = ();

pub fn generate<'a>(_ast: &AbstractSyntaxTree) -> Result<Generated, ()> {
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
