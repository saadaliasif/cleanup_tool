pub mod events;
pub mod history;
pub mod registry;
pub mod filesystem;
pub mod temp;
pub mod security;
pub mod advanced;

use crate::cli::Module;
use crate::logger::Logger;
use anyhow::Result;

pub fn run_module(
    module: Module,
    dry_run: bool,
    advanced: bool,
    wipe: bool,          // <-- NEW
    logger: &Logger,
) -> Result<(usize, usize)> {
    match module {
        Module::Events => events::clean(logger, dry_run, wipe),
        Module::History => history::clean(logger, dry_run, wipe),
        Module::Registry => registry::clean(logger, dry_run, wipe),
        Module::Filesystem => filesystem::clean(logger, dry_run, advanced, wipe),
        Module::Temp => temp::clean(logger, dry_run, advanced, wipe),
        Module::Security => security::clean(logger, dry_run, wipe),
        Module::Advanced => advanced::clean(logger, dry_run, wipe),
    }
}
