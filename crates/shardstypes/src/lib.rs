//! Type for working with the shards framework. The way it strange serialization
//! pattern. Your end goal is to provide an ShardsAst as a return to your cdylib.
//! this can be done in what ever way you want but this library is provided as
//! a way to make the easier.
//!
//! When using rust it is recomended to construct it using the Ast type an
//! then convert it to the other. Other languages can either use the interface
//! directly or to use a package that abstracts this interface.
#![deny(
    // missing_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    unused_imports,
    dead_code,
    unused_crate_dependencies
)]

/// The idomatic rust abstraction over the raw ast
pub mod internal;

/// The raw bindings to the expected ast format
pub mod external;

/// A set of imports that may be usefult when working with this library
pub mod prelude;

pub(crate) mod ffi;

pub type ParseFuncSig = fn(*const u8, usize) -> external::ShardsAst;
