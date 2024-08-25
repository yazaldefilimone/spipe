use crate::{
  lexer::Token,
  utils::range::{range_from, Range},
};
use serde::{Deserialize, Serialize};

// Programa completo
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Program {
  pub statements: Vec<Statement>,
}

impl Program {
  pub fn new(statements: Vec<Statement>) -> Self {
    Program { statements }
  }
}

// Declarações (Statements)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Statement {
  Select(SelectStatement),
  From(FromClause),
  Join(JoinClause),
  Where(WhereClause),
  GroupBy(GroupByClause),
  Order(OrderClause),
  Limit(LimitClause),
  Pipe(PipeStatement),
  Aggregate(AggregateClause),
  Expression(Expression),
}

impl Statement {
  pub fn create_select(distinct: bool, expressions: Vec<SelectExpression>, range: Range) -> Self {
    Statement::Select(SelectStatement::new(distinct, expressions, range))
  }

  pub fn create_from(table: Token, range: Range) -> Self {
    Statement::From(FromClause::new(table, range))
  }

  pub fn create_join(table: Token, on: ConditionExpression, range: Range) -> Self {
    Statement::Join(JoinClause::new(table, on, range))
  }

  pub fn create_where(condition: Expression, range: Range) -> Self {
    Statement::Where(WhereClause::new(condition, range))
  }

  pub fn create_group_by(columns: Vec<Expression>, range: Range) -> Self {
    Statement::GroupBy(GroupByClause::new(columns, range))
  }

  pub fn create_order(columns: Vec<OrderColumn>, range: Range) -> Self {
    Statement::Order(OrderClause::new(columns, range))
  }

  pub fn create_limit(count: NumberLiteral, offset: Option<NumberLiteral>, range: Range) -> Self {
    Statement::Limit(LimitClause::new(count, offset, range))
  }

  pub fn create_pipe(left: Statement, right: Statement) -> Self {
    Statement::Pipe(PipeStatement::new(left, right))
  }

  pub fn get_range(&self) -> Range {
    match self {
      Statement::Select(select) => select.get_range(),
      Statement::From(from) => from.get_range(),
      Statement::Join(join) => join.get_range(),
      Statement::Where(where_) => where_.get_range(),
      Statement::GroupBy(group_by) => group_by.get_range(),
      Statement::Order(order) => order.get_range(),
      Statement::Limit(limit) => limit.get_range(),
      Statement::Pipe(pipe) => pipe.get_range(),
      Statement::Expression(expression) => expression.get_range(),
      Statement::Aggregate(aggregate) => aggregate.get_range(),
    }
  }
}

// Declaração de Pipe (PipeStatement)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PipeStatement {
  pub left: Box<Statement>,
  pub right: Box<Statement>,
}

impl PipeStatement {
  pub fn new(left: Statement, right: Statement) -> Self {
    PipeStatement { left: Box::new(left), right: Box::new(right) }
  }

  pub fn get_range(&self) -> Range {
    let left = self.left.get_range();
    let right = self.right.get_range();
    range_from(&left, &right)
  }
}

// Declaração de Agregação (AggregateClause)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AggregateClause {
  pub function: AggregateFn,
  pub argument: Expression,
  pub alias: Option<Token>,
  pub range: Range,
}

impl AggregateClause {
  pub fn new(function: AggregateFn, argument: Expression, alias: Option<Token>, range: Range) -> Self {
    AggregateClause { function, argument, alias, range }
  }

  pub fn get_range(&self) -> Range {
    return self.range.clone();
  }
}

// Função de Agregação (AggregateFunction)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum AggregateFn {
  Count,
  Sum,
  Avg,
  Min,
  Max,
}

// Cláusula SELECT (SelectStatement)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SelectStatement {
  pub distinct: bool,
  pub expressions: Vec<SelectExpression>,
  pub from: Option<FromClause>,
  pub range: Range,
}

impl SelectStatement {
  pub fn new(distinct: bool, expressions: Vec<SelectExpression>, range: Range) -> Self {
    SelectStatement { distinct, expressions, range, from: None }
  }

  pub fn with_from_clause(&mut self, from: FromClause) {
    self.range = range_from(&self.range, &from.get_range());
    self.from = Some(from);
  }

  pub fn get_range(&self) -> Range {
    self.range.clone()
  }
}

