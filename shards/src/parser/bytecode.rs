use crate::prelude::*;
use std::marker::PhantomData;

pub struct ByteCode {
    operations: Vec<Thing>,
}

impl From<OpCode> for ByteCode {
    fn from(value: OpCode) -> Self {
        Self {
            operations: Vec::new(),
        }
    }
}

type Thing = PhantomData<String>;
