// mod builtins;
mod config;
mod env;
mod parser;
mod pipes;
// mod exec;
mod prelude;

use std::{
    fs::File, path::PathBuf
};

use crate::config::line::Line;
use crate::prelude::*;

const OPTIMIZATION_LEVEL: u8 = 3;


use clap::Parser;
/// A Shell to oxidize your terminal
#[derive(Parser, Debug)]
#[clap(
    name = "Rushi",
    author = "Focus172",
    version = "0.1.0",
    about = "A Shell to oxidize your terminal",
    long_about = None,
)]
pub struct RushiArgs {
    /// Enables debug mode
    #[arg(short = 'd', long, default_value_t = false)]
    debug: bool,

    /// File to use for debug output
    #[arg(short = 'o', long, value_name = "PATH", value_hint = clap::ValueHint::FilePath)]
    debug_output: Option<PathBuf>,

    /// Commands to be executed in place of interactive shell.
    #[arg(short = 'c', long = "command", value_name = "COMMAND")]
    batch_cmds: Option<Vec<String>>,

    /// Commands to execute after the shell's config has been read.
    #[arg(short = 'C', long = "init-command", value_name = "COMMAND")]
    postconfig_cmds: Option<Vec<String>>,

    /// Whether no-config is set. default is false.
    #[arg(short = 'N', long, default_value_t = false)]
    no_config: bool,

    /// Custom config file to use. default is ~/.config/rushi/config.rsi
    #[arg(short = 'p', long, value_name = "PATH", value_hint = clap::ValueHint::FilePath)]
    custom_config: Option<PathBuf>,

    /// Whether no-exec is set.
    #[arg(short = 'n', long, default_value_t = false)]
    no_execute: bool,

    /// Whether this is a login shell.
    #[arg(short = 'l', long, default_value_t = false)]
    is_login: bool,

    /// Whether this is an interactive session.
    #[arg(short = 'i', long = "interactive", default_value_t = false)]
    is_interactive_session: bool,

    /// Whether to enable private mode.
    #[arg(short = 'P', long = "private", default_value_t = false)]
    private_mode: bool,
    // /// Profile to login as
    // #[arg(short = 'p', long, value_name = "PROFILE")]
    // profile: Option<Profile>,

    // /// Unstable features to enable
    // #[arg(short = 'f', long, value_name = "FEATURES")]
    // features: Option<Vec<Feature>>,
}

impl RushiArgs {
    fn imply_args(mut self) -> Self {
        // if the first argument starts with a dash, we are a login shell
        if std::env::args().take(1).any(|arg| arg.starts_with('-')) {
            self.is_login = true;
        }

        // no_config implies private mode
        if self.no_config {
            self.private_mode = true;
        }

        // an output file implies something to output
        if self.debug_output.is_some() {
            self.debug = true;
        }

        // We are an interactive session if we have not been given an explicit
        // command or file to execute and stdin is a tty. Note that the -i or
        // --interactive options also force interactive mode.
        if self.batch_cmds.is_none() && atty::is(atty::Stream::Stdin) {
            self.is_interactive_session = true;
        }

        self
    }
}

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
    let args = RushiArgs::parse().imply_args();

    if args.debug {
        simplelog::WriteLogger::init(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            File::create(
                args.debug_output
                    .clone()
                    .unwrap_or_else(|| PathBuf::from("rushi.log")),
            )?,
        )?;
        log::info!("Debug mode enabled");
    }

    // setlocale(LC_ALL, "");


    let interpreter = Interpreter::new();

    // TODO: better implementation is to build config from args then env 
    // from the config

    // source user and system config
    // let mut paths = ConfigPaths::new(&args);
    // paths.source(&interpreter, &mut env, &mut sys);

    let mut env = UserState::new(&args);

    let mut l = Line::new();

    println!("Welcome to Rushi!");
    println!("Type 'exit' to exit.");

    // let (lsp, rx) = Client::start("rust-analyzer", &[""], None, HashMap::new(), 0, "rls", 100)?;
    // lsp.initialize(true).await?;

    'running: loop {
        let line = l.next_line().unwrap();

        let ast = Ast::parse(&line).unwrap();
        let mut optc = OpCode::from(ast);
        for _ in 0..=OPTIMIZATION_LEVEL { optc.reduce(); }
        let bytes = ByteCode::from(optc);

        let res = interpreter.eval(bytes, &mut env);
        res.unwrap(); // this will do until error handling is needed
    }

    // restore_term_mode();
    // restore_term_foreground_process_group_for_exit();

    Ok(())
}