// Expressão de Seleção (SelectExpression)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SelectExpression {
  pub expression: Expression,
  pub alias: Option<Token>,
  pub range: Range,
}

impl SelectExpression {
  pub fn new(expression: Expression, alias: Option<Token>, range: Range) -> Self {
    SelectExpression { expression, alias, range }
  }

  pub fn get_range(&self) -> Range {
    let right = self.alias.clone().and_then(|alias| Some(alias.range)).unwrap_or(self.range.clone());
    range_from(&right, &self.expression.get_range())
  }
}

// Cláusula FROM (FromClause)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FromClause {
  pub table: Token,
  pub range: Range,
}

impl FromClause {
  pub fn new(table: Token, range: Range) -> Self {
    FromClause { table, range }
  }

  pub fn get_range(&self) -> Range {
    range_from(&self.range, &self.table.range)
  }
}

// Cláusula JOIN (JoinClause)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct JoinClause {
  pub table: Token,
  pub on: ConditionExpression,
  pub range: Range,
}

impl JoinClause {
  pub fn new(table: Token, on: ConditionExpression, range: Range) -> Self {
    JoinClause { table, on, range }
  }

  pub fn get_range(&self) -> Range {
    let condition = self.on.get_range();
    range_from(&condition, &self.table.range)
  }
}

// Cláusula WHERE (WhereClause)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct WhereClause {
  pub condition: Expression,
  pub range: Range,
}

impl WhereClause {
  pub fn new(condition: Expression, range: Range) -> Self {
    WhereClause { condition, range }
  }
  pub fn get_range(&self) -> Range {
    self.range.clone()
  }
}

// Cláusula GROUP BY (GroupByClause)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GroupByClause {
  pub columns: Vec<Expression>,
  pub range: Range,
}

impl GroupByClause {
  pub fn new(columns: Vec<Expression>, range: Range) -> Self {
    GroupByClause { columns, range }
  }

  pub fn get_range(&self) -> Range {
    return self.range.clone();
  }
}

// Cláusula ORDER BY (OrderClause)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct OrderClause {
  pub columns: Vec<OrderColumn>,
  pub range: Range,
}

impl OrderClause {
  pub fn new(columns: Vec<OrderColumn>, range: Range) -> Self {
    OrderClause { columns, range }
  }

  pub fn get_range(&self) -> Range {
    return self.range.clone();
  }
}

// Cláusula LIMIT (LimitClause)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LimitClause {
  pub count: NumberLiteral,
  pub offset: Option<NumberLiteral>,
  pub range: Range,
}

impl LimitClause {
  pub fn new(count: NumberLiteral, offset: Option<NumberLiteral>, range: Range) -> Self {
    LimitClause { count, offset, range }
  }

  pub fn get_range(&self) -> Range {
    return self.range.clone();
  }
}

// Expressões (Expression)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Expression {
  Column(ColumnExpression),
  Literal(Literal),
  Condition(ConditionExpression),
  FunctionCall(FunctionCallExpression), // COUNT, SUM, etc.
  Subquery(SubqueryExpression),         // todo: is correct?
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SubqueryExpression {
  // (...)
  pub stmt: Box<Statement>,
  pub range: Range,
}

impl SubqueryExpression {
  pub fn new(stmt: Statement, range: Range) -> Self {
    SubqueryExpression { stmt: Box::new(stmt), range }
  }

  pub fn get_range(&self) -> Range {
    self.range.clone()
  }
}

impl Expression {
  pub fn create_column(table: Option<Token>, column: Token) -> Self {
    let range = range_from(&table.clone().unwrap_or(column.clone()).range, &column.range);
    Expression::Column(ColumnExpression::new(table, column, range))
  }

  pub fn create_literal(value: Literal) -> Self {
    Expression::Literal(value)
  }

  pub fn create_condition(left: Expression, operator: Operator, right: Expression) -> Self {
    Expression::Condition(ConditionExpression::new(left, operator, right))
  }

  pub fn create_function_call(function_name: Token, arguments: Vec<Expression>, range: Range) -> Self {
    Expression::FunctionCall(FunctionCallExpression::new(function_name, arguments, range))
  }

  pub fn create_subquery(stmt: Statement, range: Range) -> Self {
    Expression::Subquery(SubqueryExpression::new(stmt, range))
  }

