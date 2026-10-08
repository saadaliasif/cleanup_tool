use crate::logger::Logger;
use std::fs::{self, OpenOptions};
use std::io::{self, Seek, SeekFrom, Write};
use std::path::Path;

/// Overwrite file with random bytes `passes` times, then delete.
/// This prevents simple forensic recovery of deleted files.
pub fn secure_wipe(path: &str, passes: u32) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).open(path)?;
    let size = file.metadata()?.len() as usize;

    if size == 0 {
        fs::remove_file(path)?;
        return Ok(());
    }

    let chunk_size = 64 * 1024; // 64KB chunks
    let mut buffer = vec![0u8; chunk_size];

    for _pass in 0..passes {
        file.seek(SeekFrom::Start(0))?;
        let mut written = 0;

        while written < size {
            // Fill buffer with random bytes
            rand::fill(&mut buffer[..]);

            let to_write = (size - written).min(chunk_size);
            file.write_all(&buffer[..to_write])?;
            written += to_write;
        }

        file.sync_all()?; // Force to disk
    }

    // Final overwrite with zeros (hides random pattern)
    file.seek(SeekFrom::Start(0))?;
    let zeros = vec![0u8; chunk_size];
    let mut written = 0;
    while written < size {
        let to_write = (size - written).min(chunk_size);
        file.write_all(&zeros[..to_write])?;
        written += to_write;
    }
    file.sync_all()?;
    drop(file); // Close handle before delete

    fs::remove_file(path)?;
    Ok(())
}

pub fn safe_remove(path: &str, dry_run: bool, wipe: bool, logger: &Logger) -> bool {
    if !Path::new(path).exists() {
        return false;
    }

    if dry_run {
        logger.verbose(&format!(
            "[DRY RUN] Would {}remove: {}",
            if wipe { "secure-wipe then " } else { "" },
            path
        ));
        return true;
    }

    if wipe {
        logger.verbose(&format!("Secure-wiping: {}", path));
        match secure_wipe(path, 3) {
            Ok(_) => {
                logger.verbose(&format!("Secure-wiped: {}", path));
                return true;
            }
            Err(e) => {
                logger.verbose(&format!("Secure wipe failed ({}), falling back to normal delete", e));
                // Fall through to normal delete
            }
        }
    }

    match fs::remove_file(path) {
        Ok(_) => true,
        Err(e) => {
            logger.verbose(&format!("Failed to remove {}: {}", path, e));
            false
        }
    }
}

pub fn truncate_file(path: &str, dry_run: bool, logger: &Logger) -> bool {
    if !Path::new(path).exists() {
        return false;
    }

    if dry_run {
        logger.verbose(&format!("[DRY RUN] Would truncate: {}", path));
        return true;
    }

    match fs::write(path, b"") {
        Ok(_) => true,
        Err(e) => {
            logger.verbose(&format!("Failed to truncate {}: {}", path, e));
            false
        }
    }
}

pub fn remove_dir_contents(path: &str, dry_run: bool, logger: &Logger) -> bool {
    if dry_run {
        logger.verbose(&format!("[DRY RUN] Would clean directory: {}", path));
        return true;
    }

    let Ok(entries) = fs::read_dir(path) else {
        return false;
    };

    let mut ok = true;
    for entry in entries.flatten() {
        let p = entry.path();
        let result = if p.is_dir() {
            fs::remove_dir_all(&p)
        } else {
            fs::remove_file(&p)
        };

        if let Err(e) = result {
            logger.verbose(&format!("Failed to remove {}: {}", p.display(), e));
            ok = false;
        }
    }
    ok
}
