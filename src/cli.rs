use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "cleanup")]
#[command(about = "Red team cleanup tool — Rust edition")]
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

    #[arg(short, long)]        // <-- NEW
    pub wipe: bool,

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
