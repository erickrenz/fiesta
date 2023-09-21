pub mod input;
pub mod setup;

use crate::input::init;
use crate::input::options::Input;
use crate::setup::setup_project;
use color_eyre::Result;

pub fn initialize_app(opts: Input) -> Result<()> {
    let user_preferences = init(opts)?;

    setup_project(&user_preferences)?;

    Ok(())
}
