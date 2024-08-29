use crate::utils::range::Range;
use crate::utils::source::Source;

use super::report::report_error;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Severity {
  Error,
  Warning,
}

pub struct DiagnosticsManager {
  pub diagnostics: Vec<Diagnostic>,
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

  pub fn contains_error(&self) -> bool {
    self.diagnostics.iter().any(|d| d.severity == Severity::Error)
  }

  pub fn report(&self, source: &Source) {
    for diagnostic in self.diagnostics.iter() {
      report_error(
        &diagnostic.message,
        &diagnostic.hint,
        &diagnostic.range,
        &source,
        diagnostic.severity == Severity::Warning,
      );
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Diagnostic {
  pub message: String,
  pub hint: Option<String>,
  pub range: Range,
  pub severity: Severity,
}

pub enum TypeError {
  MissingIndexOnJoin { range: Range },
  PipeWithoutFrom { range: Range },
  RedundantSubQuery { range: Range },
  UnexpectedToken { range: Range },
  MissingSelectClause { range: Range },
  DuplicateColumn { range: Range },
  UnsupportedOperator { range: Range },
  MissingGroupBy { range: Range },
  AmbiguousColumn { range: Range },
  FunctionArgumentMismatch { range: Range },
}

impl From<TypeError> for Diagnostic {
  fn from(error: TypeError) -> Self {
    match error {
      TypeError::MissingIndexOnJoin { range } => Diagnostic {
        message: "missing index on join".to_string(),
        range,
        severity: Severity::Warning,
        hint: Some("consider adding an index to improve performance".to_string()),
      },
      TypeError::PipeWithoutFrom { range } => Diagnostic {
        message: "pipe missing `FROM` clause".to_string(),
        range,
        severity: Severity::Error,
        hint: Some("ensure `FROM` clause is present after aggregate".to_string()),
      },
      TypeError::RedundantSubQuery { range } => Diagnostic {
        message: "redundant subquery".to_string(),
        range,
        severity: Severity::Warning,
        hint: Some("optimize by refactoring the subquery".to_string()),
      },
      TypeError::UnexpectedToken { range } => Diagnostic {
        message: "unexpected token".to_string(),
        range,
        severity: Severity::Error,
        hint: Some("check the SQL syntax".to_string()),
      },
      TypeError::MissingSelectClause { range } => Diagnostic {
        message: "missing `SELECT` clause".to_string(),
        range,
        severity: Severity::Error,
        hint: Some("ensure the query starts with `SELECT`".to_string()),
      },
      TypeError::DuplicateColumn { range } => Diagnostic {
        message: "duplicate column".to_string(),
        range,
        severity: Severity::Warning,
        hint: Some("remove or rename the duplicate column".to_string()),
      },
      TypeError::UnsupportedOperator { range } => Diagnostic {
        message: "unsupported operator".to_string(),
        range,
        severity: Severity::Error,
        hint: Some("use supported operators like `=`, `<`, `>`".to_string()),
      },
      TypeError::MissingGroupBy { range } => Diagnostic {
        message: "missing `GROUP BY` clause".to_string(),
        range,
        severity: Severity::Error,
        hint: Some("add `GROUP BY` to group results correctly".to_string()),
      },
      TypeError::AmbiguousColumn { range } => Diagnostic {
        message: "ambiguous column reference".to_string(),
        range,
        severity: Severity::Error,
        hint: Some("qualify column names with table names".to_string()),
      },
      TypeError::FunctionArgumentMismatch { range } => Diagnostic {
        message: "function argument mismatch".to_string(),
        range,
        severity: Severity::Error,
        hint: Some("check the number and types of arguments".to_string()),
      },
    }
  }
}
