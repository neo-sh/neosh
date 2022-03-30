//! NeoSH files

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use tracing::debug;
use miette::{IntoDiagnostic, Result};

pub struct NeoshHistory {
    pub path: PathBuf,
}

impl NeoshHistory {
    pub fn init(&self) -> Result<()> {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(self.path.as_path())
            .into_diagnostic()?;

        Ok(())
    }

    pub fn get(&self) -> Result<String> {
        let history = fs::read_to_string(self.path.as_os_str()).into_diagnostic()?;
        Ok(history)
    }

    pub fn save(&self, contents: &str) -> Result<()> {
        let mut history_file = OpenOptions::new()
            .read(true)
            .append(true)
            .open(self.path.as_path())
            .into_diagnostic()?;

        // History file syntax:
        // exit_code: command
        writeln!(history_file, "{contents}").into_diagnostic()?;

        debug!("Appended '{contents}' to history");
        Ok(())
    }
}
