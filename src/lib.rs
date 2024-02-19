#![allow(non_snake_case)]
#![allow(warnings)]

mod arch;
mod cli_parse;
mod config;
mod utils;

use arch::*;
pub use cli_parse::get_args;
use config::*;
pub use utils::*;

pub type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn entry(config: Cli) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

fn install() {
    todo!();
}

fn update() {
    todo!();
}

fn uninstall() {
    todo!();
}
fn search() {
    todo!();
}
fn list() {
    todo!();
}
fn upgrade() {
    todo!();
}
//fn check_dir_exists(dir_name: Path) -> MyResult<bool> {
//    println!("{}", dir_name.display());
//    Ok(true)
//}
