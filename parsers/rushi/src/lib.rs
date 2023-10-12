//! A library for parsing and manipulating Rust source code
//! into a ast a general ast.

use libshards::prelude::*;

/// Main external entry point for the library, this is used by non-rust and
/// dynamic contexts.
///
/// # Safety
/// This function dereference the pointer to string it is given. You are
/// responsible for assuming that the value pased into here is never mutated
/// as is assumed to be dropped.
#[no_mangle]
pub unsafe extern fn parse(c: *const u8, len: usize) -> ShardsAst {
    let Ok(s) = std::str::from_utf8(std::slice::from_raw_parts(c, len)) else {
        eprintln!("Failed to parse string");
        return ShardsAst::invalid();
    };

    parse_safe(s).into()
}

/// Main entry point to the function when working in safe-rust. This is for if
/// you decide to include the library naticly or somthing.
pub fn parse_safe(s: &str) -> Ast {
    println!("parsing: {}", s);

    // let syntax = syn::parse_file(&s).expect("Unable to parse file");
    // let Some(shebang) = syntax.shebang else {
    //     panic!("No shebang found");
    // };

    // Debug impl is available if Syn is built with "extra-traits" feature.
    // println!("{:#?}", syntax);

    // this needs to leak otherwise it is a segfault
    // Token::Identifier(Identifier::Literal { value: todo!()}),

    let mut tokens = vec![
        Token::Operation(Operation::ScriptCall { name: String::from("ls") }),
        // Token::Operation(Operation::ScriptCall("ls".into())),
    ];

    tokens.shrink_to_fit();

    Ast { tokens }
}
