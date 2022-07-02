use std::fmt::{self, Display};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{Checklist, Priority};

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

impl AsRef<[Task]> for TaskSet {
    fn as_ref(&self) -> &[Task] {
        &self.0
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let is_done = if self.is_done { "x" } else { " " };
        let name = &self.name;
        let priority = self.priority;
        let checklist = &self.checklist;

        writeln!(f, "[{is_done}] {name} [{priority}]")?;
        writeln!(f, "{checklist}")
    }
}

/// Represents a to-do task
#[derive(
    Debug, Serialize, Deserialize, PartialEq, Eq, Clone,
)]
pub struct Task {
    /// This task's name
    pub name: String,
    /// Whether or not this task is done
    pub is_done: bool,
    /// When this task was created
    pub creation_date: NaiveDate,
    /// This tasks's due date, if any
    pub due_date: Option<NaiveDate>,
    /// This task's overall priority
    pub priority: Priority,
    /// This task's checklist
    pub checklist: Checklist,
}

impl PartialOrd for Task {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<std::cmp::Ordering> {
        let both_have_due_dates =
            self.due_date.is_some() && other.due_date.is_some();
        // If any of the tasks have no due date, then the
        // comparison is based on priority only
        if !both_have_due_dates
            || self.priority != other.priority
        {
            return self.priority.partial_cmp(&other.priority);
        }

        // We now know that both tasks have the same priority so
        // the comparison will be based on the due date
        // We also know that both tasks have a due date, so the
        // unwraps are safe.
        let self_due_date = &self.due_date.unwrap();
        let other_due_date = &other.due_date.unwrap();

        Some(other_due_date.cmp(self_due_date))
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::{Priority, Task};
    use crate::{utils::today, Checkbox};

    fn dummy_task() -> Task {
        Task {
            name: "Dummy".into(),
            is_done: false,
            creation_date: today(),
            due_date: None,
            priority: Priority::Low,
            checklist: vec![Checkbox::with_description(
                "Procurar metodologia".into(),
            )]
            .into_iter()
            .collect(),
        }
    }

    #[test]
    fn display() {
        println!("{t}", t = dummy_task());
    }

    #[test]
    fn task_ord_differing_priorities() {
        let mut task1 = dummy_task();
        let mut task2 = dummy_task();

        assert_eq!(task1.priority, Priority::Low);
        assert_eq!(task2.priority, Priority::Low);

        task1.priority = Priority::Medium;

        assert!(task1 > task2);

        task2.priority = Priority::High;

        assert!(task2 > task1);
    }

    #[test]
    fn task_ord_differing_due_dates() {
        let mut task1 = dummy_task();
        let mut task2 = dummy_task();

        let today = today();
        let a_day = Duration::days(1);
        let a_week = Duration::weeks(1);

        let tomorrow = today + a_day;
        let a_week_from_now = today + a_week;

        task1.due_date = Some(tomorrow);
        task2.due_date = Some(a_week_from_now);

        assert!(task1.priority == task2.priority);

        // Task no. 1 has the priority since it's closer to its
        // due date than task no. 2
        assert!(task1 > task2);

        task2.due_date = Some(today);

        // Task no. 2 now has the priority since it's closer to
        // its due date than task no. 1
        assert!(task1 < task2);
    }
}
