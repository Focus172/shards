// mod builtins;
// mod config;
// mod env;
// mod exec;
// mod pipes;

mod cli;
mod line;
mod parser;
mod prelude;

use std::{fs::File, io, path::PathBuf};

use crate::prelude::*;

const OPTIMIZATION_LEVEL: u8 = 3;

fn main() -> std::process::ExitCode {
    let args = RushiArgs::gen();

    args.debug.then(|| logger(args.debug_file));

    // unsafe { libc::setlocale(libc::LC_ALL, b"\0".as_ptr() as *const i8) };
    // setlocale(LC_ALL, "");

    let interpreter = match Interpreter::new(Lang::Rust) {
        Ok(i) => i,
        Err(e) => {
            eprintln!(
                "{:?}",
                e.attach_printable("failed to start shards. Do you have any langs intalled?")
            );
            return std::process::ExitCode::FAILURE;
        }
    };

    // let mut env = UserState::new(&args);

    // source user and system config
    // let mut paths = ConfigPaths::new(&args);
    // paths.source(&interpreter, &mut env, &mut sys);

    eprintln!("Welcome to Shards!");

    // let (lsp, rx) = Client::start("rust-analyzer", &[""], None, HashMap::new(), 0, "rls", 100)?;
    // lsp.initialize(true).await?;

    log::info!("Starting main event loop");
    let fatal = true;

    while let Some(line) = crate::line::next() {
        match parse(&interpreter, line) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{e:?}");
                if fatal {
                    return std::process::ExitCode::FAILURE;
                }
            }
        }
    }
    // restore_term_foreground_process_group_for_exit();

    std::process::ExitCode::SUCCESS
}

fn logger(path: Option<PathBuf>) -> io::Result<()> {
    // let _ = simplelog::WriteLogger::init(
    //     simplelog::LevelFilter::Info,
    //     simplelog::Config::default(),
    //     File::create(path.clone().unwrap_or_else(|| PathBuf::from("rushi.log")))?,
    // );

    log::info!("Debug mode enabled");

    Ok(())
}

#[derive(Debug)]
pub enum ShardsError {
    Ast,
    Run,
}
impl fmt::Display for ShardsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShardsError::Ast => f.write_str("failed to get a failed ast"),
            ShardsError::Run => f.write_str("failed to run the commnand"),
        }
    }
}
impl Context for ShardsError {}

fn parse(inter: &Interpreter, input: String) -> Result<(), ShardsError> {
    log::info!("read line from stdin");

    let ast = inter
        .loader
        .parse(&input)
        .ok_or(Report::new(ShardsError::Ast))?;
    log::info!("Got some ast");

    let mut optc = OpCode::from(ast);

    for _ in 0..=OPTIMIZATION_LEVEL {
        optc.reduce();
    }
    let bytes = ByteCode::from(optc);

    inter.eval(bytes).change_context(ShardsError::Run)?;

    Ok(())
}

// fn test() {
//     use tree_sitter::{Language, Parser};
//
//     let mut parser = Parser::new();
//
//     parser
//         .set_language(tree_sitter_rust::language())
//         .expect("Error loading Rust grammar");
//
//     let source_code = "fn test() {}";
//     let tree = parser.parse(source_code, None).unwrap();
//     // let mut walk = tree.walk();
//     let root_node = tree.root_node();
//     let mut i = 0;
//     while let Some(node) = root_node.named_child(i) {
//         let mut j = 0;
//         while let Some(n) = node.named_child(j) {
//             let a = n.range();
//             let s = &source_code[a.start_byte..a.end_byte];
//             dbg!(s);
//             dbg!(n.to_sexp());
//             dbg!(n.kind());
//             dbg!(n);
//             j += 1;
//         }
//         i += 1;
//     }
//
//     // dbg!(root_node.to_sexp());
//     // // let mut walk = root_node.walk();
//     // let children = root_node.children(&mut walk);
//     // for n in children {
//     //     dbg!(n.to_sexp());
//     //     dbg!(n.kind());
//     //     let mut walk = n.walk();
//     //     for c in n.named_children(&mut walk) {
//     //         // let mut walk = c.walk();
//     //         // dbg!(walk.field_name());
//     //         dbg!(c.to_sexp());
//     //     }
//     // }
//     // dbg!(&root_node);
//     // assert!(!root_node.has_error());
//     // let a = root_node.walk();
//     // dbg!(a.field_name());
//     // // root_node.language();
//     //
//     // assert_eq!(root_node.kind(), "source_file");
//     // assert_eq!(root_node.start_position().column, 0);
//     // assert_eq!(root_node.end_position().column, 12);
// }
