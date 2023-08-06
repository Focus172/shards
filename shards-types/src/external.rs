use crate::internal::ShardsAst;

#[derive(Debug)]
pub struct Ast {
    /// A pointer to the first token in a collection of tokens that make up
    /// the AST.
    pub tokens: Vec<Token>,
}

impl Ast {
    pub fn into_shards_ast(self) -> ShardsAst {
        let data = self.tokens.leak();
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

#[derive(Debug)]
pub enum Operation {
    ScriptCall,
    Add,
    Subtract,
    Multiply,
}
