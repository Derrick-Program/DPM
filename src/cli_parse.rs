#![allow(warnings)]
use crate::{MyResult, BIN, VERSION};
use clap::{value_parser, Arg, ArgAction, ArgGroup, ColorChoice, Command, ValueHint};
use clap_complete::{generate, Generator, Shell};
use std::io;

pub fn build_cli() -> Command {
    Command::new(BIN)
        .version(VERSION)
        .color(ColorChoice::Always)
        .styles(get_styles())
        .author("Derrick Lin")
        .about("Derrick Package Manager (DPM)")
        .propagate_version(true)
        .arg_required_else_help(true)
        .subcommands([
            Command::new("install")
                .about("Install Package")
                .visible_aliases(["i", "add", "inst"])
                .arg_required_else_help(true)
                .arg(
                    Arg::new("PN")
                        .value_name("Package Name")
                        .required(true)
                        .action(ArgAction::Append),
                )
                .arg(
                    Arg::new("verbose")
                        .help("Verbose")
                        .short('v')
                        .long("verbose")
                        .action(ArgAction::SetTrue),
                ),
            Command::new("update")
                .about("Update Package")
                .visible_aliases(["ud", "upda", "up"])
                .arg_required_else_help(true)
                .arg(
                    Arg::new("PN")
                        .value_name("Package name")
                        .required(true)
                        .action(ArgAction::Append),
                )
                .arg(
                    Arg::new("verbose")
                        .help("Verbose")
                        .short('v')
                        .long("verbose")
                        .action(ArgAction::SetTrue),
                ),
            Command::new("uninstall")
                .about("Uninstall Package")
                .arg_required_else_help(true)
                .visible_aliases(["un", "i!", "unin"])
                .arg(
                    Arg::new("PN")
                        .value_name("Package name")
                        .required(true)
                        .action(ArgAction::Append),
                )
                .arg(
                    Arg::new("verbose")
                        .help("Verbose")
                        .short('v')
                        .long("verbose")
                        .action(ArgAction::SetTrue),
                ),
            Command::new("search")
                .about("Search Package")
                .arg_required_else_help(true)
                .visible_aliases(["s", "se", "sea"])
                .arg(
                    Arg::new("PN")
                        .value_name("Package name")
                        .required(true)
                        .action(ArgAction::Append),
                )
                .arg(
                    Arg::new("verbose")
                        .help("Verbose")
                        .short('v')
                        .long("verbose")
                        .action(ArgAction::SetTrue),
                ),
            Command::new("list")
                .about("List can install Package")
                .visible_aliases(["l", "li", "ll"])
                .arg_required_else_help(true)
                .arg(
                    Arg::new("verbose")
                        .help("Verbose")
                        .short('v')
                        .long("verbose")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("list-installed")
                        .help("List installed Package")
                        .short('l')
                        .long("list")
                        .action(ArgAction::SetTrue),
                ),
            Command::new("upgrade")
                .about("Upgrade Package")
                .arg_required_else_help(true)
                .visible_aliases(["U", "UP", "grade"])
                .arg(
                    Arg::new("verbose")
                        .help("Verbose")
                        .short('v')
                        .long("verbose")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("upgrade-self")
                        .help("Self Upgrade")
                        .short('S')
                        .long("Self")
                        .action(ArgAction::SetTrue),
                ),
        ])
        .arg(
            Arg::new("generator")
                .short('g')
                .long("gen")
                .action(ArgAction::Set)
                .aliases(["gen", "generator", "autocomplete", "complete"])
                .value_parser(value_parser!(Shell)),
        )
}

pub fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
    std::process::exit(0);
}

pub fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .usage(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .header(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .literal(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .invalid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .error(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .valid(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .placeholder(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
}
