#![allow(dead_code)]
use crate::parser::ast::*;

impl Program {
  pub fn to_sql(&self) -> String {
    self.statements.iter().map(|stmt| stmt.to_sql()).collect::<Vec<_>>().join(" ")
  }
}

impl Statement {
  pub fn to_sql(&self) -> String {
    match self {
      Statement::Select(s) => s.emit(),
      Statement::From(f) => f.emit(),
      Statement::Join(j) => j.emit(),
      Statement::Where(w) => w.emit(),
      Statement::GroupBy(g) => g.emit(),
      Statement::Order(o) => o.emit(),
      Statement::Limit(l) => l.emit(),
      Statement::Pipe(p) => p.to_sql(),
      Statement::Aggregate(a) => a.emit(),
      Statement::Expression(e) => e.emit(),
    }
  }
}

impl PipeStatement {
  pub fn to_sql(&self) -> String {
    let left = self.left.to_sql();
    match &*self.right {
      Statement::Aggregate(a) => a.emit_with_base(left),
      Statement::Join(j) => format!("{} {}", left, j.emit()),
      Statement::Where(w) => format!("{} {}", left, w.emit()),
      Statement::GroupBy(g) => format!("{} {}", left, g.emit()),
      Statement::Order(o) => format!("{} {}", left, o.emit()),
      Statement::Limit(l) => format!("{} {}", left, l.emit()),
      _ => format!("{} {}", left, self.right.to_sql()),
    }
  }
}

impl AggregateClause {
  pub fn emit_with_base(&self, base: String) -> String {
    let alias = self.alias.as_ref().map_or(String::new(), |a| format!(" AS {}", a.lexeme.as_ref().unwrap()));
    let func = format!("{}({}){}", self.function.emit(), self.argument.emit(), alias);
    if base.trim().starts_with("FROM") {
      format!("SELECT {} {}", func, base)
    } else {
      format!("SELECT {} FROM {}", func, base)
    }
  }

  pub fn emit(&self) -> String {
    let alias = self.alias.as_ref().map_or(String::new(), |a| format!(" AS {}", a.lexeme.as_ref().unwrap()));
    format!("{}({}){}", self.function.emit(), self.argument.emit(), alias)
  }
}

impl AggregateFn {
  pub fn emit(&self) -> &str {
    match self {
      AggregateFn::Count => "COUNT",
      AggregateFn::Sum => "SUM",
      AggregateFn::Avg => "AVG",
      AggregateFn::Min => "MIN",
      AggregateFn::Max => "MAX",
      AggregateFn::StdDev => "STDDEV",
      AggregateFn::StdDevPop => "STDDEV_POP",
      AggregateFn::StdDevSamp => "STDDEV_SAMP",
      AggregateFn::VarPop => "VAR_POP",
      AggregateFn::VarSamp => "VAR_SAMP",
      AggregateFn::Variance => "VARIANCE",
      AggregateFn::First => "FIRST",
      AggregateFn::Last => "LAST",
      AggregateFn::GroupConcat => "GROUP_CONCAT",
      AggregateFn::StringAgg => "STRING_AGG",
      AggregateFn::Median => "MEDIAN",
      AggregateFn::Mode => "MODE",
      AggregateFn::ArrayAgg => "ARRAY_AGG",
      AggregateFn::JsonAgg => "JSON_AGG",
      AggregateFn::JsonObjectAgg => "JSON_OBJECT_AGG",
      AggregateFn::BitAnd => "BIT_AND",
      AggregateFn::BitOr => "BIT_OR",
      AggregateFn::BoolAnd => "BOOL_AND",
      AggregateFn::BoolOr => "BOOL_OR",
    }
  }
}

impl SelectStatement {
  pub fn emit(&self) -> String {
    let distinct = if self.distinct { "DISTINCT " } else { "" };
    let exprs = self.expressions.iter().map(|e| e.emit()).collect::<Vec<_>>().join(", ");
    let from = self.from.as_ref().map_or(String::new(), |f| format!(" {}", f.emit()));
    format!("SELECT {}{}{}", distinct, exprs, from)
  }
}

