use std::{fmt::Display, ops::Deref};

use serde::{Deserialize, Serialize};

/// A checkbox that belogns to a checklist.
/// Can be turned on or off and has a description.
#[non_exhaustive]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
// TODO: allow a specific checkbox to have a due-date?
pub struct Checkbox {
    description: String,
    is_done: bool,
}

impl Display for Checkbox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let is_done = if self.is_done { "x" } else { " " };
        let desc = &self.description;

        write!(f, "[{is_done}] {desc}")
    }
}

impl Checkbox {
    /// Creates an unchecked checkbox with the given description.
    pub fn with_description(name: String) -> Self {
        Self {
            description: name,
            is_done: false,
        }
    }

    /// The description of this checkbox
    /// ```rust
    /// use dodo_internals::Checkbox;
    /// let take_out_the_trash = Checkbox::with_description("Take out the trash".into());
    /// assert_eq!(take_out_the_trash.description(), "Take out the trash");
    /// ```
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Toggles a checkbox on or off (in other words, done or not done).
    /// ```rust
    /// use dodo_internals::Checkbox;
    /// let mut item = Checkbox::with_description("some item".into());
    /// assert!(!item.is_done());
    /// item.toggle();
    /// assert!(item.is_done());
    /// item.toggle();
    /// assert!(!item.is_done());
    /// ```
    pub fn toggle(&mut self) {
        self.is_done = !self.is_done;
    }

    /// Returns true if this checkbox is marked as done
    /// ```rust
    /// use dodo_internals::Checkbox;
    /// let mut item = Checkbox::with_description("some item".into());
    /// item.toggle();
    /// assert!(item.is_done());
    /// ```
    pub fn is_done(&self) -> bool {
        self.is_done
    }
}

impl Deref for Checkbox {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.is_done
    }
}
