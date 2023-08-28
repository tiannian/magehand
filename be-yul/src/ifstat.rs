use magehand_core::If;

use crate::{GenerateBackend, INDENT_SIZE};

impl GenerateBackend for If {
    fn generate(&self, f: &mut impl std::fmt::Write, indent: usize) -> Result<(), std::fmt::Error> {
        let width = indent * INDENT_SIZE;
        write!(f, "{:width$}if ", "")?;
        self.cond.generate(f, indent)?;
        writeln!(f, " {{")?;

        for i in &self.block {
            i.generate(f, indent + 1)?;
        }

        write!(f, "{:width$}}}", "")?;

        Ok(())
    }
}
