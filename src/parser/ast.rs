use crate::utils::range::Range;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Program {
  pub statements: Vec<Statement>,
}

impl Program {
  pub fn new(statements: Vec<Statement>) -> Self {
    Program { statements }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Statement {
  Select(SelectStatement),
  From(FromClause),
  Join(JoinClause),
  Where(WhereClause),
  GroupBy(GroupByClause),
  Aggregate(AggregateClause),
  OrderBy(OrderByClause),
  Limit(LimitClause),
  Pipe(PipeStatement),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PipeStatement {
  pub left: Box<Statement>,
  pub right: Box<Statement>,
  pub range: Range,
}

/*
eg.
SELECT * FROM table WHERE column = value
*/
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SelectStatement {
  pub distinct: bool,
  pub columns: Vec<ColumnExpression>,
  pub range: Range,
}

// eg.
// FROM table
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FromClause {
  pub table: Identifier,
  pub range: Range,
}

// eg.
// JOIN table ON column = value

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct JoinClause {
  pub join_type: JoinType,
  pub table: Identifier,
  pub on: ConditionExpression,
  pub range: Range,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum JoinType {
  Inner,
  Left,
  Right,
  Full,
}

// eg.
// WHERE column = value
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct WhereClause {
  pub condition: ConditionExpression,
  pub range: Range,
}

// eg.
// GROUP BY column
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GroupByClause {
  pub columns: Vec<ColumnExpression>,
  pub range: Range,
}

// eg.
// AGGREGATE FUNCTION(column) AS alias
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AggregateClause {
  pub functions: Vec<AggregateFunction>,
  pub range: Range,
}

// eg.
// COUNT(column) AS alias
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AggregateFunction {
  pub function: AggregateFunctionType,
  pub argument: Expression,
  pub alias: Option<Identifier>,
  pub range: Range,
}

// eg.
// COUNT
// SUM
// AVG
// MIN
// MAX
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum AggregateFunctionType {
  Count,
  Sum,
  Avg,
  Min,
  Max,
}

// eg.
// ORDER BY column
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct OrderByClause {
  pub columns: Vec<OrderByColumn>,
  pub range: Range,
}

// eg.
// ASC
// DESC
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct OrderByColumn {
  pub column: ColumnExpression,
  pub direction: OrderDirection,
  pub range: Range,
}

// eg.
// ASC
// DESC
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum OrderDirection {
  Asc,
  Desc,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LimitClause {
  pub count: NumberLiteral,
  pub offset: Option<NumberLiteral>,
  pub range: Range,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ColumnExpression {
  pub table: Option<Identifier>,
  pub name: Identifier,
  pub range: Range,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConditionExpression {
  pub left: Box<Expression>,
  pub operator: Operator,
  pub right: Box<Expression>,
  pub range: Range,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Expression {
  Column(ColumnExpression),
  Literal(Literal),
  FunctionCall(FunctionCall),
  Condition(ConditionExpression),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FunctionCall {
  pub name: Identifier,
  pub arguments: Vec<Expression>,
  pub range: Range,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Literal {
  String(StringLiteral),
  Number(NumberLiteral),
  Boolean(BooleanLiteral),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct StringLiteral {
  pub value: String,
  pub range: Range,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct NumberLiteral {
  pub value: String,
  pub range: Range,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct BooleanLiteral {
  pub value: bool,
  pub range: Range,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Identifier {
  pub value: String,
  pub range: Range,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Operator {
  Equal,
  NotEqual,
  LessThan,
  GreaterThan,
  LessThanOrEqual,
  GreaterThanOrEqual,
  And,
  Or,
}
