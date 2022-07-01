use std::{
    fmt::Display,
    iter::FromIterator,
    ops::{Deref, Index},
};

use serde::{Deserialize, Serialize};

use crate::Checkbox;

#[non_exhaustive]
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
/// A [`Checklist`](crate::Checklist) is a collection of [checkboxes](crate::Checkbox).
pub struct Checklist {
    checkboxes: Vec<Checkbox>,
}

impl Display for Checklist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for checkbox in &self.checkboxes {
            writeln!(f, "    * {}", checkbox)?;
        }

        Ok(())
    }
}

impl Checklist {
    /// Creates a checklist with the given checkboxes.
    /// This same operation may also be done more generically through [`FromIterator`](core::iter::FromIterator).
    pub fn with_checkboxes(checkboxes: Vec<Checkbox>) -> Self {
        Self { checkboxes }
    }

    /// Returns true if this checklist has no checkboxes attached.
    /// ```rust
    /// use dodo_internals::Checklist;
    /// let checklist = Checklist::with_checkboxes(vec![]);
    /// assert!(checklist.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.checkboxes.len() == 0
    }

    /// Returns the amount of checkboxes in this checklist
    /// ```
    /// use dodo_internals::{Checkbox, Checklist};
    /// let numbers = (0..=9).map(|x| x.to_string()).map(Checkbox::with_description);
    /// let mut checkboxes: Checklist = numbers.collect();
    /// assert_eq!(checkboxes.len(), 10);
    /// ```
    pub fn len(&self) -> usize {
        self.checkboxes.len()
    }

    /// Returns true if the all of the checkboxes in this checklist are done.
    /// ```rust
    /// use dodo_internals::{Checkbox, Checklist};
    /// let mut dishes = Checkbox::with_description("do the dishes".into());
    /// dishes.toggle();
    /// let mut cklist = Checklist::with_checkboxes(vec![dishes]);
    /// assert!(cklist.all_done());
    /// let calc = Checkbox::with_description("study for calculus".into());
    /// cklist.push(calc);
    /// assert!(!cklist.all_done());
    /// ```
    pub fn all_done(&self) -> bool {
        self.checkboxes.iter().all(Checkbox::is_done)
    }

    /// Returns a reference to an element given his index.
    /// Returns None if given an out-of-bounds index.
    pub fn get(&self, idx: usize) -> Option<&Checkbox> {
        if idx >= self.checkboxes.len() {
            None
        } else {
            Some(&self[idx])
        }
    }

    /// Returns a mutable reference to an element given his index.
    /// Returns None if given an out-of-bounds index.
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut Checkbox> {
        if idx >= self.checkboxes.len() {
            None
        } else {
            Some(&mut self.checkboxes[idx])
        }
    }

    /// Appends an element to the back of the collection.
    /// ```rust
    /// # use std::ops::Deref;
    /// # use dodo_internals::{Checkbox, Checklist};
    /// let mut checklist = Checklist::with_checkboxes(vec![]);
    /// assert_eq!(checklist.deref(), &[]);
    ///
    /// let checkbox = Checkbox::with_description("Trim your beard".into());
    /// checklist.push(checkbox.clone());
    /// assert_eq!(checklist.deref(), &[checkbox]);
    /// ```
    pub fn push(&mut self, elem: Checkbox) {
        self.checkboxes.push(elem);
    }

    /// Removes a checkbox from the list and returns it.
    /// Does not preserve ordering.
    /// Panics if `index` is out of bounds.
    pub fn remove(&mut self, index: usize) -> Checkbox {
        self.checkboxes.swap_remove(index)
    }
}

impl Deref for Checklist {
    type Target = [Checkbox];

    fn deref(&self) -> &Self::Target {
        &self.checkboxes
    }
}

impl Index<usize> for Checklist {
    type Output = Checkbox;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.checkboxes[idx]
    }
}

impl FromIterator<Checkbox> for Checklist {
    fn from_iter<T: IntoIterator<Item = Checkbox>>(iter: T) -> Self {
        let checkboxes: Vec<_> = iter.into_iter().collect();

        Checklist::with_checkboxes(checkboxes)
    }
}

impl IntoIterator for Checklist {
    type Item = Checkbox;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.checkboxes.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::checkbox::Checkbox;

    use super::Checklist;

    #[test]
    // Dumb test since we're essentially testing code from the stdlib
    // Still, it's ok since it ensures Checklist doesn't get his IntoIterator impl removed
    fn into_iter() {
        let numbers = (0..=9)
            .map(|x| x.to_string())
            .map(Checkbox::with_description);
        let checkboxes: Checklist = numbers.collect();

        for (idx, checkbox) in checkboxes.into_iter().enumerate() {
            assert_eq!(checkbox.description(), idx.to_string());
        }
    }

    #[test]
    fn get_and_get_mut() {
        let numbers = (0..=9)
            .map(|x| x.to_string())
            .map(Checkbox::with_description);

        let mut checkboxes: Checklist = numbers.collect();
        assert_eq!(checkboxes.len(), 10);
        assert!(checkboxes[4].description() == "4");
        assert!(checkboxes[9].description() == "9");

        assert!(!checkboxes[3].is_done());
        checkboxes.get_mut(3).unwrap().toggle();
        assert!(checkboxes[3].is_done());

        assert!(checkboxes.get(9).is_some());
        assert!(checkboxes.get(10).is_none());

        assert!(checkboxes.get_mut(9).is_some());
        assert!(checkboxes.get_mut(10).is_none());
    }
}
