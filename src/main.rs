mod cli;
mod logger;
mod mods;
mod privilege;
mod utils;

use anyhow::Result;
use clap::Parser;
use std::io::{self, Write};

fn main() -> Result<()> {
    let args = cli::Cli::parse();

    if args.list {
        println!("Available cleaning modules:");
        println!("  events     - Windows Event Logs");
        println!("  history    - PowerShell/CMD history, prefetch, jump lists");
        println!("  registry   - Registry MRUs, USB history, BAM");
        println!("  filesystem - USN journal, recycle bin, thumbcache");
        println!("  temp       - Temporary files, DNS cache, shadow copies");
        println!("  security   - EDR/AV logs");
        println!("  advanced   - Certificates, tasks, services");
        return Ok(());
    }

    let logger = logger::Logger::new(args.log_file, args.debug)?;

    logger.banner();
    logger.warning();

    if !args.dry_run {
        privilege::check_admin()?;
    }

    if !args.force && !args.dry_run {
        print!("Do you want to continue? [y/N] ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Aborted.");
            return Ok(());
        }
    }

    if args.dry_run {
        logger.info("Running in DRY RUN mode — no changes will be made");
    }

    let modules_to_run = if args.modules.is_empty() {
        vec![
            cli::Module::Events,
            cli::Module::History,
            cli::Module::Registry,
            cli::Module::Filesystem,
            cli::Module::Temp,
            cli::Module::Security,
            cli::Module::Advanced,
        ]
    } else {
        args.modules.clone()
    };

    let mut total_cleaned = 0;
    let mut total_failed = 0;

    for module in &modules_to_run {
        let name = format!("{:?}", module).to_lowercase();
        logger.info(&format!("Running module: {}", name));

        match mods::run_module(*module, args.dry_run, args.advanced, &logger) {
            Ok((c, f)) => {
                total_cleaned += c;
                total_failed += f;
            }
            Err(e) => {
                logger.error(&format!("Module {} failed: {}", name, e));
                total_failed += 1;
            }
        }
    }

    println!();
    logger.success("Cleaning Complete!");
    logger.info(&format!("Total items cleaned: {}", total_cleaned));
    logger.info(&format!("Failed operations: {}", total_failed));

    if args.dry_run {
        logger.warning();
        println!("This was a dry run. No actual changes were made.");
    }

    Ok(())
}
