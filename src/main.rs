use clap::Parser;
use color_eyre::Result;
use fiesta::input::options::Config;

fn main() -> Result<()> {
    match Config::parse() {
        Config::Init(opts) => fiesta::initialize_app(opts)?,
    };

    Ok(())
}
