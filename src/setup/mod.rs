mod htmx;
mod rust;
mod tailwind;

use crate::input::options::Input;
use color_eyre::Result;
use std::path::PathBuf;

pub fn setup_project(options: &Input) -> Result<()> {
    htmx::download_htmx(options)?;
    tailwind::cache_exe()?;
    rust::create_project(options)?;

    Ok(())
}

/// Returns the absolute path to app cache directory.
///
/// May return an error when system cache directory does not exist,
/// or when it can not create app specific directory.
///
/// | OS       | Example                            |
/// | -------- | ---------------------------------- |
/// | Linux    | /home/alice/.cache/NAME           |
/// | macOS    | /Users/Alice/Library/Caches/NAME  |
/// | Windows  | C:\Users\Alice\AppData\Local\NAME |
pub fn get_cache_dir() -> Result<PathBuf> {
    let dir = dirs::cache_dir().expect("Cache directory does not exist");

    if !dir.exists() {
        std::fs::create_dir_all(&dir).expect("Could not create directory");
    }

    Ok(dir)
}
