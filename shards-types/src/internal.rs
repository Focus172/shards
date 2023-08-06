use crate::prelude::*;

#[repr(C)]
pub struct ShardsAst {
    /// A flag that repersents if the current Ast is valid. When true treated
    /// as if the rest of None was returned. With the exceptions that the data
    /// returned must still be valid so that it can be properly freed.
    pub is_valid: bool,

    /// The number of tokens that come after the pointer, used when converting
    /// it to a rust type.
    pub number_of_tokens: usize,

    /// A pointer to the first token in a collection of tokens that make up
    /// the AST.
    pub tokens: *const ShardsToken,
}

impl ShardsAst {
    pub fn into_ast(self) -> Option<Ast> {
        let shards_tokens = unsafe {
            Vec::from_raw_parts(
                self.tokens as *mut ShardsToken,
                self.number_of_tokens,
                self.number_of_tokens,
            )
        };
        let in_len = shards_tokens.len();

        let tokens: Vec<Token> = shards_tokens
            .iter()
            .map(|shard| shard.into_token())
            .flatten()
            .collect();

        let out_len = tokens.len();

        if !self.is_valid || in_len > out_len {
            // these needs to be checked at the end as just returning none
            // will leak the tokens.
            drop(tokens);
            return None;
        } else {
            Some(Ast { tokens })
        }
    }
}

#[repr(C)]
pub enum ShardsToken {
    Identifier(ShardsIdentifier),
    Operation(ShardsOperation),
}

impl ShardsToken {
    pub fn into_token(self) -> Option<Token> {
        match self {
            Self::Identifier(ident) => match ident.into_ident() {
                Some(ident) => Some(Token::Identifier(ident)),
                None => None,
            },
            Self::Operation(op) => match op.into_operation() {
                Some(op) => Some(Token::Operation(op)),
                None => None,
            },
        }
    }
}

#[repr(C)]
pub enum ShardsIdentifier {
    Variable {
        // name: String,
        variable_type: ShardsType,
    },
    Literal {
        value: ShardsValue,
    },
}

impl ShardsIdentifier {
    pub fn into_ident(self) -> Option<Identifier> {
        None
    }
}

#[repr(C)]
pub enum ShardsOperation {
    /// A script to call passed as its raw parts
    ScriptCall {
        ptr: *const u8,
        len: usize,
    },
    Add,
    Subtract,
    Multiply,
}

impl ShardsOperation {
    pub fn into_operation(self) -> Option<Operation> {
        None
    }
}

#[repr(C)]
pub struct ShardsValue {
    /// A type hint for what the data is. Can use the None value to let
    /// the it be guessed
    pub variable_type: ShardsType,
    // A collection of bytes that make up the data
    pub data: Box<[u8]>,
}

#[repr(C)]
pub enum ShardsType {
    /// Useful for languages that are not strongly typed
    None,
    U32,
    U64,
    I32,
    I64,
    // String,
    // Array(Type),
}
