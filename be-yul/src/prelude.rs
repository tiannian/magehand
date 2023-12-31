use std::fmt::{Error, Write};

pub trait GenerateBackend {
    fn generate(&self, f: &mut impl Write, indent: usize) -> Result<(), Error>;
}
