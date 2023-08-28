use std::fmt;

use magehand_core::BlockStatament;

use crate::{GenerateBackend, INDENT_SIZE};

impl GenerateBackend for BlockStatament {
    fn generate(&self, f: &mut impl std::fmt::Write, indent: usize) -> Result<(), fmt::Error> {
        match self {
            BlockStatament::Block(v) => {
                let width = indent * INDENT_SIZE;

                writeln!(f, "{:width$}{{", "")?;

                for i in v {
                    i.generate(f, indent + 1)?;
                }

                writeln!(f, "{:width$}}}", "")?;
            }
            BlockStatament::VarDecl(v) => v.generate(f, indent)?,
            BlockStatament::Assign(v) => v.generate(f, indent)?,
            BlockStatament::Expr(v) => v.generate(f, indent)?,
            BlockStatament::If(v) => v.generate(f, indent)?,
            BlockStatament::Switch(v) => v.generate(f, indent)?,
            BlockStatament::Loop(v) => v.generate(f, indent)?,
        }

        Ok(())
    }
}

#[cfg(test)]
mod block_tests {
    use magehand_core::{BlockStatament, Expr, Ident, If, Literal, Loop, Switch};

    use crate::{var_tests, GenerateBackend};

    #[test]
    fn test_bs() {
        let mut s = String::new();
        let vd = BlockStatament::VarDecl(var_tests::build_var_decl());
        let bs = BlockStatament::Block(vec![vd.clone(), vd]);

        bs.generate(&mut s, 0).unwrap();
        println!("{}", s);

        assert_eq!(
            "{\n  let test0 := _test(arg1, 0x0123456789abcdef)\n  let test0 := _test(arg1, 0x0123456789abcdef)\n}\n",
            s
        );
    }

    #[test]
    fn test_vd() {
        let mut s = String::new();
        let vd = BlockStatament::VarDecl(var_tests::build_var_decl());

        vd.generate(&mut s, 0).unwrap();
        println!("{}", s);

        assert_eq!("let test0 := _test(arg1, 0x0123456789abcdef)\n", s);
    }

    #[test]
    fn test_a() {
        let mut s = String::new();
        let a = BlockStatament::Assign(var_tests::build_assign());

        a.generate(&mut s, 0).unwrap();
        assert_eq!("test0 := _test(arg1, 0x0123456789abcdef)\n", s);
    }

    #[test]
    fn test_if() {
        let mut s = String::new();
        let vd = BlockStatament::VarDecl(var_tests::build_var_decl());
        let block = vec![vd.clone(), vd];

        let cond = Expr::Varable(Ident::new("arg1").unwrap());

        let i = BlockStatament::If(If { cond, block });

        i.generate(&mut s, 0).unwrap();

        println!("{}", s);

        assert_eq!("if arg1 {\n  let test0 := _test(arg1, 0x0123456789abcdef)\n  let test0 := _test(arg1, 0x0123456789abcdef)\n}\n", s);
    }

    #[test]
    fn test_switch() {
        let mut s = String::new();
        let vd = BlockStatament::VarDecl(var_tests::build_var_decl());
        let default = vec![vd.clone(), vd.clone()];

        let cond = Expr::Varable(Ident::new("arg1").unwrap());

        let lit = Literal::string("hello").unwrap();

        let cases = vec![(lit.clone(), vec![vd.clone()]), (lit, vec![vd])];

        let sw = BlockStatament::Switch(Switch {
            cond,
            cases,
            default,
        });

        sw.generate(&mut s, 0).unwrap();

        assert_eq!("switch arg1\ncase \"hello\" {\n  let test0 := _test(arg1, 0x0123456789abcdef)\n}\ncase \"hello\" {\n  let test0 := _test(arg1, 0x0123456789abcdef)\n}\ndefault  {\n  let test0 := _test(arg1, 0x0123456789abcdef)\n  let test0 := _test(arg1, 0x0123456789abcdef)\n}\n", s);
    }

    #[test]
    fn test_loop() {
        let mut s = String::new();
        let vd = BlockStatament::VarDecl(var_tests::build_var_decl());
        let ass = var_tests::build_assign();
        let default = vec![vd.clone(), vd.clone()];

        let cond = Expr::Varable(Ident::new("arg1").unwrap());

        let lp = BlockStatament::Loop(Loop {
            decl: Some(var_tests::build_var_decl()),
            cond,
            step: Some(ass),
            block: default,
        });

        lp.generate(&mut s, 0).unwrap();

        println!("{}", s);

        /* assert_eq!("switch arg1\ncase \"hello\" {\n  let test0 := _test(arg1, 0x0123456789abcdef)\n}\ncase \"hello\" {\n  let test0 := _test(arg1, 0x0123456789abcdef)\n}\ndefault  {\n  let test0 := _test(arg1, 0x0123456789abcdef)\n  let test0 := _test(arg1, 0x0123456789abcdef)\n}\n", s); */
    }
}
