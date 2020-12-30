use crate::context::Context;
use crate::output::Output;
use anyhow::Result;
use clap::Clap;

/// Creates a sauce file
#[derive(Clap, Debug)]
pub struct NewCommand {
    /// Print debug info
    #[clap(short)]
    debug: bool,
}

pub fn new(context: Context, _cmd: NewCommand) -> Result<Output> {
    let parent = context.sauce_path.parent().unwrap();
    std::fs::create_dir_all(parent)?;

    if context.sauce_path.is_file() {
        Ok(Output::from_message(format!(
            "File already exists at {}",
            context.sauce_path.to_string_lossy()
        )))
    } else {
        std::fs::File::create(context.sauce_path)?;
        Ok(Output::from_message(format!("Created")))
    }
}
