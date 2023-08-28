use serde::{Deserialize, Serialize};

use crate::{Ident, Literal};

#[derive(Debug, Serialize, Deserialize)]
pub struct Func {
    pub name: Ident,
    pub args: Vec<Ident>,
    pub ret: Ident,
    pub block: Vec<BlockStatament>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BlockStatament {
    Block(Vec<BlockStatament>),
    VarDecl(VarDecl),
    Assign(Assign),
    Expr(Expr),
    If(If),
    Loop(Loop),
    Switch(Switch),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VarDecl {
    pub name: Ident,
    pub expr: Expr,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assign {
    pub name: Ident,
    pub expr: Expr,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Expr {
    Literal(Literal),
    Varable(Ident),
    FunctionCall(Ident, Vec<Expr>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct If {
    pub cond: Expr,
    pub block: Vec<BlockStatament>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Loop {
    pub decl: VarDecl,
    pub cond: Expr,
    pub step: Assign,
    pub block: Vec<BlockStatament>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Switch {
    pub cond: Expr,
    pub cases: Vec<(Literal, Vec<BlockStatament>)>,
    pub default: Vec<BlockStatament>,
}

#[cfg(test)]
mod test {}
