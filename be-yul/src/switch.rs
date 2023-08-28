use magehand_core::Switch;

use crate::{GenerateBackend, INDENT_SIZE};

impl GenerateBackend for Switch {
    fn generate(&self, f: &mut impl std::fmt::Write, indent: usize) -> Result<(), std::fmt::Error> {
        let width = indent * INDENT_SIZE;
        write!(f, "{:width$}switch ", "")?;
        self.cond.generate(f, indent)?;
        writeln!(f)?;

        for (lit, bs) in &self.cases {
            write!(f, "{:width$}case ", "")?;
            lit.generate(f, indent)?;

            writeln!(f, " {{")?;

            for i in bs {
                i.generate(f, indent + 1)?;
            }

            writeln!(f, "{:width$}}}", "")?;
        }

        write!(f, "{:width$}default ", "")?;
        writeln!(f, " {{")?;

        for i in &self.default {
            i.generate(f, indent + 1)?;
        }

        writeln!(f, "{:width$}}}", "")?;

        Ok(())
    }
}
