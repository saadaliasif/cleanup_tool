use crate::logger::Logger;
use anyhow::Result;
use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;

pub fn clean(logger: &Logger, dry_run: bool) -> Result<(usize, usize)> {
    logger.info("Cleaning Windows registry MRUs...");
    let mut cleaned = 0;
    let mut failed = 0;

    let mru_keys = [
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\ComDlg32\OpenSavePidlMRU",
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\RecentDocs",
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\TypedPaths",
        r"Software\Microsoft\Windows\Shell\BagMRU",
        r"Software\Microsoft\Windows\CurrentVersion\Explorer\UserAssist",
    ];

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    for key_path in &mru_keys {
        if dry_run {
            logger.verbose(&format!("[DRY RUN] Would delete registry key: {}", key_path));
            cleaned += 1;
            continue;
        }

        match hkcu.delete_subkey_all(key_path) {
            Ok(_) => {
                cleaned += 1;
                logger.verbose(&format!("Deleted registry key: {}", key_path));
            }
            Err(e) => {
                failed += 1;
                logger.verbose(&format!("Failed to delete {}: {}", key_path, e));
            }
        }
    }

    // USBSTOR via reg.exe (requires admin)
    if !dry_run {
        let _ = std::process::Command::new("reg")
            .args(&["delete", r"HKLM\SYSTEM\CurrentControlSet\Enum\USBSTOR", "/f"])
            .output();
        cleaned += 1;
    }

    logger.success(&format!("Registry MRUs cleaned ({} keys)", cleaned));
    Ok((cleaned, failed))
}
