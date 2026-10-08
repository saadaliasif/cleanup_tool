use anyhow::{bail, Result};

pub fn check_admin() -> Result<()> {
    #[cfg(windows)]
    {
        use std::process::Command;
        let out = Command::new("whoami").args(&["/groups"]).output()?;
        let stdout = String::from_utf8_lossy(&out.stdout);

        if !stdout.contains("S-1-16-12288") {
            bail!("This tool must be run as Administrator");
        }
    }

    #[cfg(not(windows))]
    {
        bail!("this tool is designed for Windows targets only");
    }

    Ok(())
}