  pub fn get_range(&self) -> Range {
    match self {
      Expression::Column(column) => column.get_range(),
      Expression::Literal(literal) => literal.get_range(),
      Expression::Condition(condition) => condition.get_range(),
      Expression::FunctionCall(function_call) => function_call.get_range(),
      Expression::Subquery(select) => select.get_range(),
    }
  }
}

// Expressão de Função (FunctionCallExpression)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FunctionCallExpression {
  pub function_name: Token,
  pub arguments: Vec<Expression>,
  pub range: Range,
}

impl FunctionCallExpression {
  pub fn new(function_name: Token, arguments: Vec<Expression>, range: Range) -> Self {
    FunctionCallExpression { function_name, arguments, range }
  }

  pub fn get_range(&self) -> Range {
    let first = self.arguments.first().unwrap().get_range();
    let last = self.arguments.last().unwrap().get_range();
    range_from(&first, &last)
  }
}

// Expressões de Colunas (ColumnExpression)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ColumnExpression {
  pub table: Option<Token>, // Nome da tabela opcional para colunas qualificadas
  pub column: Token,
  pub range: Range,
}

impl ColumnExpression {
  pub fn new(table: Option<Token>, column: Token, range: Range) -> Self {
    ColumnExpression { table, column, range }
  }

  pub fn get_range(&self) -> Range {
    let right = self.table.clone().and_then(|table| Some(table.range)).unwrap_or(self.range.clone());
    range_from(&right, &self.column.range)
  }
}

// Expressão de Condição (ConditionExpression)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConditionExpression {
  pub left: Box<Expression>,
  pub operator: Operator,
  pub right: Box<Expression>,
}

impl ConditionExpression {
  pub fn new(left: Expression, operator: Operator, right: Expression) -> Self {
    ConditionExpression { left: Box::new(left), operator, right: Box::new(right) }
  }

  pub fn get_range(&self) -> Range {
    let left = self.left.get_range();
    let right = self.right.get_range();
    range_from(&left, &right)
  }
}

// Literais (Literal)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Literal {
  String(StringLiteral),
  Number(NumberLiteral),
  Boolean(BooleanLiteral),
}

impl Literal {
  pub fn create_string(value: String, range: Range) -> Self {
    Literal::String(StringLiteral::new(value, range))
  }

  pub fn create_number(value: String, range: Range) -> Self {
    Literal::Number(NumberLiteral::new(value, range))
  }

  pub fn create_boolean(value: bool, range: Range) -> Self {
    Literal::Boolean(BooleanLiteral::new(value, range))
  }

  pub fn get_range(&self) -> Range {
    match self {
      Literal::String(string) => string.range.clone(),
      Literal::Number(number) => number.range.clone(),
      Literal::Boolean(boolean) => boolean.range.clone(),
    }
  }
}

// Literais de String (StringLiteral)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct StringLiteral {
  pub value: String,
  pub range: Range,
}

impl StringLiteral {
  pub fn new(value: String, range: Range) -> Self {
    StringLiteral { value, range }
  }
}

// Literais Numéricos (NumberLiteral)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct NumberLiteral {
  pub raw: String, // Consider changing to f64 for numeric operations
  pub range: Range,
}

impl NumberLiteral {
  pub fn new(raw: String, range: Range) -> Self {
    NumberLiteral { raw, range }
  }
}

// Literais Booleanos (BooleanLiteral)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct BooleanLiteral {
  pub value: bool,
  pub range: Range,
}

impl BooleanLiteral {
  pub fn new(value: bool, range: Range) -> Self {
    BooleanLiteral { value, range }
  }
}

// Operadores (Operator)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Operator {
  Equal,              // =
  NotEqual,           // !=
  LessThan,           // <
  GreaterThan,        // >
  LessThanOrEqual,    // <=
  GreaterThanOrEqual, // >=
  And,                // AND
  Or,                 // OR
}

// Colunas de Ordenação (OrderColumn)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct OrderColumn {
  pub column: Expression,
  pub direction: OrderDirection,
}

impl OrderColumn {
  pub fn new(column: Expression, direction: OrderDirection) -> Self {
    OrderColumn { column, direction }
  }

  pub fn get_range(&self) -> Range {
    return self.column.get_range();
  }
}

// Direções de Ordenação (OrderDirection)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum OrderDirection {
  Asc,  // ASC
  Desc, // DESC
}
