use crate::logger::Logger;
use anyhow::Result;
use std::fs;
use std::process::Command;

pub fn clean(logger: &Logger, dry_run: bool) -> Result<(usize, usize)> {
    logger.info("Cleaning advanced security traces...");
    let mut cleaned = 0;
    let mut failed = 0;

    // Test certificates
    if !dry_run {
        let ps = r"Get-ChildItem -Path Cert:\CurrentUser\My | Where-Object { $_.Subject -like '*NYX-TEST*' } | Remove-Item -Force";
        let _ = Command::new("powershell")
            .args(&["-Command", ps])
            .output();
        cleaned += 1;
    }

    // Scheduled tasks
    if !dry_run {
        let ps = r"Get-ScheduledTask | Where-Object { $_.TaskName -like '*NYX-TEST*' } | Unregister-ScheduledTask -Confirm:$false";
        let _ = Command::new("powershell")
            .args(&["-Command", ps])
            .output();
        cleaned += 1;
    }

    // Test services
    if !dry_run {
        let ps = r"Get-Service | Where-Object { $_.Name -like '*NYX-TEST*' } | ForEach-Object { Stop-Service $_.Name -Force -ErrorAction SilentlyContinue; & sc.exe delete $_.Name }";
        let _ = Command::new("powershell")
            .args(&["-Command", ps])
            .output();
        cleaned += 1;
    }

    // Wireless profiles
    if !dry_run {
        let out = Command::new("netsh")
            .args(&["wlan", "show", "profiles"])
            .output()?;
        let text = String::from_utf8_lossy(&out.stdout);
        for line in text.lines() {
            if line.contains("NYX-TEST") {
                if let Some(idx) = line.find(':') {
                    let name = line[idx + 1..].trim();
                    let _ = Command::new("netsh")
                        .args(&[
                            "wlan",
                            "delete",
                            "profile",
                            &format!("name=\"{}\"", name),
                        ])
                        .output();
                    cleaned += 1;
                }
            }
        }
    }

    // VPN connections
    if !dry_run {
        let ps = r"Get-VpnConnection | Where-Object { $_.Name -like '*NYX-TEST*' } | Remove-VpnConnection -Force -ErrorAction SilentlyContinue";
        let _ = Command::new("powershell")
            .args(&["-Command", ps])
            .output();
        cleaned += 1;
    }

    // Chrome extensions
    if !dry_run {
        let chrome = format!(
            r"{}\Google\Chrome\User Data\Default\Extensions\nyxtest*",
            std::env::var("LOCALAPPDATA").unwrap_or_default()
        );
        let _ = Command::new("cmd")
            .args(&["/c", &format!("rd /s /q \"{}\" 2>nul", chrome)])
            .output();
        cleaned += 1;
        logger.verbose("Cleaned Chrome extensions");
    }

    // Crypto provider data
    if !dry_run {
        let crypto = format!(
            r"{}\Microsoft\Crypto\RSA\*nyx*",
            std::env::var("APPDATA").unwrap_or_default()
        );
        let _ = Command::new("cmd")
            .args(&["/c", &format!("del /q \"{}\" 2>nul", crypto)])
            .output();
        cleaned += 1;
        logger.verbose("Cleaned cryptographic provider data");
    }

    // Windows Update logs — filter lines instead of deleting
    if !dry_run {
        let wu = r"C:\Windows\SoftwareDistribution\ReportingEvents.log";
        if fs::metadata(wu).is_ok() {
            let content = fs::read_to_string(wu).unwrap_or_default();
            let filtered: Vec<&str> = content
                .lines()
                .filter(|l| !l.contains("NYX-WINDOWS-UPDATE-TEST"))
                .collect();
            let _ = fs::write(wu, filtered.join("\n"));
            cleaned += 1;
        }
    }

    // Windows Push Notifications
    if !dry_run {
        let wpn = format!(
            r"{}\Microsoft\Windows\Notifications\*nyx*",
            std::env::var("LOCALAPPDATA").unwrap_or_default()
        );
        let _ = Command::new("cmd")
            .args(&["/c", &format!("del /q \"{}\" 2>nul", wpn)])
            .output();
        cleaned += 1;
        logger.verbose("Cleaned Windows Push Notifications");
    }

    logger.success(&format!(
        "Advanced security traces cleaned ({} items)",
        cleaned
    ));
    Ok((cleaned, failed))
}