impl JoinClause {
  pub fn emit(&self) -> String {
    format!("JOIN {} ON {}", self.table.lexeme.as_ref().unwrap(), self.on.emit())
  }
}

impl WhereClause {
  pub fn emit(&self) -> String {
    format!("WHERE {}", self.condition.emit())
  }
}

impl GroupByClause {
  pub fn emit(&self) -> String {
    let cols = self.columns.iter().map(|expression| expression.emit()).collect::<Vec<_>>().join(", ");
    format!("GROUP BY {}", cols)
  }
}

impl OrderClause {
  pub fn emit(&self) -> String {
    let cols = self.columns.iter().map(|order| order.emit()).collect::<Vec<_>>().join(", ");
    format!("ORDER BY {}", cols)
  }
}

impl OrderColumn {
  pub fn emit(&self) -> String {
    format!("{} {}", self.column.emit(), self.direction.emit())
  }
}

impl OrderDirection {
  pub fn emit(&self) -> &str {
    match self {
      OrderDirection::Asc => "ASC",
      OrderDirection::Desc => "DESC",
    }
  }
}

impl LimitClause {
  pub fn emit(&self) -> String {
    let offset = self.offset.as_ref().map_or(String::new(), |literal| format!(", {}", literal.emit()));
    format!("LIMIT {}{}", self.count.emit(), offset)
  }
}

impl Expression {
  pub fn emit(&self) -> String {
    match self {
      Expression::Column(c) => c.emit(),
      Expression::Literal(l) => l.emit(),
      Expression::Condition(c) => c.emit(),
      Expression::FunctionCall(f) => f.emit(),
      Expression::Subquery(s) => s.emit(),
    }
  }
}

impl ColumnExpression {
  pub fn emit(&self) -> String {
    if self.table.is_none() {
      return self.column.lexeme.as_ref().unwrap().to_string();
    }

    let table = self.table.as_ref().unwrap();

    format!("{}.{}", self.column.lexeme.as_ref().unwrap(), table.lexeme.as_ref().unwrap())
  }
}

impl ConditionExpression {
  pub fn emit(&self) -> String {
    format!("{} {} {}", self.left.emit(), self.operator.emit(), self.right.emit())
  }
}

impl Operator {
  pub fn emit(&self) -> &str {
    match self {
      Operator::Equal => "=",
      Operator::NotEqual => "!=",
      Operator::LessThan => "<",
      Operator::GreaterThan => ">",
      Operator::LessThanOrEqual => "<=",
      Operator::GreaterThanOrEqual => ">=",
      Operator::And => "AND",
      Operator::Or => "OR",
    }
  }
}

impl FromClause {
  pub fn emit(&self) -> String {
    format!("FROM {}", self.table.lexeme.as_ref().unwrap())
  }
}

impl Literal {
  pub fn emit(&self) -> String {
    match self {
      Literal::String(s) => s.emit(),
      Literal::Number(n) => n.emit(),
      Literal::Boolean(b) => b.emit(),
    }
  }
}

impl NumberLiteral {
  pub fn emit(&self) -> String {
    self.raw.clone()
  }
}
impl StringLiteral {
  pub fn emit(&self) -> String {
    format!("'{}'", self.value)
  }
}
impl BooleanLiteral {
  pub fn emit(&self) -> String {
    self.value.to_string()
  }
}

impl FunctionCallExpression {
  pub fn emit(&self) -> String {
    let args = self.arguments.iter().map(|e| e.emit()).collect::<Vec<_>>().join(", ");
    format!("{}({})", self.function_name.lexeme.as_ref().unwrap(), args)
  }
}

impl SubqueryExpression {
  pub fn emit(&self) -> String {
    format!("({})", self.stmt.to_sql())
  }
}

impl SelectExpression {
  pub fn emit(&self) -> String {
    let expr = self.expression.emit();
    let alias = self.alias.as_ref().map_or(String::new(), |a| format!(" AS {}", a.lexeme.as_ref().unwrap()));
    format!("{}{}", expr, alias)
  }
}
