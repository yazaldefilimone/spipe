#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Range {
  pub start: usize,
  pub end: usize,
}

impl Range {
  pub fn new(start: usize, end: usize) -> Range {
    Range { start, end }
  }
}

impl Default for Range {
  fn default() -> Self {
    Range { start: 0, end: 0 }
  }
}

pub fn range_from(left: &Range, right: &Range) -> Range {
  Range::new(left.start, right.end)
}
