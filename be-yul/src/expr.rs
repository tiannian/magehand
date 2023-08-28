use std::fmt::Write;

use magehand_core::Expr;

use crate::GenerateBackend;

impl GenerateBackend for Expr {
    fn generate(&self, f: &mut impl Write) -> Result<(), std::fmt::Error> {
        match self {
            Expr::Literal(s) => s.generate(f),
            Expr::Varable(s) => s.generate(f),
            Expr::FunctionCall(name, value) => {
                name.generate(f)?;
                write!(f, "(")?;
                for (i, v) in value.iter().enumerate() {
                    if i != 0 {
                        write!(f, ",")?;
                    }
                    v.generate(f)?;
                }
                write!(f, ")")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use magehand_core::{Expr, Ident, Literal};

    use crate::GenerateBackend;

    #[test]
    fn test() {
        let mut s = String::new();
        let e0 = Expr::Literal(Literal::hex("0123456789abcdef").unwrap());
        e0.generate(&mut s).unwrap();
        assert_eq!("0x0123456789abcdef", s);

        let mut s = String::new();
        let i = Expr::Varable(Ident::new("arg1").unwrap());
        let args = vec![i, e0];
        let e = Expr::FunctionCall(Ident::new("_test").unwrap(), args);
        e.generate(&mut s).unwrap();
        assert_eq!("_test(arg1,0x0123456789abcdef)", s);

        let mut s = String::new();
        let args = vec![];
        let e = Expr::FunctionCall(Ident::new("_test").unwrap(), args);
        e.generate(&mut s).unwrap();
        assert_eq!("_test()", s);
    }
}
