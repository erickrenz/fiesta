use super::get_cache_dir;
use color_eyre::{eyre::eyre, Result};
use reqwest;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

const TAILWIND_VERSION: &str = "v3.3.3";
const BASE_URL: &str = "https://github.com/tailwindlabs/tailwindcss/releases/download";

#[tokio::main]
pub async fn cache_exe() -> Result<()> {
    let mut cache_dir: PathBuf = get_cache_dir()?;
    let exe_name: String = excecutable_name()?;

    if cache_dir.join(&exe_name).exists() {
        return Ok(());
    }

    let github_url: String = format!("{BASE_URL}/{TAILWIND_VERSION}/{exe_name}");
    let response = reqwest::get(github_url)
        .await
        .expect("failed to get tailwind from github")
        .bytes()
        .await
        .expect("tailwind exe from github is invalid");

    cache_dir.push(&exe_name);

    let mut new_file = File::create(cache_dir).expect("failed to create tailwind file");
    new_file
        .write_all(&response)
        .expect("failed to copy content");

    #[cfg(target_family = "unix")]
    {
        use std::os::unix::fs::PermissionsExt;

        let mut path = get_cache_dir()?;
        path.push(&exe_name);

        let mut permissions = new_file.metadata()?.permissions();
        permissions.set_mode(0o550);

        fs::set_permissions(path.as_path(), permissions)?;
    }

    Ok(())
}

fn excecutable_name() -> Result<String> {
    let target_os: &str = if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        return Err(eyre!("unsupported OS"));
    };

    let target_arch: &str = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else if cfg!(target_arch = "armv7") {
        "armv7"
    } else {
        return Err(eyre!("unsupported target architecture"));
    };

    let exe_name: &str = match (target_os, target_arch) {
        ("windows", "x86_64") => "tailwindcss-windows-x64.exe",
        ("windows", "aarch64") => "tailwindcss-windows-arm64.exe",
        ("macos", "x86_64") => "tailwindcss-macos-x64",
        ("macos", "aarch64") => "tailwindcss-macos-arm64",
        ("linux", "x86_64") => "tailwindcss-linux-x64",
        ("linux", "aarch64") => "tailwindcss-linux-arm64",
        ("linux", "armv7") => "tailwindcss-linux-armv7",
        _ => {
            return Err(eyre!(
                "Tailwindcss failed to find a match for {target_os}-{target_arch}."
            ))
        }
    };

    Ok(exe_name.to_string())
}
