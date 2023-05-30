pub mod client;
mod transport;
mod util;
mod jsonrpc;
mod snippet;

pub use client::Client;
pub use futures_executor::block_on;
pub use jsonrpc::Call;
pub use lsp_types as lsp;
pub use lsp::{Position, Url};
use futures_util::stream::select_all::SelectAll;
use tokio::sync::mpsc::UnboundedReceiver;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use thiserror::Error;
use tokio_stream::wrappers::UnboundedReceiverStream;

pub type Result<T> = core::result::Result<T, Error>;
pub type LanguageServerName = String;

#[derive(Error, Debug)]
pub enum Error {
    #[error("protocol error: {0}")]
    Rpc(#[from] jsonrpc::Error),
    #[error("failed to parse: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),
    #[error("request {0} timed out")]
    Timeout(jsonrpc::Id),
    #[error("server closed the stream")]
    StreamClosed,
    #[error("Unhandled")]
    Unhandled,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OffsetEncoding {
    /// UTF-8 code units aka bytes
    Utf8,
    /// UTF-32 code units aka chars
    Utf32,
    /// UTF-16 code units
    #[default]
    Utf16,
}

