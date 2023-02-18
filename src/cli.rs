use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[arg(short, action = clap::ArgAction::Count)]
    pub debug: u8,
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    List(ListArgs),
    Show(ShowArgs),
}

#[derive(Args, Debug)]
pub struct ShowArgs {
    #[arg(long, short, action = clap::ArgAction::Append)]
    pub reactions: Option<Vec<String>>,
    pub compound: String,
}

#[derive(Args, Debug)]
pub struct ListArgs {
    #[arg(long, short)]
    pub reaction: bool,

    #[arg(long, short)]
    pub compound: bool,
}
