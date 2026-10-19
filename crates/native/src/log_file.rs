//! The launcher's log file: `<data>/logs/redrob-launcher.log` (on Windows
//! `%LOCALAPPDATA%\Redrob Launcher\logs`). A windowed build has no console, so without this file
//! a failed install leaves only the one-line message in the window.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::OnceLock;

use crate::paths::Paths;

/// The file starts over (keeping one `.old` copy) once it passes this size.
const MAX_BYTES: u64 = 5 * 1024 * 1024;

static PATH: OnceLock<Option<PathBuf>> = OnceLock::new();

/// Where the log is written, once [`open`] has run; `None` when it couldn't be opened.
pub fn path() -> Option<PathBuf> {
    PATH.get().cloned().flatten()
}

/// Opens the log file for appending. `None` when there's no data folder or it can't be written.
pub fn open() -> Option<File> {
    let file = (|| {
        let dir = Paths::new().ok()?.data.join("logs");
        fs::create_dir_all(&dir).ok()?;
        let path = dir.join("redrob-launcher.log");
        if fs::metadata(&path).is_ok_and(|m| m.len() > MAX_BYTES) {
            let _ = fs::rename(&path, dir.join("redrob-launcher.log.old"));
        }
        let file = OpenOptions::new().create(true).append(true).open(&path).ok()?;
        Some((path, file))
    })();
    let _ = PATH.set(file.as_ref().map(|(p, _)| p.clone()));
    file.map(|(_, f)| f)
}

/// Writes every log line to stderr and, when it opened, the log file.
pub struct Tee(pub Option<File>);

impl Write for Tee {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let _ = io::stderr().write_all(buf);
        if let Some(f) = &mut self.0 {
            let _ = f.write_all(buf);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        let _ = io::stderr().flush();
        if let Some(f) = &mut self.0 {
            let _ = f.flush();
        }
        Ok(())
    }
}
