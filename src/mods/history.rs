use crate::logger::Logger;
use crate::utils::{safe_remove, truncate_file};
use anyhow::Result;
use std::process::Command;

pub fn clean(logger: &Logger, dry_run: bool) -> Result<(usize, usize)> {
    logger.info("Cleaning Windows history...");
    let mut cleaned = 0;

    let ps_history = format!(
        r"{}\Microsoft\Windows\PowerShell\PSReadLine\ConsoleHost_history.txt",
        std::env::var("APPDATA").unwrap_or_default()
    );
    if truncate_file(&ps_history, dry_run, logger) {
        cleaned += 1;
        logger.verbose("Cleaned PowerShell history");
    }

    if !dry_run {
        let _ = Command::new("doskey").arg("/reinstall").output();
        cleaned += 1;
    }

    if !dry_run {
        let _ = Command::new("cmd")
            .args(&[
                "/c",
                r"del /q C:\Windows\Prefetch\*.pf",
            ])
            .output();
        cleaned += 1;
        logger.verbose("Cleaned Prefetch");
    }

    let recent = format!(
        r"{}\Microsoft\Windows\Recent\*.lnk",
        std::env::var("APPDATA").unwrap_or_default()
    );
    if !dry_run {
        let _ = Command::new("cmd").args(&["/c", &format!("del /q {}", recent)]).output();
        cleaned += 1;
    }

    let edge_db = format!(
        r"{}\Microsoft\Edge\User Data\Default\History",
        std::env::var("LOCALAPPDATA").unwrap_or_default()
    );
    if safe_remove(&edge_db, dry_run, logger) {
        cleaned += 1;
        logger.verbose("Cleaned Edge (Chromium) History");
    }

    logger.success(&format!("Windows history cleaned ({} items)", cleaned));
    Ok((cleaned, 0))
}
