use super::options::{Input, Template};
use color_eyre::Result;
use std::{path::PathBuf, str::FromStr};

pub fn select_name(options: &mut Input) -> Result<()> {
    if options.name.is_some() {
        return Ok(());
    }

    println!("What is the project name?");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    options.name = match input.trim().is_empty() {
        true => Some(String::from("fiesta")),
        false => Some(input.trim().to_string()),
    };

    Ok(())
}

pub fn select_path(options: &mut Input) -> Result<()> {
    if options.path.is_some() {
        return Ok(());
    }

    loop {
        println!("What is the directory path?  (Default = '.')");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let input: &str = match input.trim().is_empty() {
            true => ".",
            false => input.trim(),
        };

        match PathBuf::from_str(input) {
            Ok(path) => {
                options.path = Some(path);
                break;
            }
            Err(_) => println!("Invalid Path!"),
        }
    }

    Ok(())
}

impl FromStr for Template {
    type Err = color_eyre::eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "cloudflare" | "c" => Ok(Template::Cloudflare),
            "openapi" | "o" => Ok(Template::OpenAPI),
            "none" | "n" => Ok(Template::None),
            _ => Err(color_eyre::eyre::eyre!("Invalid backend")),
        }
    }
}

pub fn select_backend(options: &mut Input) -> Result<()> {
    if options.template.is_some() {
        return Ok(());
    }

    println!("Choose a template:\n  [C]loudflare\n  [O]penAPI\n  [N]one (Default)");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    options.template = match input.trim().parse::<Template>() {
        Ok(back) => Some(back),
        Err(_) => Some(Template::None),
    };

    Ok(())
}
