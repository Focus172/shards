use crate::external::{ShardsAst, ShardsIdentifier, ShardsOperation, ShardsToken, ShardsValue};

#[derive(Debug)]
pub struct Ast {
    /// A pointer to the first token in a collection of tokens that make up
    /// the AST.
    pub tokens: Vec<Token>,
}

impl From<Ast> for ShardsAst {
    /// Turns a valid ast into a an ast that can pass properly pass through
    /// library boundieres.
    ///
    /// # Safety
    /// Leaks the tokens of this ast. The person who recives the ast after this
    /// is responsible for freeing the memory.
    fn from(value: Ast) -> Self {
        let data = value
            .tokens
            .iter()
            .cloned()
            .map(Into::<ShardsToken>::into)
            .collect::<Vec<ShardsToken>>()
            .leak();
        ShardsAst {
            is_valid: true,
            number_of_tokens: data.len(),
            tokens: data.as_ptr(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Identifier(Identifier),
    Operation(Operation),
}

impl From<Token> for ShardsToken {
    fn from(value: Token) -> Self {
        match value {
            Token::Identifier(ident) => Self::Identifier(ident.into()),
            Token::Operation(op) => Self::Operation(op.into()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Identifier {
    Variable { name: String },
    Literal { value: Value },
}

impl From<Identifier> for ShardsIdentifier {
    fn from(value: Identifier) -> Self {
        match value {
            Identifier::Variable { name } => ShardsIdentifier::Variable(name.into()),
            Identifier::Literal { value } => ShardsIdentifier::Literal {
                value: value.into(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operation {
    /// A script to call passed as its raw parts
    ScriptCall {
        name: String, /* args: Vec<String> */
    },
    Add,
    Subtract,
    Multiply,
}

impl From<Operation> for ShardsOperation {
    fn from(value: Operation) -> Self {
        match value {
            Operation::ScriptCall { mut name } => {
                name.shrink_to_fit();
                let slice = name.leak();
                Self::ScriptCall(crate::ffi::FfiString {
                    ptr: slice.as_mut_ptr(),
                    len: slice.len(),
                })
            }
            Operation::Add => Self::Add,
            Operation::Subtract => Self::Subtract,
            Operation::Multiply => Self::Multiply,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    /// Useful for languages that are not strongly typed
    Untyped(Box<[u8]>),
    U32(u32),
    U64(u64),
    I32(i32),
    I64(i64),
    // String,
    // Array(Type),
}

impl From<Value> for ShardsValue {
    fn from(_value: Value) -> Self {
        todo!()
    }
}
