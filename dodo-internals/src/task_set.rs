use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use crate::Task;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[repr(transparent)]
pub struct TaskSet(pub Vec<Task>);

impl Display for TaskSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, task) in self.0.iter().enumerate() {
            writeln!(f, "{}. {}", i + 1, task)?;
        }

        Ok(())
    }
}

impl TaskSet {
    /// Returns the first incorrect index (starting at 1) in this
    /// taskset, if there's any
    pub fn check_for_invalid_indices(&self) -> Option<usize> {
        for (idx, task) in self.0.iter().enumerate() {
            let expected_idx = idx + 1;
            if expected_idx != task.idx {
                eprintln!("Error: Expected index {expected_idx}, received {t}", t=task.idx);
                return Some(expected_idx);
            }
        }

        None
    }
}

impl AsRef<[Task]> for TaskSet {
    fn as_ref(&self) -> &[Task] {
        &self.0
    }
}
