//! Stardust's piping features. 
//! defaults to unix pipes when not avalable.

mod streams;
mod fds;

pub use fds::open_cloexec;
