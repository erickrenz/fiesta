use crate::input::options::Input;
use color_eyre::Result;
use reqwest;
use std::{fs::File, io};

const HTMX_URL: &str = "https://unpkg.com/htmx.org@1.9.5/dist/htmx.min.js";

#[tokio::main]
pub async fn download_htmx(options: &Input) -> Result<()> {
    let response = reqwest::get(HTMX_URL)
        .await
        .expect("failed to get htmx from cdn")
        .text()
        .await
        .expect("htmx from cdn is invalid");

    let mut path = options
        .path
        .clone()
        .expect("this should have been set up already");
    path.push("htmx.min.js");

    let mut new_file = File::create(path).expect("failed to create htmx file");
    io::copy(&mut response.as_bytes(), &mut new_file).expect("failed to copy content");

    Ok(())
}
