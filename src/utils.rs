use crate::logger::Logger;
use std::fs;
use std::path::Path;

pub fn safe_remove(path: &str, dry_run: bool, logger: &Logger) -> bool {
    if !Path::new(path).exists() {
        return false;
    }

    if dry_run {
        logger.verbose(&format!("[DRY RUN] Would remove: {}", path));
        return true;
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
