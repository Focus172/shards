#![feature(vec_into_raw_parts)]

use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    NotValid,
    BadTokens,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::NotValid => write!(f, "Invalid trees cannot be parsed to rust trees"),
            ParseError::BadTokens => write!(
                f,
                "Not all tokens in the tree could be parsed to valid tokens"
            ),
        }
    }
}
impl resu::Context for ParseError {}

#[repr(C)]
#[derive(Debug)]
pub struct ShardsAst {
    /// A flag that repersents if the current Ast is valid. When true treated
    /// as if the rest of None was returned. With the exceptions that the data
    /// returned must still be valid so that it can be properly freed.
    pub is_valid: bool,

    /// A pointer to the first token in a collection of tokens that make up
    /// the AST.
    pub tokens_pointer: *mut ShardsToken,

    /// The number of tokens that come after the pointer, used when converting
    /// it to a rust type.
    pub tokens_count: usize,
}

/// Used to construct an invalid Ast from external code.
///
/// # Safety
/// If you are using Rust do `None.into()` instead.
#[no_mangle]
pub unsafe extern "C" fn shards_invalid_ast() -> ShardsAst {
    ShardsAst {
        is_valid: false,
        tokens_pointer: std::ptr::null_mut(),
        tokens_count: 0,
    }
}

#[repr(C)]
#[derive(Debug)]
pub enum ShardsToken {
    Identifier(ShardsIdentifier),
    Operation(ShardsOperation),
}

#[repr(C)]
#[derive(Debug)]
pub enum ShardsIdentifier {
    Variable { name: FfiString },
    Literal { val: ShardsValue },
}

#[repr(C)]
#[derive(Debug)]
pub enum ShardsOperation {
    // Takes the raw parts of an owned String
    ScriptCall(FfiString),
    Add,
    Subtract,
    Multiply,
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

#[repr(C)]
#[derive(Debug)]
pub struct FfiString {
    pub ptr: *mut u8,
    pub len: usize,
    pub cap: usize,
}

impl From<String> for FfiString {
    fn from(value: String) -> Self {
        let (ptr, len, cap) = value.into_raw_parts();
        Self { ptr, len, cap }
    }
}

impl TryFrom<FfiString> for String {
    type Error = resu::Report<FfiStringError>;

    fn try_from(value: FfiString) -> Result<Self, Self::Error> {
        if value.ptr.is_null() {
            return Err(resu::Report::new(FfiStringError::Null));
        }

        Ok(unsafe { String::from_raw_parts(value.ptr, value.len, value.cap) })
    }
}

impl Drop for FfiString {
    fn drop(&mut self) {
        log::error!("FfiString [`Drop`]ed while value leaked. This is likely a memory leak.")
    }
}

#[derive(Debug)]
pub enum FfiStringError {
    Null,
}
impl fmt::Display for FfiStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FfiStringError::Null => f.write_str("pointer was null"),
        }
    }
}
impl resu::Context for FfiStringError {}
