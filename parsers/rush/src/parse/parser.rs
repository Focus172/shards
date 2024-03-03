//! Parser takes in a token steam and outputs commands.

use super::{Cmd, CmdError};
use crate::prelude::*;

/// The parser reads in tokens and converts them into commands.
pub struct Parser<I>
where
    I: Iterator<Item = Token>,
{
    tokens: I,
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    pub fn new(tokens: I) -> Parser<I> {
        Parser { tokens }
    }
}

impl<I> Iterator for Parser<I>
where
    I: Iterator<Item = Token>,
{
    type Item = Result<Cmd, CmdError>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.tokens.next()?;
        dbg!(&token);
        todo!()
    }
}
