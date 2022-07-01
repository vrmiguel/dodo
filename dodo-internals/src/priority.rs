use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

/// Represents a task's priority.
///
/// ```rust
/// # use dodo_internals::Priority;
/// assert!(Priority::High > Priority::Low);
/// assert!(Priority::Medium > Priority::Low);
/// assert!(Priority::High > Priority::Medium);
/// ```
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy)]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Priority::*;

        match (self, other) {
            (x, y) if x == y => Some(Ordering::Equal),
            (High, _) => Some(Ordering::Greater),
            (Medium, High) => Some(Ordering::Less),
            (Medium, _) => Some(Ordering::Greater),
            (Low, _) => Some(Ordering::Less),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn priority_cmp() {
        use super::Priority::*;

        assert!(High > Medium);
        assert!(High > Low);

        assert!(Medium > Low);
        assert!(Medium < High);

        assert!(Low < High);
        assert!(Low < Medium);

        assert!(High >= High);
        assert!(Medium >= Medium);
    }
}
