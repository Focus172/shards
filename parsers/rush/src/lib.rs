pub mod builtins;
pub mod helpers;
pub mod lexer;
pub mod parser;
pub mod runner;

use crate::helpers::Shell;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::runner::Runner;

use std::cell::RefCell;
use std::env;
use std::rc::Rc;

fn rush() {
    let mut args = env::args();
    args.next();

    let shell = Shell::new(args.last());
    let shell = Rc::new(RefCell::new(shell));
    let runner = Runner::new(shell.clone());

    loop {
        let input = shell.borrow_mut().next();
        let Some(line) = input else {
            if shell.borrow().is_interactive() {
                println!();
            }
            break;
        };
        let lexer = Lexer::new(&line, Rc::clone(&shell));
        let mut parser = Parser::new(lexer, Rc::clone(&shell));
        match parser.get() {
            Ok(command) => {
                #[cfg(debug_assertions)] // Only include when not built with `--release` flag
                println!("\u{001b}[34m{:#?}\u{001b}[0m", command);

                runner.execute(command, false);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}
