//! Maintains the many files dodo generates

use std::{
    env,
    io::prelude::*,
    io::SeekFrom,
    io::{BufRead, BufReader},
    ops::Not,
    path::Path,
};

use fs_err as fs;
use fs_err::{File, OpenOptions};

use directories::ProjectDirs;
use dodo_internals::{chrono::NaiveDate, utils::today, Task, TaskSet};

use crate::formatting::{DateBuffer, FMT_STRING};
use crate::{Error, Result};

pub struct Bookkeeper {
    pub bookkeeping_file: File,
    pub last_entry: NaiveDate,
}

impl Bookkeeper {
    /// "Initializes" the bookkeeper by checking if there's a
    /// bookkeeping file and annotating what's the last
    /// date for which we have an entry.
    ///
    /// Assumes the process is currently in the project's data directory.
    pub fn init() -> Result<Self> {
        let mut bookkeeping_file = open_or_create("bookkeeper")?;

        let end_pos = bookkeeping_file.seek(SeekFrom::End(0))?;

        if end_pos == 0 {
            // Bookkeeping file is empty, therefore the latest date we
            // have an entry on is today
            Ok(Self {
                bookkeeping_file,
                last_entry: today(),
            })
        } else if end_pos % 11 == 0 {
            // All lines in the bookkeeping file must be 11 bytes long (10 bytes for the date and a newline)
            bookkeeping_file.seek(SeekFrom::Start(end_pos - 11))?;

            // This mustn't fail unless the bookkeeping file became malformed
            let line = first_line(&bookkeeping_file).expect("Malformed line in bookkeeping file");
            let date = NaiveDate::parse_from_str(&line, FMT_STRING)?;

            Ok(Self {
                bookkeeping_file,
                last_entry: date,
            })
        } else {
            // The bookkeeping file is somehow malformed
            Err(Error::InvalidBookkeepingFile)
        }
    }

    pub(crate) fn last_entry_taskset(&self) -> Result<TaskSet> {
        let mut buf = DateBuffer::new();

        let path = buf.format_path(self.last_entry)?;
        let file = File::open(path)?;

        bincode::deserialize_from(file).map_err(Into::into)
    }

    fn append_to_bookkeeping_file(&mut self, date: NaiveDate) -> Result<()> {
        let fmt = date.format(FMT_STRING);

        // TODO: check if it was already added in order to avoid adding repeated
        // entries
        writeln!(self.bookkeeping_file, "{fmt}").map_err(Into::into)
    }

    /// Appends the given entries to today's task list.
    ///
    /// After this, the current task list will be considered
    /// the last entry.
    pub fn append_to_today(&mut self, tasks: impl AsRef<[Task]>) -> Result<()> {
        let today = today();

        if self.last_entry != today {
            self.append_to_bookkeeping_file(today)?;
            self.last_entry = today;
        }

        let last_entry_file = {
            let mut buf = DateBuffer::new();
            let path = buf.format_path(self.last_entry)?;
            open_or_create(path)?
        };

        bincode::serialize_into(last_entry_file, tasks.as_ref())?;

        Ok(())
    }
}

pub fn open_or_create(path: impl AsRef<Path>) -> Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(false)
        .create(true)
        .open(path.as_ref())
        .map_err(Into::into)
}

/// Reads the first line from a file, if there's any
fn first_line(file: &File) -> Option<String> {
    BufReader::new(file)
        .lines()
        .next()
        .transpose()
        .ok()
        .flatten()
}

/// Moves the process' current directory to the project's data directory.
pub fn move_to_data_dir() -> Result<()> {
    let dirs = ProjectDirs::from("", "vrmiguel", "dodo").ok_or(Error::NoValidHomeDirFound)?;

    let data_dir_did_not_already_exist = dirs.data_dir().exists().not();

    fs::create_dir_all(dirs.data_dir())?;
    env::set_current_dir(dirs.data_dir())?;

    if data_dir_did_not_already_exist {
        let mut file = File::create("README")?;
        write!(file, "Please do not manually edit any files in this folder")?;
        println!("Data directory initial setup complete");
    }

    Ok(())
}
