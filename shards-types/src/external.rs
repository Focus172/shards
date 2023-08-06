use crate::internal::{ShardsAst, ShardsIdentifier, ShardsOperation, ShardsToken};

#[derive(Debug)]
pub struct Ast {
    /// A pointer to the first token in a collection of tokens that make up
    /// the AST.
    pub tokens: Vec<Token>,
}

impl Ast {
    /// Turns a valid ast into a an ast that can pass properly pass through
    /// library boundieres.
    ///
    /// # Safety
    /// Leaks the tokens of this ast. The person who recives the ast after this
    /// is responsible for freeing the memory.
    pub unsafe fn into_shards_ast(self) -> ShardsAst {
        let data = self
            .tokens
            .iter()
            .map(|tok| tok.into())
            .collect::<Vec<ShardsToken>>()
            .leak();
        ShardsAst {
            is_valid: true,
            number_of_tokens: data.len(),
            tokens: data.as_ptr(),
        }
    }
}

#[derive(Debug)]
pub enum Token {
    Identifier(Identifier),
    Operation(Operation),
}

impl Into<ShardsToken> for Token {
    fn into(self) -> ShardsToken {
        match self {
            Self::Identifier(ident) => ShardsToken::Identifier(ident.into()),
            Self::Operation(op) => ShardsToken::Operation(op.into()),
        }
    }
}

#[derive(Debug)]
pub enum Identifier {
    Variable {
        // name: String,
        variable_type: Type,
    },
    Literal {
        value: Value,
    },
}

impl Into<ShardsIdentifier> for Identifier {
    fn into(self) -> ShardsIdentifier {
        match self {
            Self::Variable { variable_type } => ShardsIdentifier::Variable {
                variable_type: variable_type.into(),
            },
            Self::Literal { value } => ShardsIdentifier::Literal {
                value: value.into(),
            },
        }
    }
}

#[derive(Debug)]
pub enum Operation {
    /// A script to call passed as its raw parts
    ScriptCall {
        name: String, /* args: Vec<String> */
    },
    Add,
    Subtract,
    Multiply,
}

impl Into<ShardsOperation> for Operation {
    fn into(self) -> ShardsOperation {
        match self {
            Self::ScriptCall { name } => {
                let data = name.leak();
                ShardsOperation::ScriptCall {
                    ptr: data.as_ptr(),
                    len: data.len(),
                }
            }
            Self::Add => ShardsOperation::Add,
            Self::Subtract => ShardsOperation::Subtract,
            Self::Multiply => ShardsOperation::Multiply,
        }
    }
}

#[derive(Debug)]
pub struct Value {
    /// A type hint for what the data is. Can use the None value to let
    /// the it be guessed
    pub variable_type: Type,
    // A collection of bytes that make up the data
    pub data: Box<[u8]>,
}

#[derive(Debug)]
pub enum Type {
    /// Useful for languages that are not strongly typed
    None,
    U32,
    U64,
    I32,
    I64,
    // String,
    // Array(Type),
}
