use crate::{ffi::FfiString, prelude::*};

#[repr(C)]
#[derive(Debug)]
pub struct ShardsAst {
    /// A flag that repersents if the current Ast is valid. When true treated
    /// as if the rest of None was returned. With the exceptions that the data
    /// returned must still be valid so that it can be properly freed.
    pub is_valid: bool,

    /// A pointer to the first token in a collection of tokens that make up
    /// the AST.
    pub tokens: *const ShardsToken,

    /// The number of tokens that come after the pointer, used when converting
    /// it to a rust type.
    pub number_of_tokens: usize,
}

#[no_mangle]
pub extern "C" fn shards_invalid_ast() -> ShardsAst {
    ShardsAst {
        is_valid: false,
        tokens: std::ptr::null(),
        number_of_tokens: 0,
    }
}

impl ShardsAst {
    pub fn invalid() -> Self {
        shards_invalid_ast()
    }
}

impl TryFrom<ShardsAst> for Ast {
    type Error = ParseError;

    fn try_from(value: ShardsAst) -> Result<Self, Self::Error> {
        if !value.is_valid {
            return Err(ParseError::NotValid);
        }

        let shards_tokens = unsafe {
            Vec::from_raw_parts(
                value.tokens as *mut ShardsToken,
                value.number_of_tokens,
                value.number_of_tokens,
            )
        };
        let in_len = shards_tokens.len();

        let tokens: Vec<Token> = shards_tokens
            .into_iter()
            .map(|shard| shard.try_into())
            .flatten()
            .collect();

        let out_len = tokens.len();

        if in_len > out_len {
            // these needs to be checked at the end as just returning none
            // will leak the tokens.
            drop(tokens);
            Err(ParseError::BadTokens)
        } else {
            Ok(Ast { tokens })
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub enum ShardsToken {
    Identifier(ShardsIdentifier),
    Operation(ShardsOperation),
}

impl TryFrom<ShardsToken> for Token {
    type Error = String;

    fn try_from(value: ShardsToken) -> Result<Self, Self::Error> {
        match value {
            ShardsToken::Identifier(ident) => match ident.try_into() {
                Ok(ident) => Ok(Token::Identifier(ident)),
                Err(e) => Err(e),
            },
            ShardsToken::Operation(op) => match op.try_into() {
                Ok(op) => Ok(Token::Operation(op)),
                Err(e) => Err(e),
            },
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub enum ShardsIdentifier {
    Variable(FfiString),
    Literal { value: ShardsValue },
}

impl TryFrom<ShardsIdentifier> for Identifier {
    type Error = String;

    fn try_from(value: ShardsIdentifier) -> Result<Self, Self::Error> {
        match value {
            ShardsIdentifier::Variable(ffi_string) => {
                ffi_string.try_into().map(|name| Self::Variable { name })
            }
            ShardsIdentifier::Literal { value } => {
                value.try_into().map(|value: Value| Self::Literal { value })
            }
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub enum ShardsOperation {
    // Takes the raw parts of and owned String
    ScriptCall(String),
    Add,
    Subtract,
    Multiply,
}

impl TryFrom<ShardsOperation> for Operation {
    type Error = String;

    fn try_from(value: ShardsOperation) -> Result<Self, Self::Error> {
        match value {
            ShardsOperation::ScriptCall(s) => {
                // TODO: check validity of passed values
                let name = s.try_into().unwrap();
                Ok(Self::ScriptCall { name })
            }
            _ => todo!(),
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ShardsValue {
    /// A type hint for what the data is. Can use the None value to let
    /// it be guessed
    pub variable_type: ShardsType,
    // A collection of bytes that make up the data
    pub data: Box<[u8]>,
}

impl TryFrom<ShardsValue> for Value {
    type Error = String;

    fn try_from(value: ShardsValue) -> Result<Self, Self::Error> {
        match value.variable_type {
            ShardsType::Untyped => todo!(),
            ShardsType::U32 => todo!(),
            ShardsType::U64 => todo!(),
            ShardsType::I32 => todo!(),
            ShardsType::I64 => todo!(),
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub enum ShardsType {
    /// Useful for languages that are not strongly typed
    Untyped,
    U32,
    U64,
    I32,
    I64,
    // String,
    // Array(Type),
}
