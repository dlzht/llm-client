use serde::{Deserialize, Serialize};

pub trait TokenEvaluator {
  fn update(&mut self, bytes: usize, token: usize);

  fn estimate(&self, token: usize) -> usize;
}

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct SimpleEvaluator;

impl TokenEvaluator for SimpleEvaluator {
  fn update(&mut self, _bytes: usize, _token: usize) {
  }

  fn estimate(&self, token: usize) -> usize {
    token
  }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct ProportionEvaluator {
  bytes: usize,
  token: usize,
  ratio: f64
}

impl Default for ProportionEvaluator {
  fn default() -> Self {
    ProportionEvaluator {
      bytes: 0,
      token: 0,
      ratio: 1.0,
    }
  }
}

impl TokenEvaluator for ProportionEvaluator {

  fn update(&mut self, bytes: usize, token: usize) {
    if bytes == 0 || token == 0 {
      return;
    }
    self.bytes += bytes;
    self.token += token;
    self.ratio = self.token as f64 / self.bytes as f64;
  }

  fn estimate(&self, token: usize) -> usize {
    (self.ratio * token as f64) as usize
  }
}

#[cfg(test)]
mod test {
  use crate::token::evaluator::{ProportionEvaluator, SimpleEvaluator, TokenEvaluator};

  #[test]
  fn test_simple_evaluator() {
    let mut estimator = SimpleEvaluator::default();
    assert_eq!(estimator.estimate(0), 0);
    assert_eq!(estimator.estimate(1), 1);

    estimator.update(1, 1);
    assert_eq!(estimator.estimate(1), 1);
  }

  #[test]
  fn test_proportion_evaluator() {
    let mut estimator = ProportionEvaluator::default();
    assert_eq!(estimator.estimate(0), 0);
    assert_eq!(estimator.estimate(1), 1);

    estimator.update(1, 2);
    assert_eq!(estimator.estimate(1), 2);
  }
}