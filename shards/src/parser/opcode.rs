//! the main logic of the starship thing
//! the two main points of interest anre the from meathod that walks the ast
//! and the reduce meathod that removes unessisary memory copies

use crate::prelude::*;

pub struct OpCode {
    ops: Vec<Operation>,
}

impl From<Ast> for OpCode {
    fn from(ast: Ast) -> Self {
        dbg!(ast);
        OpCode { ops: Vec::new() }
    }
}

impl OpCode {
    pub fn reduce(&mut self) {}
}

pub struct Operation {
    command: Command,
    arg1: Argument,
    arg2: Option<Argument>,
}

enum Command {
    Add,
    Subtract,
}

enum Argument {
    Number(f64),
    String(String),
    Bool(bool),
    Table(Table),
}

struct Table {
    members: Vec<Value>,
}

enum Value {
    KeyValue(Pair),
    Array(Vec<Argument>),
    Literal(Argument),
}

struct Pair {
    key: Argument,
    value: Argument,
}
