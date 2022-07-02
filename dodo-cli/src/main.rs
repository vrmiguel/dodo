use dodo::{utils::today, Checkbox, Priority, Task};
pub use dodo_internals as dodo;
pub use error::{Error, Result};
use file_ext::FileExt;
use files::Bookkeeper;
use formatting::DateBuffer;
use parser::Parser;

mod error;
mod file_ext;
mod files;
mod formatting;
mod parser;

fn run() -> Result<()> {
    let today = dodo::utils::today();

    files::move_to_data_dir()?;

    // Check if there's already a task file for the current day
    let file = {
        let mut buf = DateBuffer::new();
        let path = buf.format_path(today)?;
        files::open_or_create(path)?
    };

    let mut bookkeeper = Bookkeeper::init()?;

    if file.is_empty()? {
        eprintln!("Creating initial file for {today}");
        if bookkeeper.last_entry == today {
            // Clean slate: there are no tasks to move over to
            // today!
            println!("Adding a sample task");
            bookkeeper.append_to_today(&[sample_task()])?;
        } else {
            // We'll move the pending tasks from the last entry
            // over to the current entry
            let tasks = bookkeeper.last_entry_taskset()?;
            bookkeeper.append_to_today(&tasks)?;
            println!("{tasks}");
        }
    }

    // Get the current task set
    let task_set = bookkeeper.last_entry_taskset()?;
    // Let the user edit the task set as he sees fit
    let edited_text = edit::edit(task_set.to_string())?;

    let edited_tasks = Parser::parse(&edited_text)?;

    println!("{edited_tasks}");

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("Error: {err}");
    }
}

pub fn sample_task() -> Task {
    Task {
        name: "Fill out my tasks".into(),
        is_done: false,
        creation_date: today(),
        due_date: None,
        priority: Priority::High,
        checklist: vec![Checkbox::with_description(
            "Figure out how to use dodo".into(),
        )]
        .into_iter()
        .collect(),
    }
}
