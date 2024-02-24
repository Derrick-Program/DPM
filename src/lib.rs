#![allow(non_snake_case)]
#![allow(warnings)]
mod action;
mod arch;
mod cli_parse;
mod config;
mod utils;
use std::path::Path;

use action::*;
use arch::*;
pub use cli_parse::get_args;
use config::*;
use std::fs::create_dir as mkdir;
use std::fs::read_dir;
pub use utils::*;
pub type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn entry(config: Cli) -> MyResult<()> {
    // println!("{:#?}", config);
    match config.Commands.unwrap() {
        CliCommands::Install => install(&config.PackageName.unwrap()),
        CliCommands::List => {
            if let Some(options) = &config.Other {
                if let Some(true) = options.List_sys_installed {
                    list(true);
                }
                if let Some(true) = options.List_installed {
                    list(false);
                }
            }
        }
        CliCommands::Search => search(&config.PackageName.unwrap()),
        CliCommands::Uninstall => uninstall(&config.PackageName.unwrap()),
        CliCommands::Update => update(),
        CliCommands::Upgrade => upgrade(&config.PackageName.unwrap()),
        CliCommands::UpgradeSelf => upgrade_self(),
        CliCommands::None => panic!("No command found"),
    }
    // if !utils::check_dir_exists(Path::new(INSTALL_DIR)) {
    //     mkdir(INSTALL_DIR)?;
    // }
    // list_dir(Path::new(INSTALL_DIR));
    Ok(())
}
