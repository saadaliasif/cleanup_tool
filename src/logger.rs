use chrono;
use colored::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;

pub struct Logger {
    log_file: Option<Mutex<std::fs::File>>,
    debug: bool,
}

impl Logger {
    pub fn new(log_file: Option<String>, debug: bool) -> anyhow::Result<Self> {
        let file = log_file
            .map(|path| OpenOptions::new().create(true).append(true).open(path))
            .transpose()?;

        Ok(Self {
            log_file: file.map(Mutex::new),
            debug,
        })
    }

    pub fn banner(&self) {
        let sep = "=".repeat(48);
        println!("{}", sep.blue());
        println!("{}", "-------clean this-------".blue());
        println!("{}", sep.blue());
        println!();
    }

    pub fn warning(&self) {
        println!(
            "{}",
            "WARNING: This tool will permanently delete forensic traces if you use wipe".yellow()
        );
        println!(
            "{}",
            "This action cannot be undone and may impact system stability.".yellow()
        );
        println!();
    }

    pub fn error(&self, msg: &str) {
        let line = format!("[ERROR] {}", msg);
        println!("{}", line.red());
        self.write_log(&line);
    }

    pub fn success(&self, msg: &str) {
        let line = format!("[✓] {}", msg);
        println!("{}", line.green());
        self.write_log(&line);
    }

    pub fn info(&self, msg: &str) {
        let line = format!("[*] {}", msg);
        println!("{}", line.blue());
        self.write_log(&line);
    }

    pub fn verbose(&self, msg: &str) {
        if self.debug {
            let line = format!("[DEBUG] {}", msg);
            println!("{}", line);
            self.write_log(&line);
        }
    }

    fn write_log(&self, msg: &str) {
        if let Some(file) = &self.log_file {
            let ts = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
            let _ = file
                .lock()
                .unwrap()
                .write_all(format!("[{}] {}\n", ts, msg).as_bytes());
        }
    }
}
