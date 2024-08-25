use crate::utils::{range::Range, Source};

use super::report::report_error;

pub struct DiagnosticsManager {
  pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Diagnostic {
  pub message: String,
  pub range: Range,
}

impl DiagnosticsManager {
  pub fn new() -> Self {
    Self { diagnostics: vec![] }
  }

  pub fn add(&mut self, diagnostic: Diagnostic) {
    self.diagnostics.push(diagnostic);
  }

  pub fn get_diagnostics(&self) -> Vec<Diagnostic> {
    self.diagnostics.clone()
  }

  pub fn report(&self, source: &Source, warring: bool) {
    for diagnostic in self.diagnostics.iter() {
      report_error(&diagnostic.message, &diagnostic.range, &source, warring);
    }
  }
}
