use std::process::exit;

use clap::Parser;

mod backend;
mod cli;
mod fmt;

use backend::{ListCommand, ShowCommand};
use cli::{
    Cli,
    Command::{List, Show},
};

fn main() {
    let cli = Cli::parse();

    let exit_code = match cli.command {
        Some(c) => match c {
            Show(args) => ShowCommand::execute(&args),
            List(args) => ListCommand::execute(&args),
        },
        None => backend::no_command(),
    };

    exit(exit_code.into());
}
