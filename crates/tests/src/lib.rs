#![cfg(test)]

use nawa::Rope;

fn to_vec<T: Clone>(r: &Rope<T>) -> Vec<T> {
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
#[should_panic(
  expected = "index out of bounds: the len is 3 but the index is 123"
)]
fn test_rope_bad() {
  let r = Rope::from(b"hey".to_vec());
  r.insert(123, b"nope".to_vec());
}
