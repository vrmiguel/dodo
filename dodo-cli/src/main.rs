pub use dodo_internals as dodo;
pub use error::{Error, Result};

// use dodo::{Priority, Task};
use file_ext::FileExt;
use files::Bookkeeper;
use formatting::DateBuffer;

mod error;
mod file_ext;
mod files;
mod formatting;

fn run() -> Result<()> {
    let mut date_buf = DateBuffer::new();
    let today = dodo::utils::today();

    files::move_to_data_dir()?;

    // Check if there's already a task file for the current day
    let file = {
        let path = date_buf.format_path(today)?;
        files::open_or_create(path)?
    };

    if file.is_empty()? {
        eprintln!("Creating initial file for {today}");
        // TODO: check yesterday's file
        Bookkeeper::init()?;
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("Error: {err}");
    }
}
