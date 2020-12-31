use crate::context::Context;
use crate::output::Output;
use clap::Clap;

/// Creates a sauce file
#[derive(Clap, Debug)]
pub struct NewCommand {
    /// Print debug info
    #[clap(short)]
    debug: bool,
}

pub fn new(context: Context, _cmd: NewCommand, output: &mut Output) {
    let parent = context.sauce_path.parent().unwrap();
    if std::fs::create_dir_all(parent).is_err() {
        output.push_message(format!(
            "Couldn't create the thing {}",
            parent.to_string_lossy()
        ));
        return;
    }

    if context.sauce_path.is_file() {
        output.push_message(format!(
            "File already exists at {}",
            context.sauce_path.to_string_lossy()
        ));
    } else {
        if std::fs::File::create(context.sauce_path).is_err() {
            output.push_message("couldn't create the file");
        } else {
            output.push_message(format!("Created"));
        }
    }
}
