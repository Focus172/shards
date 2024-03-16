#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct ShardsValue;

struct FfiString {
  uint8_t *ptr;
  uintptr_t len;
  uintptr_t cap;
};

struct ShardsIdentifier {
  enum class Tag {
    Variable,
    Literal,
  };

  struct Variable_Body {
    FfiString name;
  };

  struct Literal_Body {
    ShardsValue val;
  };

  Tag tag;
  union {
    Variable_Body variable;
    Literal_Body literal;
  };
};

struct ShardsOperation {
  enum class Tag {
    ScriptCall,
    Add,
    Subtract,
    Multiply,
  };

  struct ScriptCall_Body {
    FfiString _0;
  };

  Tag tag;
  union {
    ScriptCall_Body script_call;
  };
};

struct ShardsToken {
  enum class Tag {
    Identifier,
    Operation,
  };

  struct Identifier_Body {
    ShardsIdentifier _0;
  };

  struct Operation_Body {
    ShardsOperation _0;
  };

  Tag tag;
  union {
    Identifier_Body identifier;
    Operation_Body operation;
  };
};

struct ShardsAst {
  /// A flag that repersents if the current Ast is valid. When true treated
  /// as if the rest of None was returned. With the exceptions that the data
  /// returned must still be valid so that it can be properly freed.
  bool is_valid;
  /// A pointer to the first token in a collection of tokens that make up
  /// the AST.
  ShardsToken *tokens_pointer;
  /// The number of tokens that come after the pointer, used when converting
  /// it to a rust type.
  uintptr_t tokens_count;
};

extern "C" {

/// Used to construct an invalid Ast from external code.
///
/// # Safety
/// If you are using Rust use [`ShardsAst::invalid`] instead.
ShardsAst shards_invalid_ast();

} // extern "C"
