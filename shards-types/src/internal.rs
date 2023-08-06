use crate::external::{Ast, Token};

#[repr(C)]
#[derive(Debug)]
pub struct ShardsAst {
    /// a flag that repersents if the current as is valid. Use this
    /// in place of returning none
    pub is_valid: bool,

    /// The number of tokens that come after the pointer, used when converting
    /// it to a rust type.
    pub number_of_tokens: usize,

    /// A pointer to the first token in a collection of tokens that make up
    /// the AST.
    pub tokens: *const Token,
}

impl ShardsAst {
    pub fn into_ast(self) -> Option<Ast> {
        if !self.is_valid {
            return None;
        }
        let tokens = unsafe {
            Vec::from_raw_parts(
                self.tokens as *mut Token,
                self.number_of_tokens,
                self.number_of_tokens,
            )
        };
        Some(Ast { tokens })
    }
}
