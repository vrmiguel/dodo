use std::path::PathBuf;

use dodo_internals::chrono;
use thiserror::Error as ErrorMacro;

#[derive(ErrorMacro, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("IO: `{0}`")]
    Io(#[from] std::io::Error),
    #[error("Bincode: `{0}`")]
    Bincode(#[from] bincode::Error),
    #[error("No valid home directory was found")]
    NoValidHomeDirFound,
    #[error("Could not create folder `{0}`")]
    CouldNotCreateFolder(PathBuf),
    #[error("The bookkeeping file is invalid")]
    InvalidBookkeepingFile,
    #[error("Date parsing: {0}")]
    Chrono(#[from] chrono::ParseError),
}

pub type Result<T> = std::result::Result<T, Error>;
