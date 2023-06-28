//! A library for parsing and manipulating Rust source code 
//! into a ast a general ast. 

/// main entry point for the library
pub fn ast_from_str(s: &str) -> Option<syn::File> {

    let syntax = syn::parse_file(s).expect("Unable to parse file");
    // let Some(shebang) = syntax.shebang else {
    //     panic!("No shebang found");
    // };

    // Debug impl is available if Syn is built with "extra-traits" feature.
    println!("{:#?}", syntax);

    Some(syntax)
}
