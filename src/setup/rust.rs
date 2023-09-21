use crate::input::options::Input;
use color_eyre::Result;

pub fn create_project(options: &Input) -> Result<()> {
    println!("{options:?}");
    Ok(())
}
