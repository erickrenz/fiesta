use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(ValueEnum, Debug, Parser, Clone)]
pub enum Template {
    Cloudflare,
    OpenAPI,
    None,
}

/// Initialize a new Rust powered HTMX application
#[derive(Debug, Parser, Clone)]
pub struct Input {
    /// Name for the new project
    #[arg(short, long)]
    pub name: Option<String>,

    /// Path to the new project directory
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Template for the default project files
    #[arg(short, long)]
    pub template: Option<Template>,
}

/// A simple program for Rust powered HTMX applications
#[derive(Debug, Parser)]
#[command(version)]
pub enum Config {
    Init(Input),
}
