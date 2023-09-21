pub mod options;
mod prompt;

use color_eyre::Result;
use options::Input;

pub fn init(mut options: Input) -> Result<Input> {
    prompt::select_name(&mut options)?;
    prompt::select_path(&mut options)?;
    prompt::select_backend(&mut options)?;

    Ok(options)
}
