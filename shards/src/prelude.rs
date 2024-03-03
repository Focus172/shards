// Rexports of the common untilites of the crate
pub use crate::parser::*;
pub use libshards::prelude::*;
pub use crate::cli::RushiArgs;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO: {0}")]
    IO(std::io::Error),
    #[error(transparent)]
    Static(anyhow::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Self::Static(e)
    }
}

// impl From<log::SetLoggerError> for Error {
//     fn from(e: log::SetLoggerError) -> Self {
//         Self::Static(e.into())
//     }
// }
