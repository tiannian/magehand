use std::fmt::Write;

use magehand_core::{Assign, VarDecl};

use crate::{GenerateBackend, INDENT_SIZE};

impl GenerateBackend for VarDecl {
    fn generate(&self, f: &mut impl Write, indent: usize) -> Result<(), std::fmt::Error> {
        let width = indent * INDENT_SIZE;

        write!(f, "{:width$}let ", "")?;
        self.name.generate(f, indent)?;
        write!(f, " := ")?;
        self.expr.generate(f, indent)
    }
}

impl GenerateBackend for Assign {
    fn generate(&self, f: &mut impl Write, indent: usize) -> Result<(), std::fmt::Error> {
        let width = indent * INDENT_SIZE;

        write!(f, "{:width$}", "")?;
        self.name.generate(f, indent)?;
        write!(f, " := ")?;
        self.expr.generate(f, indent)
    }
}

#[cfg(test)]
pub mod var_tests {
    use magehand_core::{Assign, Expr, Ident, Literal, VarDecl};

    use crate::GenerateBackend;

    pub fn build_expr() -> Expr {
        let e0 = Expr::Literal(Literal::hex("0123456789abcdef").unwrap());

        let i = Expr::Varable(Ident::new("arg1").unwrap());
        let args = vec![i, e0];
        Expr::FunctionCall(Ident::new("_test").unwrap(), args)
    }

    pub fn build_var_decl() -> VarDecl {
        let expr = build_expr();
        VarDecl {
            name: Ident::new("test0").unwrap(),
            expr,
        }
    }

    pub fn build_assign() -> Assign {
        let expr = build_expr();

        Assign {
            name: Ident::new("test0").unwrap(),
            expr,
        }
    }

    #[test]
    fn test_var_decl() {
        let mut s = String::new();

        let v = build_var_decl();
        v.generate(&mut s, 0).unwrap();

        assert_eq!("let test0 := _test(arg1, 0x0123456789abcdef)", s);
    }

    #[test]
    fn test_assign() {
        let mut s = String::new();

        let a = build_assign();
        a.generate(&mut s, 0).unwrap();

        assert_eq!("test0 := _test(arg1, 0x0123456789abcdef)", s);
    }
}
