use crate::logger::Logger;
use anyhow::Result;
use std::process::Command;

pub fn clean(logger: &Logger, dry_run: bool) -> Result<(usize, usize)> {
    logger.info("Cleaning Windows event logs...");
    let mut cleaned = 0;
    let mut failed = 0;

    if dry_run {
        logger.verbose("[DRY RUN] Would clear all event logs");
        cleaned = 180;
    } else {
        // Get all log names
        let output = Command::new("wevtutil").arg("el").output()?;
        let logs = String::from_utf8_lossy(&output.stdout);

        for log in logs.lines() {
            let log = log.trim();
            if log.is_empty() {
                continue;
            }

            match Command::new("wevtutil").args(&["cl", log]).output() {
                Ok(_) => cleaned += 1,
                Err(_) => {
                    failed += 1;
                    logger.verbose(&format!("Failed to clear event log: {}", log));
                }
            }
        }

        // Backup method for critical logs
        for critical in &["Application", "System", "Security"] {
            let _ = Command::new("wevtutil").args(&["cl", critical]).output();
        }
    }

    logger.success(&format!("Event logs cleaned ({} logs)", cleaned));
    Ok((cleaned, failed))
}
