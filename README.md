# cleanup

> A post-operation forensic cleanup tool for red team engagements, rewritten from PowerShell to Rust.

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows-lightgrey.svg)](https://www.microsoft.com/windows)

## Overview

**cleanup** is a Windows forensic artifact cleanup tool designed for authorized red team operations and homelab testing. It systematically erases forensic traces left behind after an engagement, including event logs, registry MRUs, prefetch data, browser history, EDR logs, and more.

Originally written in PowerShell, this version is a complete Rust rewrite with:
- **Modular architecture** — run specific cleanup modules or all at once
- **Dry-run mode** — preview what would be deleted without making changes
- **Secure wipe** — overwrite files with random bytes before deletion
- **Colored terminal output** — clear visual feedback
- **Audit logging** — optional log file for operation review
- **Cross-compilation support** — build on Linux, deploy on Windows

## Disclaimer

**This tool is intended for authorized security testing, red team exercises, and personal homelab environments only.**

- Do **NOT** run this on production systems without explicit authorization
- Do **NOT** run this on systems you do not own or have written permission to test
- This tool **permanently deletes forensic data** — there is no undo
- The authors assume **no liability** for misuse or damage caused by this tool

## Features

| Module | Description |
|--------|-------------|
| `events` | Clears all Windows Event Logs (Application, System, Security, Sysmon, etc.) |
| `history` | Wipes PowerShell/CMD history, prefetch, jump lists, Edge browser data |
| `registry` | Deletes MRU keys, USB history (USBSTOR), BAM, ShimCache, RDP history |
| `filesystem` | Deletes USN journal, empties Recycle Bin, clears thumbcache, SRUDB |
| `temp` | Cleans temp files, flushes DNS cache, deletes shadow copies, WER archives |
| `security` | Removes EDR/AV logs (CrowdStrike, SentinelOne, Carbon Black, etc.) |
| `advanced` | Cleans test certificates, scheduled tasks, services, WiFi/VPN profiles |

### Flags

| Flag | Short | Description |
|------|-------|-------------|
| `--dry-run` | `-n` | Preview mode — no changes are made |
| `--wipe` | `-w` | Securely overwrite files with random bytes before deletion |
| `--advanced` | `-a` | Enable aggressive options (hiberfil removal, pagefile clearing) |
| `--modules` | `-m` | Comma-separated list of modules to run |
| `--force` | `-f` | Skip confirmation prompt |
| `--debug` | `-d` | Enable verbose debug output |
| `--log-file` | | Write audit log to specified file |
| `--list` | `-l` | List available modules |

## Building

### Prerequisites

- [Rust](https://rustup.rs/) 1.70 or later
- Windows target (for cross-compilation from Linux)

### Build on Windows

```bash
git clone https://github.com/yourusername/cleanup.git
cd cleanup
cargo build --release
```

### Cross-compile from Linux

```bash
# Install Windows target
rustup target add x86_64-pc-windows-gnu

# Install mingw (Debian/Ubuntu)
sudo apt install mingw-w64

# Build
cargo build --release --target x86_64-pc-windows-gnu
```

The binary will be at:
```
target/x86_64-pc-windows-gnu/release/cleanup.exe
```

## Usage

### List available modules
```cmd
cleanup.exe --list
```

### Dry run (preview what would be cleaned)
```cmd
cleanup.exe --dry-run --debug
```

### Run specific modules
```cmd
cleanup.exe --modules events,history --force
```

### Full cleanup with secure wipe
```cmd
cleanup.exe --force --wipe --advanced
```

### Run with audit logging
```cmd
cleanup.exe --force --log-file cleanup.log
```

## Architecture

```
src/
├── main.rs           # Entry point, CLI orchestration
├── cli.rs            # Clap argument definitions
├── logger.rs         # Colored output and file logging
├── utils.rs          # Safe file operations and secure wipe
├── privilege.rs      # Administrator privilege check
└── mods/
    ├── mod.rs        # Module dispatcher
    ├── events.rs     # Windows Event Log cleanup
    ├── history.rs    # User activity traces
    ├── registry.rs   # Registry MRU cleanup
    ├── filesystem.rs # USN journal, Recycle Bin, etc.
    ├── temp.rs       # Temp files, DNS, shadow copies
    ├── security.rs   # EDR/AV artifact cleanup
    └── advanced.rs   # Certificates, tasks, services
```

### Verification

After running cleanup, verify cleanup with:

```powershell
# Event logs
wevtutil qe Application /c:1

# DNS cache
ipconfig /displaydns

# Prefetch
Get-ChildItem C:\Windows\Prefetch\*.pf

# Shadow copies
vssadmin list shadows

# Registry MRUs
Get-ItemProperty "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\RecentDocs"
```

## Secure Wipe

When using `--wipe`, cleanup overwrites target files **3 times** with cryptographically random bytes, followed by a final zero pass, before deletion. This prevents recovery via standard file-carving techniques.

**Note:** Secure wipe operates at the file level. For volume-level free space wiping, use `cipher /w:C` after running cleanup.
