use std::fmt::Write;

use magehand_core::{Ident, Literal};

use crate::GenerateBackend;

impl GenerateBackend for Ident {
    fn generate(&self, f: &mut impl Write) -> Result<(), std::fmt::Error> {
        f.write_str(&self.0)
    }
}

impl GenerateBackend for Literal {
    fn generate(&self, f: &mut impl Write) -> Result<(), std::fmt::Error> {
        match self {
            Literal::Hex(s) => write!(f, "0x{}", s),
            Literal::Int(s) => write!(f, "{}", s),
            Literal::String(s) => write!(f, "\"{}\"", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use magehand_core::{Ident, Literal};

    use crate::GenerateBackend;

    #[test]
    fn test_ident() {
        let mut s = String::new();
        let id = Ident::new("_hello").unwrap();
        id.generate(&mut s).unwrap();

        assert_eq!("_hello", s);
    }

    #[test]
    fn test_literal() {
        assert!(Literal::hex("fetfrtt").is_err());

        let mut s = String::new();
        let l = Literal::hex("0123456789abcdef").unwrap();
        l.generate(&mut s).unwrap();
        assert_eq!("0x0123456789abcdef", s);

        let mut s = String::new();
        let l = Literal::int("1234567890").unwrap();
        l.generate(&mut s).unwrap();
        assert_eq!("1234567890", s);

        let mut s = String::new();
        let l = Literal::string("1你好234567890").unwrap();
        l.generate(&mut s).unwrap();
        assert_eq!("\"1你好234567890\"", s);
    }
}
