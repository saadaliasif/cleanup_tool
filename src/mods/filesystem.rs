use crate::logger::Logger;
use crate::utils::safe_remove;
use anyhow::Result;
use std::process::Command;
use std::thread;
use std::time::Duration;

pub fn clean(
    logger: &Logger,
    dry_run: bool,
    advanced: bool,
    wipe: bool,          // <-- NEW
) -> Result<(usize, usize)> {
    logger.info("Cleaning Windows file system traces...");
    let mut cleaned = 0;
    let mut failed = 0;

    let drives = get_fixed_drives();

    // USN Journal
    if !dry_run {
        for drive in &drives {
            match Command::new("fsutil")
                .args(&["usn", "deletejournal", "/D", drive])
                .output()
            {
                Ok(_) => logger.verbose(&format!("Deleted USN journal on {}", drive)),
                Err(e) => {
                    failed += 1;
                    logger.verbose(&format!("Failed USN delete on {}: {}", drive, e));
                }
            }
        }
        cleaned += 1;
    } else {
        logger.verbose("[DRY RUN] Would delete USN journals");
        cleaned += 1;
    }

    // Recycle Bin
    if !dry_run {
        for drive in &drives {
            let recycle = format!("{}\\$Recycle.Bin", drive.trim_end_matches('\\'));
            let _ = crate::utils::remove_dir_contents(&recycle, dry_run, logger);
        }
        cleaned += 1;
    } else {
        logger.verbose("[DRY RUN] Would empty Recycle Bins");
        cleaned += 1;
    }

    // Thumbcache
    let thumb = format!(
        r"{}\AppData\Local\Microsoft\Windows\Explorer\thumbcache_*.db",
        std::env::var("USERPROFILE").unwrap_or_default()
    );
    if !dry_run {
        let _ = Command::new("cmd")
            .args(&["/c", &format!("del /q \"{}\" 2>nul", thumb)])
            .output();
        cleaned += 1;
    }

    // SRUDB.dat — NOW USES secure wipe!
    if safe_remove(r"C:\Windows\System32\sru\SRUDB.dat", dry_run, wipe, logger) {
        cleaned += 1;
    }

    // Windows.edb (Advanced only) — NOW USES secure wipe!
    if advanced && !dry_run {
        logger.verbose("Stopping Windows Search service...");
        let _ = Command::new("net").args(&["stop", "WSearch"]).output();

        let edb = r"C:\ProgramData\Microsoft\Search\Data\Applications\Windows\Windows.edb";
        if safe_remove(edb, dry_run, wipe, logger) {
            cleaned += 1;
            logger.verbose("Cleaned Windows Search Index (Advanced mode)");
        }

        thread::sleep(Duration::from_secs(3));
        let _ = Command::new("net").args(&["start", "WSearch"]).output();
    } else if !dry_run {
        logger.verbose("Skipped Windows Search Index (use --advanced to enable)");
    }

    // Notifications
    if !dry_run {
        let notif = format!(
            r"{}\Microsoft\Windows\Notifications\*",
            std::env::var("LOCALAPPDATA").unwrap_or_default()
        );
        let _ = Command::new("cmd")
            .args(&["/c", &format!("del /q \"{}\" 2>nul", notif)])
            .output();
        cleaned += 1;
        logger.verbose("Cleaned Windows Notification History");
    }

    // Thumbs.db recursive
    if !dry_run {
        let profile = std::env::var("USERPROFILE").unwrap_or_default();
        let _ = Command::new("cmd")
            .args(&[
                "/c",
                &format!("del /s /q \"{}\\Thumbs.db\" 2>nul", profile),
            ])
            .output();
        cleaned += 1;
        logger.verbose("Removed legacy Thumbs.db files");
    }

    logger.success(&format!("File system traces cleaned ({} items)", cleaned));
    Ok((cleaned, failed))
}

fn get_fixed_drives() -> Vec<String> {
    ('C'..='Z')
        .map(|c| format!("{}:\\", c))
        .filter(|p| std::path::Path::new(p).exists())
        .collect()
}
