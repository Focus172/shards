
use std::{
    fs::File,
    path::PathBuf,
};

use crate::RushiArgs;
use crate::env::{UserState, SystemState};
use crate::interpreter::Interpreter;

pub struct ConfigPaths {
    user: Option<PathBuf>,
    sys: Option<PathBuf>,
}

impl ConfigPaths {
    pub fn new(args: &RushiArgs) -> ConfigPaths {
        if args.no_config {
            ConfigPaths {
                user: None,
                sys: None,
            }
        } else {
            // let Some(p) = args.custom_config

            let xdg = xdg::BaseDirectories::new().unwrap();
            let user = xdg.create_config_directory(PathBuf::from("rushi")).unwrap();
            let sys = PathBuf::from("/etc/rushi");

            ConfigPaths {
                user: Some(user),
                sys: Some(sys),
            }
        }
    }

    /// Parse init files. exec_path is the path of fish executable as determined by argv[0].
    pub fn source(&mut self, interpreter: &Interpreter, env: &mut UserState, sys: &mut SystemState) {
        if let Some(user) = &mut self.user {
            log::info!("sourcing user config");
            Self::source_config_in_directory(user, env);
        }
        if let Some(sys) = &mut self.sys {
            log::info!("sourcing system config");
            Self::source_config_in_directory(sys, env);
        }
    }

    /// Source the file config.rsi or config.zigi in the given directory.
    /// If the config file doesn't exist or isn't readable silently return.
    fn source_config_in_directory(path: &mut PathBuf, _env: &mut UserState) {
        // We need to get the configuration directory before we can source the user configuration file.
        // If path_get_config returns false then we have no configuration directory and no custom config
        // to load.

        // let paths = get_config_paths();
        // if let Some(path) = paths.data {
        //     for path in paths {
        //         source_config_in_directory(path, env);
        //     }
        // }

        path.push("config.rsi");
        let name = path.display().to_string();

        let _f = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                log::warn!(
                    "not sourcing {} (Not readable or does not exist): {}",
                    name,
                    e
                );
                return;
            }
        };

        log::info!("sourcing {:?}", name);

        // Parser::new(f).parse(env);
    }
}

