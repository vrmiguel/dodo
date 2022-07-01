pub use dodo_internals as dodo;
pub use error::{Error, Result};

use dodo::Priority;
use dodo::Task;
use file_ext::FileExt;
use files::Bookkeeper;
use formatting::DateBuffer;

mod error;
mod file_ext;
mod files;
mod formatting;

fn run() -> Result<()> {
    let today = dodo::utils::today();

    files::move_to_data_dir()?;

    // Check if there's already a task file for the current day
    let file = {
        let mut buf = DateBuffer::new();
        let path = buf.format_path(today)?;
        files::open_or_create(path)?
    };

    if file.is_empty()? {
        eprintln!("Creating initial file for {today}");
        // TODO: check the last registered file
        let mut bookkeeper = Bookkeeper::init()?;
        if bookkeeper.last_entry == today {
            // Clean slate: there are no tasks to move over to today!
            println!("adding sample task");
            let task = Task {
                name: String::new(),
                is_done: true,
                description: String::new(),
                creation_date: today,
                due_date: None,
                priority: Priority::High,
                checklist: vec![].into_iter().collect(),
            };
            bookkeeper.append_to_today(&[task])?;
        } else {
            // We'll move the pending tasks from the last entry over to
            // the current entry
            let tasks = bookkeeper.last_entry_tasks()?;
            bookkeeper.append_to_today(&tasks)?;
            dbg!(tasks);
        }
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("Error: {err}");
    }
}
