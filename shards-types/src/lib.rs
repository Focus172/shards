//! Type for working with the shards framework. The way it strange serialization
//! pattern. Your end goal is to provide an ShardsAst as a return to your cdylib.
//! this can be done in what ever way you want but this library is provided as
//! a way to make the easier.
//!
//! When using rust it is recomended to construct it using the Ast type an
//! then convert it to the other. Other languages can either use the interface
//! directly or to use a package that abstracts this interface.

/// The raw bindings to the expected ast format
pub mod external;

/// The idomatic rust abstraction over the raw ast
pub mod internal;

/// A short set of imports that may be usefult when working with this library
pub mod prelude;
