use crate::prelude::*;

pub struct ByteCode {
    // operations: Vec<Thing>,
}

impl From<OpCode> for ByteCode {
    fn from(_value: OpCode) -> Self {
        Self {
            // operations: Vec::new(),
        }
    }
}

// use std::marker::PhantomData;
// type Thing = PhantomData<String>;
