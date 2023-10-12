use std::error::Error;
use std::fmt::Display;

pub use crate::external::ShardsAst;
pub use crate::internal::*;
pub use crate::ParseFuncSig;

#[derive(Debug)]
pub enum ParseError {
    NotValid,
    BadTokens,
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::NotValid => write!(f, "Invalid trees cannot be parsed to rust trees"),
            ParseError::BadTokens => write!(
                f,
                "Not all tokens in the tree could be parsed to valid tokens"
            ),
        }
    }
}
