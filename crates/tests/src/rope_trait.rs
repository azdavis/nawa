//! A trait for ropes.

pub trait Rope<T> {
  fn new() -> Self;
  fn len(&self) -> usize;
  fn is_empty(&self) -> bool;
  fn insert(self, i: usize, xs: Vec<T>) -> Self;
  fn remove(self, range: std::ops::Range<usize>) -> Self;
  fn to_vec(&self) -> Vec<&T>;
}

impl<T> Rope<T> for nawa::Rope<T> {
  fn new() -> Self {
    Self::new()
  }

  fn len(&self) -> usize {
    self.len()
  }

  fn is_empty(&self) -> bool {
    self.is_empty()
  }

  fn insert(self, i: usize, xs: Vec<T>) -> Self {
    self.insert(i, xs)
  }

  fn remove(self, range: std::ops::Range<usize>) -> Self {
    self.remove(range)
  }

  fn to_vec(&self) -> Vec<&T> {
    self.to_vec()
  }
}

impl<T> Rope<T> for naive::Rope<T> {
  fn new() -> Self {
    Self::new()
  }

  fn len(&self) -> usize {
    self.len()
  }

  fn is_empty(&self) -> bool {
    self.is_empty()
  }

  fn insert(self, i: usize, xs: Vec<T>) -> Self {
    self.insert(i, xs)
  }

  fn remove(self, range: std::ops::Range<usize>) -> Self {
    self.remove(range)
  }

  fn to_vec(&self) -> Vec<&T> {
    self.to_vec()
  }
}
