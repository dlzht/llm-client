use std::sync::{
  Arc,
  atomic::{AtomicUsize, Ordering},
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
pub struct SimpleTokenCounter {
  bytes: usize,
  token: usize,
}

impl TokenCounter for SimpleTokenCounter {
  #[inline]
  fn incr_bytes(&mut self, bytes: usize) {
    self.bytes = self.bytes.saturating_add(bytes);
  }

  #[inline]
  fn incr_token(&mut self, token: usize) {
    self.token = self.token.saturating_add(token);
  }

  #[inline]
  fn fetch_bytes(&self) -> usize {
    self.bytes
  }

  #[inline]
  fn fetch_token(&self) -> usize {
    self.token
  }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct AtomicTokenCounter {
  bytes: AtomicUsize,
  token: AtomicUsize,
}

impl TokenCounter for AtomicTokenCounter {
  #[inline]
  fn incr_bytes(&mut self, bytes: usize) {
    let _ = self.bytes.fetch_add(bytes, Ordering::Relaxed);
  }

  #[inline]
  fn incr_token(&mut self, token: usize) {
    let _ = self.token.fetch_add(token, Ordering::Relaxed);
  }

  #[inline]
  fn fetch_bytes(&self) -> usize {
    self.bytes.load(Ordering::Relaxed)
  }

  #[inline]
  fn fetch_token(&self) -> usize {
    self.token.load(Ordering::Relaxed)
  }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SessionTokenCounter {
  input_counter: SimpleTokenCounter,
  output_counter: SimpleTokenCounter,
}

impl SessionTokenCounter {
  pub fn incr_input(&mut self, input_bytes: usize, input_token: usize) {
    self.input_counter.incr_both(input_bytes, input_token);
  }

  pub fn incr_output(&mut self, output_bytes: usize, output_token: usize) {
    self.output_counter.incr_both(output_bytes, output_token);
  }

  pub fn fetch_input(&self) -> (usize, usize) {
    (
      self.input_counter.fetch_bytes(),
      self.input_counter.fetch_token(),
    )
  }

  pub fn fetch_output(&self) -> (usize, usize) {
    (
      self.output_counter.fetch_bytes(),
      self.output_counter.fetch_token(),
    )
  }
}

pub type ClientTokenCounter = Arc<ClientTokenCounterInner>;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ClientTokenCounterInner {
  input_counter: AtomicTokenCounter,
  output_counter: AtomicTokenCounter,
}

impl ClientTokenCounterInner {
  pub fn incr_input(&mut self, input_bytes: usize, input_token: usize) {
    self.input_counter.incr_both(input_bytes, input_token);
  }

  pub fn incr_output(&mut self, output_bytes: usize, output_token: usize) {
    self.output_counter.incr_both(output_bytes, output_token);
  }

  pub fn fetch_input(&self) -> (usize, usize) {
    (
      self.input_counter.fetch_bytes(),
      self.input_counter.fetch_token(),
    )
  }

  pub fn fetch_output(&self) -> (usize, usize) {
    (
      self.output_counter.fetch_bytes(),
      self.output_counter.fetch_token(),
    )
  }
}

#[cfg(test)]
mod test {
  use crate::token::counter::{AtomicTokenCounter, SimpleTokenCounter, TokenCounter};

  #[test]
  fn test_simple_counter() {
    let mut counter = SimpleTokenCounter::default();
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
    let mut counter = AtomicTokenCounter::default();
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
