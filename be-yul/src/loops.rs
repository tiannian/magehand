use magehand_core::Loop;

use crate::{GenerateBackend, INDENT_SIZE};

impl GenerateBackend for Loop {
    fn generate(&self, f: &mut impl std::fmt::Write, indent: usize) -> Result<(), std::fmt::Error> {
        let width = indent * INDENT_SIZE;
        write!(f, "{:width$}for {{ ", "")?;

        if let Some(v) = &self.decl {
            v.generate(f, 0)?;
        }

        write!(f, " }} ")?;

        self.cond.generate(f, 0)?;

        write!(f, " {{ ")?;

        if let Some(v) = &self.step {
            v.generate(f, 0)?;
        }

        write!(f, " }} ")?;

        writeln!(f, " {{")?;

        for i in &self.block {
            i.generate(f, indent + 1)?;
        }

        writeln!(f, "{:width$}}}", "")?;

        Ok(())
    }
}
