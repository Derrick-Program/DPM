#![allow(warnings)]
mod cli_parse;
mod config;
mod error;
mod arch;
// mod json_parse;

use clap_complete::Shell;
use cli_parse::*;
use colored::Colorize;
use config::*;
use error::*;
use arch::*;

pub type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn entry(config: Cli) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}
pub fn get_args() -> MyResult<Cli> {
    let matches = build_cli().get_matches();
    if let Some(generator) = matches.get_one::<Shell>("generator").copied() {
        let mut cmd = build_cli();
        eprintln!("Generating completion file for {generator}...");
        print_completions(generator, &mut cmd);
    }
    let mut Commands: Option<CliCommands> = Option::<CliCommands>::None;
    let mut Verbose = false;
    let mut PN = vec![];
    let mut Other = Option_set::default();

    let config = match matches.subcommand() {
        Some(("install", sub_command)) => {
            Commands = Some(CliCommands::Install);
            Verbose = sub_command.get_flag("verbose");
            PN = sub_command
                .get_many::<String>("PN")
                .unwrap_or_default()
                .map(|v| v.to_string())
                .collect::<Vec<String>>();
        }
        Some(("update", sub_command)) => {
            Commands = Some(CliCommands::Update);
            Verbose = sub_command.get_flag("verbose");
            PN = sub_command
                .get_many::<String>("PN")
                .unwrap_or_default()
                .map(|v| v.to_string())
                .collect::<Vec<String>>();
        }
        Some(("uninstall", sub_command)) => {
            Commands = Some(CliCommands::Uninstall);
            Verbose = sub_command.get_flag("verbose");
            PN = sub_command
                .get_many::<String>("PN")
                .unwrap_or_default()
                .map(|v| v.to_string())
                .collect::<Vec<String>>();
        }
        Some(("search", sub_command)) => {
            Commands = Some(CliCommands::Search);
            Verbose = sub_command.get_flag("verbose");
            PN = sub_command
                .get_many::<String>("PN")
                .unwrap_or_default()
                .map(|v| v.to_string())
                .collect::<Vec<String>>();
        }
        Some(("list", sub_command)) => {
            Commands = Some(CliCommands::List);
            Verbose = sub_command.get_flag("verbose");
            Other.List_installed = Some(sub_command.get_flag("list-installed"));
        }
        Some(("upgrade", sub_command)) => {
            Commands = Some(CliCommands::List);
            Verbose = sub_command.get_flag("verbose");
            Other.Upgrade_self = Some(sub_command.get_flag("upgrade-self"));
        }
        _ => return Err(Box::new(CommandParseError::new("Unrecognized command"))),
    };
    let PackageName = if PN.is_empty() { None } else { Some(PN) };
    Ok(Cli {
        Commands,
        PackageName,
        Verbose,
        Other: Some(Other),
    })
}
//fn check_dir_exists(dir_name: Path) -> MyResult<bool> {
//    println!("{}", dir_name.display());
//    Ok(true)
//}
