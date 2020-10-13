//! One-off tests.

use crate::rope_trait::Rope;

fn to_vec<R, T>(r: &R) -> Vec<T>
where
  R: Rope<T>,
  T: Clone,
{
  r.to_vec().into_iter().cloned().collect()
}

fn test_rope_good<R>()
where
  R: Rope<u8>,
  Vec<u8>: Into<R>,
{
  let r: R = b"break".to_vec().into();

  assert_eq!(r.len(), 5);
  assert!(!r.is_empty());
  assert_eq!(to_vec(&r), b"break");

  let r = r.insert(5, b"fast".to_vec());

  assert_eq!(r.len(), 9);
  assert!(!r.is_empty());
  assert_eq!(to_vec(&r), b"breakfast");

  let r = r.remove(3..8);

  assert_eq!(to_vec(&r), b"bret");
  assert_eq!(r.len(), 4);
  assert!(!r.is_empty());

  let r = r.remove(1..2);

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

  let r = r.remove(0..6);

  assert_eq!(to_vec(&r), b"");
  assert_eq!(r.len(), 0);
  assert!(r.is_empty());
}

#[test]
fn test_nawa_good() {
  test_rope_good::<nawa::Rope<u8>>()
}

#[test]
fn test_naive_good() {
  test_rope_good::<naive::Rope<u8>>()
}

fn test_rope_bad<R>()
where
  R: Rope<u8>,
  Vec<u8>: Into<R>,
{
  let r: R = b"hey".to_vec().into();
  r.insert(123, b"nope".to_vec());
}

#[test]
#[should_panic(
  expected = "index out of bounds: the len is 3 but the index is 123"
)]
fn test_nawa_bad() {
  test_rope_bad::<nawa::Rope<u8>>()
}

#[test]
#[should_panic(expected = "`at` split index (is 123) should be <= len (is 3)")]
fn test_naive_bad() {
  test_rope_bad::<naive::Rope<u8>>()
}
