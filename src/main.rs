use clap::Command;

pub mod taurus;

fn main() {
    let cmd = Command::new("taurus")
        .bin_name("taurus")
        .styles(CLAP_STYLE)
        .subcommand_required(true);
    let matches = cmd.get_matches();
    let _matches = match matches.subcommand() {
        _ => unreachable!("unrecognized subcommand."),
    };
}

use clap::builder::styling::Styles;
use clap_cargo::style::*;

const CLAP_STYLE: Styles = Styles::styled()
    .header(HEADER)
    .usage(USAGE)
    .literal(LITERAL)
    .placeholder(PLACEHOLDER)
    .error(ERROR)
    .valid(VALID)
    .invalid(INVALID);

//const SUBCOMMANDS = [clap_command!("help")];
