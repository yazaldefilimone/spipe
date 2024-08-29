#![allow(dead_code)]
use crate::diagnostics::maneger::{DiagnosticsManager, TypeError};
use crate::parser::ast::*;
use crate::utils::source::Source;

pub struct Checker {
  diagnostics: DiagnosticsManager,
  tables: Vec<String>,
  columns: Vec<String>,
}

impl Checker {
  pub fn new() -> Self {
    Self { diagnostics: DiagnosticsManager::new(), tables: vec![], columns: vec![] }
  }

  pub fn check(&mut self, program: &Program) {
    for stmt in &program.statements {
      self.check_statement(stmt);
    }
  }

  pub fn check_statement(&mut self, stmt: &Statement) {
    match stmt {
      Statement::Select(select) => self.check_select(select),
      Statement::From(from) => self.check_from(from),
      Statement::Join(join) => self.check_join(join),
      Statement::Where(where_clause) => self.check_where(where_clause),
      Statement::GroupBy(group_by) => self.check_group_by(group_by),
      Statement::Aggregate(agg) => self.check_aggregate(agg),
      Statement::Pipe(pipe) => self.check_pipe(pipe),
      _ => {}
    }
  }

  fn check_from(&mut self, from: &FromClause) {
    let table_name = from.table.lexeme.as_ref().unwrap().clone();
    if self.tables.contains(&table_name) {
      self.diagnostics.add(
        TypeError::DuplicateColumn {
          range: from.get_range(),
          // hint: Some("check for duplicate table usage".to_string()),
        }
        .into(),
      );
    } else {
      self.tables.push(table_name);
    }
  }

  fn check_select(&mut self, select: &SelectStatement) {
    if select.expressions.is_empty() {
      self.diagnostics.add(
        TypeError::MissingSelectClause {
          range: select.get_range(),
          // hint: Some("`SELECT` clause is empty".to_string()),
        }
        .into(),
      );
    }
    for expr in &select.expressions {
      if let Expression::Column(col) = &expr.expression {
        let column_name = col.column.lexeme.as_ref().unwrap().clone();
        if !self.columns.contains(&column_name) {
          self.columns.push(column_name);
        } else {
          self.diagnostics.add(
            TypeError::DuplicateColumn {
              range: col.get_range(),
              // hint: Some("column is used multiple times".to_string()),
            }
            .into(),
          );
        }
      }
    }
  }

  fn check_join(&mut self, join: &JoinClause) {
    let table_name = join.table.lexeme.as_ref().unwrap().clone();
    if !self.tables.contains(&table_name) {
      self.diagnostics.add(
        TypeError::MissingIndexOnJoin {
          range: join.get_range(),
          // hint: Some("missing index on the join table".to_string()),
        }
        .into(),
      );
    } else {
      // Checks if JOIN conditions are correctly structured
      // if let Expression::Condition(cond) = &join.on {
      //   if !self.check_condition_validity(cond) {
      //     self.diagnostics.add(
      //       TypeError::UnsupportedOperator {
      //         range: join.get_range(),
      //         // hint: Some("invalid or unsupported join condition".to_string()),
      //       }
      //       .into(),
      //     );
      //   }
      // }
    }
  }

  fn check_where(&mut self, where_clause: &WhereClause) {
    // Check for potential performance issues with OR conditions
    if where_clause.condition.emit().contains(" OR ") {
      self.diagnostics.add(
        TypeError::UnsupportedOperator {
          range: where_clause.get_range(),
          // hint: Some("use AND instead of OR for better indexing".to_string()),
        }
        .into(),
      );
    }
  }

  fn check_group_by(&mut self, group_by: &GroupByClause) {
    if group_by.columns.is_empty() {
      self.diagnostics.add(
        TypeError::MissingGroupBy {
          range: group_by.get_range(),
          // hint: Some("grouping by columns is missing".to_string()),
        }
        .into(),
      );
    }
  }

  fn check_aggregate(&mut self, agg: &AggregateClause) {
    // Check if the aggregate is properly associated with a table
    // if !self.tables.iter().any(|t| agg.argument.emit().contains(t)) {
    //   self.diagnostics.add(
    //     TypeError::PipeWithoutFrom {
    //       range: agg.get_range(),
    //       // hint: Some("ensure `FROM` clause is included before aggregate".to_string()),
    //     }
    //     .into(),
    //   );
    // }

    // Avoid redundant subqueries within aggregates
    if let Expression::Subquery(_) = &agg.argument {
      self.diagnostics.add(
        TypeError::RedundantSubQuery {
          range: agg.get_range(),
          // hint: Some("simplify or refactor the subquery".to_string()),
        }
        .into(),
      );
    }
  }

  fn check_pipe(&mut self, pipe: &PipeStatement) {
    self.check_statement(&pipe.left);
    self.check_statement(&pipe.right);

    // Check if the aggregate function is properly piped
    if let Statement::Aggregate(_) = &*pipe.right {
      // let left = pipe.left.as_ref();
      // if !left.to_sql().contains("FROM") {
      //   self.diagnostics.add(
      //     TypeError::PipeWithoutFrom {
      //       range: pipe.get_range(),
      //       // hint: Some("`FROM` clause required before aggregate".to_string()),
      //     }
      //     .into(),
      //   );
      // }
    }
  }

  fn check_condition_validity(&self, cond: &ConditionExpression) -> bool {
    // Add more complex condition checks if necessary
    !cond.left.emit().is_empty() && !cond.right.emit().is_empty()
  }

  pub fn report(&self, source: &Source) {
    self.diagnostics.report(source);
  }

  pub fn contains_error(&self) -> bool {
    self.diagnostics.contains_error()
  }
}
