//! A naive implementation of a [rope][1] with the same API as nawa's.
//!
//! [1]: https://en.wikipedia.org/wiki/Rope_(data_structure)

#![deny(missing_docs)]

/// A naive implementation of a rope with the same API as nawa's.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rope<T> {
  repr: Vec<T>,
}

impl<T> Rope<T> {
  /// Returns an empty `Rope`.
  ///
  /// Computes in O(1).
  #[allow(clippy::new_without_default)]
  pub fn new() -> Self {
    Self { repr: Vec::new() }
  }

  /// Returns the length of this `Rope`.
  ///
  /// Computes in O(1).
  ///
  /// # Examples
  ///
  /// ```
  /// use naive::Rope;
  ///
  /// let r: Rope<i32> = Rope::new();
  /// assert_eq!(r.len(), 0);
  /// let r = Rope::from(vec![2, 4, 6]);
  /// assert_eq!(r.len(), 3);
  /// ```
  pub fn len(&self) -> usize {
    self.repr.len()
  }

  /// Returns whether this `Rope` is empty.
  ///
  /// Computes in O(1).
  ///
  /// # Examples
  ///
  /// ```
  /// use naive::Rope;
  ///
  /// let r: Rope<i32> = Rope::new();
  /// assert!(r.is_empty());
  /// let r = Rope::from(vec![2, 4, 6]);
  /// assert!(!r.is_empty());
  /// ```
  pub fn is_empty(&self) -> bool {
    self.repr.is_empty()
  }

  /// Inserts `xs` into the `Rope` starting at `i`.
  ///
  /// Panics iff `i > len`.
  ///
  /// # Examples
  ///
  /// ```
  /// use naive::Rope;
  ///
  /// let r = Rope::from(vec![2, 4]);
  /// assert_eq!(r.to_vec(), [&2, &4]);
  /// let r = r.insert(1, vec![3]);
  /// assert_eq!(r.to_vec(), [&2, &3, &4]);
  /// ```
  pub fn insert(mut self, i: usize, mut xs: Vec<T>) -> Self {
    let mut end = self.repr.split_off(i);
    self.repr.append(&mut xs);
    self.repr.append(&mut end);
    self
  }

  /// Removes the `range` from this `Rope`.
  ///
  /// Panics iff the range is out of bounds.
  ///
  /// # Examples
  ///
  /// ```
  /// use naive::Rope;
  ///
  /// let r = Rope::from(vec![2, 4, 6, 8]);
  /// let r = r.remove(1..3);
  /// assert_eq!(r.to_vec(), [&2, &8]);
  /// ```
  pub fn remove(self, range: std::ops::Range<usize>) -> Self {
    assert!(range.start <= range.end);
    let repr: Vec<_> = self
      .repr
      .into_iter()
      .enumerate()
      .filter_map(|(i, x)| if range.contains(&i) { None } else { Some(x) })
      .collect();
    Self { repr }
  }

  /// Returns the `Vec` represented by this `Rope`.
  ///
  /// Computes in O(n).
  ///
  /// # Examples
  ///
  /// ```
  /// use naive::Rope;
  ///
  /// let r = Rope::from(vec![2, 4, 6, 8]);
  /// assert_eq!(r.to_vec(), [&2, &4, &6, &8]);
  /// ```
  pub fn to_vec(&self) -> Vec<&T> {
    self.repr.iter().collect()
  }
}

impl<T> From<Vec<T>> for Rope<T> {
  fn from(repr: Vec<T>) -> Self {
    Self { repr }
  }
}
