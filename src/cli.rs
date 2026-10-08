use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "cleanup")]
#[command(about = "saads cleanup tool")]
pub struct Cli {
    #[arg(short, long)]
    pub dry_run: bool,

    #[arg(short, long)]
    pub debug: bool,

    #[arg(short, long)]
    pub list: bool,

    #[arg(short, long, value_delimiter = ',')]
    pub modules: Vec<Module>,

    #[arg(short, long)]
    pub force: bool,

    #[arg(short, long)]
    pub advanced: bool,

    #[arg(long)]
    pub log_file: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Module {
    Events,
    History,
    Registry,
    Filesystem,
    Temp,
    Security,
    Advanced,
}
