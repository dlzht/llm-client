use std::{
  ops::Add,
  sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
  },
};

use serde::{Deserialize, Serialize};

pub trait TokenCounter {
  fn incr_bytes(&mut self, bytes: usize);

  fn incr_token(&mut self, token: usize);

  fn incr_both(&mut self, bytes: usize, token: usize) {
    self.incr_bytes(bytes);
    self.incr_token(token);
  }

  fn fetch_bytes(&self) -> usize;

  fn fetch_token(&self) -> usize;

  fn fetch_both(&self) -> (usize, usize) {
    (self.fetch_bytes(), self.fetch_token())
  }
}

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct SimpleCounter {
  bytes: usize,
  token: usize,
}

impl TokenCounter for SimpleCounter {
  fn incr_bytes(&mut self, bytes: usize) {
    self.bytes = self.bytes.saturating_add(bytes);
  }

  fn incr_token(&mut self, token: usize) {
    self.token = self.token.saturating_add(token);
  }

  fn fetch_bytes(&self) -> usize {
    self.bytes
  }

  fn fetch_token(&self) -> usize {
    self.token
  }
}

pub type AtomicCounter = Arc<AtomicTokenCounter>;

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct AtomicTokenCounter {
  bytes: AtomicUsize,
  token: AtomicUsize,
}

impl TokenCounter for AtomicCounter {
  fn incr_bytes(&mut self, bytes: usize) {
    let _ = self.bytes.fetch_add(bytes, Ordering::Relaxed);
  }

  fn incr_token(&mut self, token: usize) {
    let _ = self.token.fetch_add(token, Ordering::Relaxed);
  }

  fn fetch_bytes(&self) -> usize {
    self.bytes.load(Ordering::Relaxed)
  }

  fn fetch_token(&self) -> usize {
    self.token.load(Ordering::Relaxed)
  }
}

#[cfg(test)]
mod test {
  use crate::token::counter::{AtomicCounter, SimpleCounter, TokenCounter};

  #[test]
  fn test_simple_counter() {
    let mut counter = SimpleCounter::default();
    assert_eq!(counter.fetch_bytes(), 0);
    assert_eq!(counter.fetch_token(), 0);

    counter.incr_bytes(1);
    assert_eq!(counter.fetch_bytes(), 1);
    assert_eq!(counter.fetch_token(), 0);

    counter.incr_token(1);
    assert_eq!(counter.fetch_bytes(), 1);
    assert_eq!(counter.fetch_token(), 1);

    counter.incr_both(2, 2);
    assert_eq!(counter.fetch_bytes(), 3);
    assert_eq!(counter.fetch_token(), 3);
  }

  #[test]
  fn test_atomic_counter() {
    let mut counter = AtomicCounter::default();
    assert_eq!(counter.fetch_bytes(), 0);
    assert_eq!(counter.fetch_token(), 0);

    counter.incr_bytes(1);
    assert_eq!(counter.fetch_bytes(), 1);
    assert_eq!(counter.fetch_token(), 0);

    counter.incr_token(1);
    assert_eq!(counter.fetch_bytes(), 1);
    assert_eq!(counter.fetch_token(), 1);

    counter.incr_both(2, 2);
    assert_eq!(counter.fetch_bytes(), 3);
    assert_eq!(counter.fetch_token(), 3);
  }
}
