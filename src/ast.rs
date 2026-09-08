//! この言語の値と構文木。構文木には、まだ評価していない式を保持します。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Int(i64),
    Bool(bool),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Prim {
    Add,
    Sub,
    Mul,
    Lt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Exp {
    Literal(Value),
    Binary(Box<Exp>, Prim, Box<Exp>),
    If(Box<Exp>, Box<Exp>, Box<Exp>),
}
