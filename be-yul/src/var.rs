use std::fmt::Write;

use magehand_core::VarDecl;

use crate::GenerateBackend;

impl GenerateBackend for VarDecl {
    fn generate(&self, f: &mut impl Write) -> Result<(), std::fmt::Error> {
        write!(f, "let")?;
        self.name.generate(f)?;
        write!(f, ":=")
    }
}
