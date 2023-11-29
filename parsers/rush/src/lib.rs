pub mod helpers;
pub mod lexer;
pub mod parser;

use parser::Cmd;

use crate::helpers::Shell;
use crate::lexer::Lexer;
use crate::parser::Parser;

use std::cell::RefCell;
use std::rc::Rc;

pub fn rush(input: String) -> Result<Cmd, String> {
    let shell = Rc::new(RefCell::new(Shell::new()));

    let lexer = Lexer::new(&input, shell.clone());
    let mut parser = Parser::new(lexer, shell.clone());
    parser.get()
}
