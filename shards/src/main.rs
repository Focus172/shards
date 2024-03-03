// mod builtins;
// mod config;
// mod env;
// mod exec;
mod cli;
mod parser;
mod pipes;
mod prelude;

use std::{
    fs::File,
    io::{stdin, BufRead},
    path::PathBuf,
};

// use crate::config::line::Line;
use crate::prelude::*;

const OPTIMIZATION_LEVEL: u8 = 3;

fn main() -> ! {
    match rushi() {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

#[tokio::main]
async fn rushi() -> Result<()> {
    let args = RushiArgs::gen();

    if args.debug {
        simplelog::WriteLogger::init(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            File::create(
                args.debug_output
                    .clone()
                    .unwrap_or_else(|| PathBuf::from("rushi.log")),
            )?,
        )
        .expect("Failed to start logger");

        log::info!("Debug mode enabled");
    }

    // setlocale(LC_ALL, "");

    let interpreter = Interpreter::new();

    // TODO: better implementation is to build config from args then env
    // from the config

    // source user and system config
    // let mut paths = ConfigPaths::new(&args);
    // paths.source(&interpreter, &mut env, &mut sys);

    // let mut env = UserState::new(&args);

    // let mut l = Line::new();
    let mut l = stdin().lock();

    println!("Welcome to Shards!");

    // let (lsp, rx) = Client::start("rust-analyzer", &[""], None, HashMap::new(), 0, "rls", 100)?;
    // lsp.initialize(true).await?;

    log::info!("Starting main event loop");

    'running: loop {
        // let line = l.next_line().unwrap();
        let mut line = String::new();
        _ = l.read_line(&mut line);

        log::info!("read line from stdin");

        let ast = Ast::parse(&line).unwrap();

        log::info!("Got some ast");

        let mut optc = OpCode::from(ast);
        for _ in 0..=OPTIMIZATION_LEVEL {
            // finds reduntant memory copyies that can be remove and still
            // garentee correctness
            optc.reduce();
        }
        let bytes = ByteCode::from(optc);

        match interpreter.eval(bytes) {
            Ok(_) => {}
            Err(_) => break 'running,
        }
    }

    // restore_term_mode();
    // restore_term_foreground_process_group_for_exit();

    Ok(())
}
