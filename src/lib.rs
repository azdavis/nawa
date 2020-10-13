//! A [rope][1] data structure.
//!
//! [1]: https://en.wikipedia.org/wiki/Rope_(data_structure)

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
  pub fn len(&self) -> usize {
    self.repr.len()
  }

  /// Returns whether this `Rope` is empty.
  ///
  /// Computes in O(1).
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// Inserts `xs` into the `Rope` starting at `i`.
  ///
  /// Panics iff `i > len`.
  pub fn insert(self, i: usize, xs: Vec<T>) -> Self {
    let (a, c) = self.repr.split(i);
    let b = Repr::Leaf(xs);
    Self::of(Repr::node(a, Repr::node(b, c)))
  }

  /// Deletes the `range` from this `Rope`.
  ///
  /// Panics iff:
  /// - `range.start > range.end`
  /// - `range.start >= len`
  /// - `range.end > len`
  pub fn delete(self, range: std::ops::Range<usize>) -> Self {
    assert!(range.start <= range.end);
    let (a, b) = self.repr.split(range.start);
    let (_, d) = b.split(range.end - range.start);
    Self::of(Repr::node(a, d))
  }

  /// Returns the `Vec` represented by this `Rope`.
  ///
  /// Computes in O(n).
  pub fn to_vec(&self) -> Vec<&T> {
    self.repr.to_vec()
  }

  #[inline(always)]
  fn of(repr: Repr<T>) -> Self {
    Self { repr }
  }
}

impl<T> PartialEq for Rope<T>
where
  T: PartialEq,
{
  fn eq(&self, other: &Rope<T>) -> bool {
    self.to_vec() == other.to_vec()
  }
}

impl<T> Eq for Rope<T> where T: Eq {}

impl<T> PartialOrd for Rope<T>
where
  T: PartialOrd,
{
  fn partial_cmp(&self, other: &Rope<T>) -> Option<std::cmp::Ordering> {
    self.to_vec().partial_cmp(&other.to_vec())
  }
}

impl<T> Ord for Rope<T>
where
  T: Ord,
{
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
    loop {
      match right {
        Repr::Leaf(mut xs) => {
          right = Repr::Leaf(xs.split_off(i));
          work.push((Repr::Leaf(xs), Direction::Left));
          break;
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
    }
    let mut left = Repr::new();
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
            Some(x) => this = x,
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

#[cfg(test)]
fn to_vec<T>(r: &Rope<T>) -> Vec<T>
where
  T: Clone,
{
  r.to_vec().into_iter().cloned().collect()
}

#[test]
fn test_rope_good() {
  let r = Rope::from(b"break".to_vec());

  assert_eq!(r.len(), 5);
  assert!(!r.is_empty());
  assert_eq!(to_vec(&r), b"break");

  let r = r.insert(5, b"fast".to_vec());

  assert_eq!(r.len(), 9);
  assert!(!r.is_empty());
  assert_eq!(to_vec(&r), b"breakfast");

  let r = r.delete(3..8);

  assert_eq!(to_vec(&r), b"bret");
  assert_eq!(r.len(), 4);
  assert!(!r.is_empty());

  let r = r.delete(1..2);

  assert_eq!(to_vec(&r), b"bet");
  assert_eq!(r.len(), 3);
  assert!(!r.is_empty());

  let r = r.insert(3, b"".to_vec());

  assert_eq!(to_vec(&r), b"bet");
  assert_eq!(r.len(), 3);
  assert!(!r.is_empty());

  let r = r.insert(3, b"ter".to_vec());

  assert_eq!(to_vec(&r), b"better");
  assert_eq!(r.len(), 6);
  assert!(!r.is_empty());

  let r = r.delete(0..6);

  assert_eq!(to_vec(&r), b"");
  assert_eq!(r.len(), 0);
  assert!(r.is_empty());
}

#[test]
#[should_panic(
  expected = "index out of bounds: the len is 3 but the index is 123"
)]
fn test_rope_bad() {
  let r = Rope::from(b"hey".to_vec());
  r.insert(123, b"nope".to_vec());
}
