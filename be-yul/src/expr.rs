use magehand_core::Expr;

use crate::GenerateBackend;

impl GenerateBackend for Expr {
    fn generate(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Expr::Literal(s) => s.generate(f),
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
    use magehand_core::{Expr, Literal};

    #[test]
    fn test() {
        let e0 = Expr::Literal(Literal::Hex("0123456789abcdef"));
        println!("{}", e0.generate)
    }
}
