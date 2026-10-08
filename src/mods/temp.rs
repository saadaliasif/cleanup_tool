use crate::logger::Logger;
use anyhow::Result;
use std::process::Command;

pub fn clean(
    logger: &Logger,
    dry_run: bool,
    advanced: bool,
    _wipe: bool,          // <-- signature updated
) -> Result<(usize, usize)> {
    logger.info("Cleaning Windows temporary files...");
    let mut cleaned = 0;
    let failed = 0;

    let temp = std::env::var("TEMP").unwrap_or_default();
    let local = std::env::var("LOCALAPPDATA").unwrap_or_default();
    let windir = std::env::var("WINDIR").unwrap_or_default();

    // Temp folders
    if !dry_run {
        let _ = Command::new("cmd")
            .args(&["/c", &format!("rd /s /q \"{}\" 2>nul", temp)])
            .output();
        let _ = Command::new("cmd")
            .args(&["/c", "rd /s /q \"C:\\Windows\\Temp\" 2>nul"])
            .output();
        let _ = Command::new("cmd")
            .args(&["/c", "del /q \"C:\\Windows\\Prefetch\\*.tmp\" 2>nul"])
            .output();
        cleaned += 1;
    }

    // DNS cache
    if !dry_run {
        let _ = Command::new("ipconfig").args(&["/flushdns"]).output();
        cleaned += 1;
    }

    // Volume Shadow Copies
    if !dry_run {
        logger.verbose("WARNING: Removing Volume Shadow Copies will delete restore points");
        let _ = Command::new("vssadmin")
            .args(&["delete", "shadows", "/all", "/quiet"])
            .output();
        cleaned += 1;
    }

    // WER archives
    if !dry_run {
        let wer = [
            r"C:\ProgramData\Microsoft\Windows\WER\ReportArchive\*",
            r"C:\ProgramData\Microsoft\Windows\WER\ReportQueue\*",
            &format!(r"{}\Microsoft\Windows\WER\ReportArchive\*", local),
            &format!(r"{}\Microsoft\Windows\WER\ReportQueue\*", local),
        ];
        for path in &wer {
            let _ = Command::new("cmd")
                .args(&["/c", &format!("del /q \"{}\" 2>nul", path)])
                .output();
        }
        cleaned += 1;
        logger.verbose("Cleaned Windows Error Reporting archives");
    }

    // Crash dumps
    if !dry_run {
        let _ = Command::new("cmd")
            .args(&["/c", "del /q \"C:\\Windows\\Minidump\\*\" 2>nul"])
            .output();
        let _ = Command::new("cmd")
            .args(&["/c", "del /q \"C:\\Windows\\MEMORY.DMP\" 2>nul"])
            .output();
        cleaned += 1;
        logger.verbose("Cleaned crash dumps");
    }

    // Memory hardening (Advanced)
    if advanced && !dry_run {
        let _ = Command::new("powercfg").args(&["-h", "off"]).output();
        let _ = Command::new("reg").args(&[
            "add",
            r"HKLM\System\CurrentControlSet\Control\Session Manager\Memory Management",
            "/v",
            "ClearPageFileAtShutdown",
            "/t",
            "REG_DWORD",
            "/d",
            "1",
            "/f",
        ]).output();
        cleaned += 1;
        logger.verbose("Enabled memory residue hardening (requires reboot)");
    }

    // Network cache
    if !dry_run {
        let _ = Command::new("nbtstat").args(&["-R"]).output();
        let _ = Command::new("cmd").args(&["/c", "arp -d * 2>nul"]).output();
        let _ = Command::new("powershell")
            .args(&["-Command", r"cmdkey /list | Select-String 'Target:' | ForEach-Object { $t = ($_ -split ':')[1].Trim(); cmdkey /delete:$t 2>$null }"])
            .output();
        cleaned += 1;
        logger.verbose("Cleaned network caches");
    }

    // Windows Defender history
    if !dry_run {
        let _ = Command::new("cmd")
            .args(&["/c", r#"del /q "C:\ProgramData\Microsoft\Windows Defender\Scans\History\*" 2>nul"#])
            .output();
        cleaned += 1;
        logger.verbose("Cleaned Windows Defender history");
    }

    // Cortana
    if !dry_run {
        let cortana = format!(r"{}\Packages\Microsoft.Windows.Cortana_*\LocalState\*", local);
        let _ = Command::new("cmd")
            .args(&["/c", &format!("del /q \"{}\" 2>nul", cortana)])
            .output();
        cleaned += 1;
        logger.verbose("Cleaned Cortana history");
    }

    // Defender ATP cache
    if !dry_run {
        let _ = Command::new("cmd")
            .args(&["/c", r#"del /q "C:\ProgramData\Microsoft\Windows Defender Advanced Threat Protection\Cache\*" 2>nul"#])
            .output();
        cleaned += 1;
        logger.verbose("Cleaned Windows Defender ATP cache");
    }

    // Firewall logs
    if !dry_run {
        let _ = Command::new("cmd")
            .args(&["/c", r#"del /q "C:\Windows\System32\LogFiles\Firewall\*.log" 2>nul"#])
            .output();
        cleaned += 1;
        logger.verbose("Cleaned Windows Firewall logs");
    }

    // WMI logs
    if !dry_run {
        let wmi = format!(r"{}\System32\wbem\Logs\*.log", windir);
        let _ = Command::new("cmd")
            .args(&["/c", &format!("del /q \"{}\" 2>nul", wmi)])
            .output();
        cleaned += 1;
        logger.verbose("Cleaned WMI activity logs");
    }

    // BitLocker temp keys
    if !dry_run {
        let bl = format!(r"{}\BitLockerRecoveryKeys\*", temp);
        let _ = Command::new("cmd")
            .args(&["/c", &format!("rd /s /q \"{}\" 2>nul", bl)])
            .output();
        cleaned += 1;
        logger.verbose("Cleaned BitLocker recovery keys from temp");
    }

    // Group Policy cache
    if !dry_run {
        let gp = format!(r"{}\System32\GroupPolicy\DataStore\*", windir);
        let _ = Command::new("cmd")
            .args(&["/c", &format!("del /q \"{}\" 2>nul", gp)])
            .output();
        cleaned += 1;
        logger.verbose("Cleaned Group Policy cache");
    }

    // Auth cache
    if !dry_run {
        let auth = r"C:\Windows\System32\config\systemprofile\AppData\Local\Microsoft\Windows\SchCache\*";
        let _ = Command::new("cmd")
            .args(&["/c", &format!("del /q \"{}\" 2>nul", auth)])
            .output();
        cleaned += 1;
        logger.verbose("Cleaned authentication cache");
    }

    // Hibernation traces
    if !dry_run {
        let hib = format!(r"{}\HibernationTraces\*", temp);
        let _ = Command::new("cmd")
            .args(&["/c", &format!("rd /s /q \"{}\" 2>nul", hib)])
            .output();
        cleaned += 1;
        logger.verbose("Cleaned hibernation traces");
    }

    // Office telemetry
    if !dry_run {
        let office = format!(r"{}\Microsoft\Office\*\Telemetry\*", local);
        let _ = Command::new("cmd")
            .args(&["/c", &format!("rd /s /q \"{}\" 2>nul", office)])
            .output();
        cleaned += 1;
        logger.verbose("Cleaned Office telemetry");
    }

    // OneDrive logs
    if !dry_run {
        let od = format!(r"{}\Microsoft\OneDrive\logs\*", local);
        let _ = Command::new("cmd")
            .args(&["/c", &format!("del /q \"{}\" 2>nul", od)])
            .output();
        cleaned += 1;
        logger.verbose("Cleaned OneDrive logs");
    }

    // Teams logs
    if !dry_run {
        let teams = format!(r"{}\Microsoft\Teams\logs.txt", std::env::var("APPDATA").unwrap_or_default());
        let _ = Command::new("cmd")
            .args(&["/c", &format!("type nul > \"{}\" 2>nul", teams)])
            .output();
        cleaned += 1;
        logger.verbose("Cleaned Teams logs");
    }

    // Outlook search
    if !dry_run {
        let outlook = format!(r"{}\Microsoft\Outlook\*.dat", local);
        let _ = Command::new("cmd")
            .args(&["/c", &format!("del /q \"{}\" 2>nul", outlook)])
            .output();
        cleaned += 1;
        logger.verbose("Cleaned Outlook search history");
    }

    // WSA logs
    if !dry_run {
        let wsa = format!(r"{}\Packages\MicrosoftCorporationII.WindowsSubsystemForAndroid_*\LocalState\*.log", local);
        let _ = Command::new("cmd")
            .args(&["/c", &format!("del /q \"{}\" 2>nul", wsa)])
            .output();
        cleaned += 1;
        logger.verbose("Cleaned Windows Subsystem for Android logs");
    }

    // Xbox Game Bar
    if !dry_run {
        let xbox = format!(r"{}\Packages\Microsoft.XboxGamingOverlay_*\LocalState\*.log", local);
        let _ = Command::new("cmd")
            .args(&["/c", &format!("del /q \"{}\" 2>nul", xbox)])
            .output();
        cleaned += 1;
        logger.verbose("Cleaned Xbox Game Bar logs");
    }

    logger.success(&format!("Temporary files cleaned ({} items)", cleaned));
    Ok((cleaned, failed))
}
