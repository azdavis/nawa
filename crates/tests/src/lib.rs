//! Randomly-generated tests for ropes.

#![cfg(test)]

use getrandom::getrandom;
use oorandom::Rand32;
use std::convert::TryFrom as _;

fn get_seed() -> u64 {
  let mut buf = [0u8; 8];
  getrandom(&mut buf).unwrap();
  u64::from_ne_bytes(buf)
}

fn rand_usize(rand: &mut Rand32, range: std::ops::Range<usize>) -> usize {
  let start = u32::try_from(range.start).unwrap();
  let end = u32::try_from(range.end).unwrap();
  usize::try_from(rand.rand_range(start..end)).unwrap()
}

#[test]
fn random() {
  let seed = get_seed();
  println!("seed: {}", seed);
  let mut rand = Rand32::new(seed);
  let mut naive = naive::Rope::<u32>::new();
  let mut nawa = nawa::Rope::<u32>::new();
  for _ in 0..100000 {
    match rand.rand_range(0..2) {
      0 => {
        let mut xs = vec![0u32; rand_usize(&mut rand, 0..50)];
        for x in xs.iter_mut() {
          *x = rand.rand_range(100..1000);
        }
        let i = rand_usize(&mut rand, 0..(naive.len() + 1));
        naive = naive.insert(i, xs.clone());
        nawa = nawa.insert(i, xs);
      }
      1 => {
        if naive.is_empty() {
          continue;
        }
        let start = rand_usize(&mut rand, 0..naive.len());
        let end = rand_usize(&mut rand, (start + 1)..(naive.len() + 1));
        naive = naive.remove(start..end);
        nawa = nawa.remove(start..end);
      }
      _ => unreachable!(),
    }
    assert_eq!(naive.len(), nawa.len());
    assert_eq!(naive.is_empty(), nawa.is_empty());
    assert_eq!(naive.to_vec(), nawa.to_vec());
  }
}
