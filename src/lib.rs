#![allow(warnings)]
mod action;
mod arch;
mod cli_parse;
mod config;
mod repo;
mod utils;
pub use repo::*;
use std::collections::HashMap;
use std::path::Path;

pub use action::*;
pub use arch::*;
pub use cli_parse::get_args;
pub use config::*;
use std::fs::create_dir as mkdir;
use std::fs::read_dir;
pub use utils::*;
pub type MyResult<T> = Result<T, Box<dyn std::error::Error>>;
pub type setting = HashMap<String, String>;
pub type hashes = HashMap<String, String>;
use tokio;
#[tokio::main]
pub async fn entry(config: Cli) -> MyResult<()> {
    let setting_config: setting = init().await?;
    let db_info = Db::new(Path::new(MAIN_DIR))?;
    let pass_info = ActionInfo::new(
        config.PackageName.unwrap_or_default(),
        config.Verbose,
        setting_config,
        // repo_info,
        db_info,
    );
    match config.Commands.unwrap() {
        CliCommands::Install => pass_info.install().await?,
        CliCommands::List => {
            if let Some(options) = &config.Other {
                if let Some(true) = options.List_sys_installed {
                    pass_info.list(true)?;
                }
                if let Some(true) = options.List_installed {
                    pass_info.list(false)?;
                }
            }
        }
        CliCommands::Search => pass_info.search()?,
        CliCommands::Uninstall => pass_info.uninstall()?,
        CliCommands::Update => pass_info.update().await?,
        CliCommands::Upgrade => pass_info.upgrade(),
        CliCommands::UpgradeSelf => pass_info.upgrade_self(),
        CliCommands::None => panic!("No command found"),
    }
    permision_check();
    Ok(())
}
