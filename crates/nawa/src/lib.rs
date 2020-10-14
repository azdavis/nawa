//! A [rope][1] data structure.
//!
//! [1]: https://en.wikipedia.org/wiki/Rope_(data_structure)

#![deny(missing_docs)]

/// A rope data structure.
#[derive(Debug, Clone)]
pub struct Rope<T> {
  repr: Repr<T>,
}

impl<T> Rope<T> {
  /// Returns an empty `Rope`.
  ///
  /// Computes in O(1).
  #[allow(clippy::new_without_default)]
  pub fn new() -> Self {
    Self::of(Repr::new())
  }

  /// Returns the length of this `Rope`.
  ///
  /// Computes in O(1).
  ///
  /// # Examples
  ///
  /// ```
  /// use nawa::Rope;
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
  /// use nawa::Rope;
  ///
  /// let r: Rope<i32> = Rope::new();
  /// assert!(r.is_empty());
  /// let r = Rope::from(vec![2, 4, 6]);
  /// assert!(!r.is_empty());
  /// ```
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// Inserts `xs` into the `Rope` starting at `i`.
  ///
  /// Panics iff `i > len`.
  ///
  /// # Examples
  ///
  /// ```
  /// use nawa::Rope;
  ///
  /// let r = Rope::from(vec![2, 4]);
  /// assert_eq!(r.to_vec(), [&2, &4]);
  /// let r = r.insert(1, vec![3, 5]);
  /// assert_eq!(r.to_vec(), [&2, &3, &5, &4]);
  /// ```
  pub fn insert(self, i: usize, xs: Vec<T>) -> Self {
    let (a, c) = self.repr.split(i);
    let b = Repr::Leaf(xs);
    Self::of(Repr::node(a, Repr::node(b, c)))
  }

  /// Removes the `range` from this `Rope`.
  ///
  /// Panics iff the range is out of bounds.
  ///
  /// # Examples
  ///
  /// ```
  /// use nawa::Rope;
  ///
  /// let r = Rope::from(vec![2, 4, 6, 8]);
  /// let r = r.remove(1..3);
  /// assert_eq!(r.to_vec(), [&2, &8]);
  /// ```
  pub fn remove(self, range: std::ops::Range<usize>) -> Self {
    assert!(range.start <= range.end);
    let (a, b) = self.repr.split(range.start);
    let (_, d) = b.split(range.end - range.start);
    Self::of(Repr::node(a, d))
  }

  /// Returns the `Vec` represented by this `Rope`.
  ///
  /// Computes in O(n).
  ///
  /// # Examples
  ///
  /// ```
  /// use nawa::Rope;
  ///
  /// let r = Rope::from(vec![2, 4, 6, 8]);
  /// assert_eq!(r.to_vec(), [&2, &4, &6, &8]);
  /// ```
  pub fn to_vec(&self) -> Vec<&T> {
    self.repr.to_vec()
  }

  #[inline(always)]
  fn of(repr: Repr<T>) -> Self {
    Self { repr }
  }
}

impl<T: PartialEq> PartialEq for Rope<T> {
  fn eq(&self, other: &Rope<T>) -> bool {
    self.to_vec() == other.to_vec()
  }
}

impl<T: Eq> Eq for Rope<T> {}

impl<T: PartialOrd> PartialOrd for Rope<T> {
  fn partial_cmp(&self, other: &Rope<T>) -> Option<std::cmp::Ordering> {
    self.to_vec().partial_cmp(&other.to_vec())
  }
}

impl<T: Ord> Ord for Rope<T> {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.to_vec().cmp(&other.to_vec())
  }
}

impl<T> From<Vec<T>> for Rope<T> {
  fn from(val: Vec<T>) -> Self {
    Self::of(Repr::Leaf(val))
  }
}

enum Direction {
  Left,
  Right,
}

#[derive(Debug, Clone)]
enum Repr<T> {
  Leaf(Vec<T>),
  Node(Box<Repr<T>>, usize, Box<Repr<T>>),
}

impl<T> Repr<T> {
  fn new() -> Self {
    Self::Leaf(Vec::new())
  }

  fn len(&self) -> usize {
    match *self {
      Repr::Leaf(ref xs) => xs.len(),
      Repr::Node(_, len, _) => len,
    }
  }

  fn node(left: Self, right: Self) -> Self {
    match (left.len(), right.len()) {
      (0, _) => right,
      (_, 0) => left,
      (a, b) => Self::Node(left.into(), a + b, right.into()),
    }
  }

  fn split(self, mut i: usize) -> (Self, Self) {
    if i > self.len() {
      panic!(
        "index out of bounds: the len is {} but the index is {}",
        self.len(),
        i
      );
    }
    let mut work = Vec::with_capacity(1);
    let mut right = self;
    let mut left = loop {
      match right {
        Repr::Leaf(mut xs) => {
          right = Repr::Leaf(xs.split_off(i));
          break Repr::Leaf(xs);
        }
        Repr::Node(node_l, _, node_r) => {
          if i < node_l.len() {
            work.push((*node_r, Direction::Right));
            right = *node_l;
          } else {
            i -= node_l.len();
            work.push((*node_l, Direction::Left));
            right = *node_r;
          }
        }
      }
    };
    for (repr, dir) in work.into_iter().rev() {
      match dir {
        Direction::Left => left = Repr::node(repr, left),
        Direction::Right => right = Repr::node(right, repr),
      }
    }
    (left, right)
  }

  fn to_vec(&self) -> Vec<&T> {
    let mut ret = Vec::with_capacity(self.len());
    let mut work = Vec::with_capacity(1);
    let mut this = self;
    loop {
      match this {
        Repr::Leaf(xs) => {
          ret.extend(xs.iter());
          match work.pop() {
            None => break,
            Some(right) => this = right,
          }
        }
        Repr::Node(left, _, right) => {
          work.push(right);
          this = left;
        }
      }
    }
    ret
  }
}
