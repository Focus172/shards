pub struct EnvState {
    pub prompt: String,
    pub rhs: String,
}

impl Default for EnvState {
    fn default() -> Self {
        Self {
            prompt: String::from(">> "),
            rhs: String::from(" "),
        }
    }
}

// pub mod environment;
// pub mod var;
//
// pub use environment::*;
// pub use var::*;
