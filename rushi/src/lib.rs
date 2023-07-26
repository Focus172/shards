//! A library for parsing and manipulating Rust source code
//! into a ast a general ast.

use shards_types::Ast;

use libc::c_char;
use std::ffi::CStr;

/// Main external entry point for the library, this is used by non-rust and
/// dynamic contexts.
///
/// # Safety
/// This function dereference the pointer to string it is given. You are
/// responsible for assuming that the value pased into here is never mutated
/// as is assumed to be dropped.
#[no_mangle]
pub unsafe extern "C" fn parse(s: *const c_char) -> Ast {
    let c_str = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };

    let r_str = c_str.to_str().unwrap();
    parse_safe(r_str)
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

    Ast {
        is_valid: true,
        // string: s,
    }
}
