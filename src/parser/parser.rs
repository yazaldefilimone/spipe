use super::ast::*;
use crate::diagnostics::report::report_and_exit;
use crate::lexer::{Lexer, Token, TokenType};
use crate::utils::range::{range_from, Range};
use crate::utils::source::Source;

pub struct Parser<'a> {
  lexer: &'a mut Lexer<'a>,
}

impl<'a> Parser<'a> {
  pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
    Self { lexer }
  }

  pub fn parse(&mut self) -> Program {
    self.parse_program()
  }

  fn parse_program(&mut self) -> Program {
    let mut statements = vec![];
    while !self.is_end() {
      let statement = self.parse_statement();
      statements.push(statement);
      self.match_token_and_consume(TokenType::Semicolon);
    }
    Program::new(statements)
  }

  fn parse_statement(&mut self) -> Statement {
    self.skip_comments();
    let mut statement = self.parse_primary_statement();

    while self.match_token_and_consume(TokenType::Pipe).is_some() {
      let next_statement = self.parse_primary_statement();
      statement = Statement::Pipe(PipeStatement::new(statement, next_statement));
    }

    statement
  }

  fn parse_primary_statement(&mut self) -> Statement {
    let token = self.lexer.peek_token();
    match token.kind {
      TokenType::Select => Statement::Select(self.parse_select_statement()),
      TokenType::From => Statement::From(self.parse_from_clause()),
      TokenType::Join => Statement::Join(self.parse_join_clause()),
      TokenType::Where => Statement::Where(self.parse_where_clause()),
      TokenType::Group => Statement::GroupBy(self.parse_group_by_clause()),
      TokenType::Order => Statement::Order(self.parse_order_clause()),
      TokenType::Limit => Statement::Limit(self.parse_limit_clause()),
      TokenType::Aggregate => Statement::Aggregate(self.parse_aggregate_clause()),
      _ => self.report_unexpected_token(token),
    }
  }

  fn parse_aggregate_function(&mut self) -> AggregateFn {
    let token = self.consume_token();
    match token.kind {
      TokenType::Count => AggregateFn::Count,
      TokenType::Sum => AggregateFn::Sum,
      TokenType::Avg => AggregateFn::Avg,
      TokenType::Min => AggregateFn::Min,
      TokenType::Max => AggregateFn::Max,
      TokenType::StdDev => AggregateFn::StdDev,
      TokenType::StdDevPop => AggregateFn::StdDevPop,
      TokenType::StdDevSamp => AggregateFn::StdDevSamp,
      TokenType::VarPop => AggregateFn::VarPop,
      TokenType::VarSamp => AggregateFn::VarSamp,
      TokenType::Variance => AggregateFn::Variance,
      TokenType::First => AggregateFn::First,
      TokenType::Last => AggregateFn::Last,
      TokenType::StringAgg => AggregateFn::StringAgg,
      TokenType::Median => AggregateFn::Median,
      TokenType::Mode => AggregateFn::Mode,
      TokenType::ArrayAgg => AggregateFn::ArrayAgg,
      TokenType::JsonAgg => AggregateFn::JsonAgg,
      TokenType::BitAnd => AggregateFn::BitAnd,
      TokenType::BitOr => AggregateFn::BitOr,
      TokenType::BoolAnd => AggregateFn::BoolAnd,
      TokenType::BoolOr => AggregateFn::BoolOr,
      _ => {
        let message = format!("unexpected token '{}'", token.kind.to_string());
        self.report_error(message, token);
      }
    }
  }

  fn parse_aggregate_clause(&mut self) -> AggregateClause {
    let aggregate_range = self.consume_expect_token(TokenType::Aggregate).range;
    let function = self.parse_aggregate_function();
    self.consume_expect_token(TokenType::LeftParen);
    let argument = self.parse_expression();
    self.consume_expect_token(TokenType::RightParen);
    let alias = if self.match_token_and_consume(TokenType::As).is_some() {
      Some(self.consume_expect_token(TokenType::Identifier))
    } else {
      None
    };

    let mut range = argument.get_range();
    if let Some(alias) = &alias {
      range = range_from(&range, &alias.range);
    }

    let range = range_from(&aggregate_range, &range);
    AggregateClause::new(function, argument, alias, range)
  }

  fn parse_select_statement(&mut self) -> SelectStatement {
    let select_range = self.consume_expect_token(TokenType::Select).range;
    let mut expressions = vec![];
    let mut last_range = select_range.clone();
    let is_distinct = self.match_token_and_consume(TokenType::Distinct).is_some();
    while !self.match_token(&TokenType::From) && !self.is_end() {
      let expression = self.parse_select_expression();
      expressions.push(expression);
      self.match_token_and_consume(TokenType::Comma);
    }

    if !expressions.is_empty() {
      last_range = expressions.last().unwrap().get_range();
    }
    let range = range_from(&select_range, &last_range);
    let mut select_statement = SelectStatement::new(is_distinct, expressions, range);

    if self.match_token(&TokenType::From) {
      let from = self.parse_from_clause();
      select_statement.with_from_clause(from);
    }
    select_statement
  }

  fn parse_select_expression(&mut self) -> SelectExpression {
    let expression = self.parse_expression();
    let alias = if self.match_token_and_consume(TokenType::As).is_some() {
      Some(self.consume_expect_token(TokenType::Identifier))
    } else {
      None
    };
    let mut range = expression.get_range();

    if let Some(alias) = &alias {
      range = range_from(&range, &alias.range);
    }
    SelectExpression::new(expression, alias, range)
  }

  fn parse_from_clause(&mut self) -> FromClause {
    let from_range = self.consume_expect_token(TokenType::From).range;
    let table_name = self.consume_expect_token(TokenType::Identifier);
    let range = range_from(&from_range, &table_name.range);
    FromClause::new(table_name, range)
  }

  fn parse_join_clause(&mut self) -> JoinClause {
    let join_range = self.consume_expect_token(TokenType::Join).range;
    let table_name = self.consume_expect_token(TokenType::Identifier);

    self.consume_expect_token(TokenType::On);

    let left = self.parse_column_expression();
    let operator = self.parse_operator();
    let right = self.parse_column_expression();

    // todo: is correct
    let left_range = range_from(&join_range, &left.get_range());

    let range = range_from(&left_range, &right.get_range());

    let condition = ConditionExpression::new(left, operator, right);

    JoinClause::new(table_name, condition, range)
  }

  fn parse_where_clause(&mut self) -> WhereClause {
    let where_range = self.consume_expect_token(TokenType::Where).range;

    let condition = self.parse_condition_expression();

    let range = range_from(&where_range, &condition.get_range());
    WhereClause::new(condition, range)
  }

  fn parse_group_by_clause(&mut self) -> GroupByClause {
    let group_range = self.consume_expect_token(TokenType::Group).range;
    self.consume_expect_token(TokenType::By);
    let mut columns = vec![];
    while !self.match_any_token(&[TokenType::Order, TokenType::Limit]) && !self.is_end() {
      columns.push(self.parse_column_expression());
      if self.match_token_and_consume(TokenType::Comma).is_none() {
        break;
      }
    }
    if !columns.is_empty() {
      let last_range = columns.last().unwrap().get_range();
      let range = range_from(&group_range, &last_range);
      return GroupByClause::new(columns, range);
    }
    GroupByClause::new(columns, group_range)
  }

  fn parse_order_clause(&mut self) -> OrderClause {
    let order_range = self.consume_expect_token(TokenType::Order).range;
    self.consume_expect_token(TokenType::By);
    let mut columns = vec![];

    while !self.match_token(&TokenType::Limit) && !self.is_end() {
      columns.push(self.parse_order_column());
      if self.match_token_and_consume(TokenType::Comma).is_none() {
        break;
      }
    }

    if !columns.is_empty() {
      let last_range = columns.last().unwrap().get_range();
      let range = range_from(&order_range, &last_range);
      return OrderClause::new(columns, range);
    }
    OrderClause::new(columns, self.current_range())
  }

  fn parse_limit_clause(&mut self) -> LimitClause {
    let limit_range = self.consume_expect_token(TokenType::Limit).range;
    let count = self.parse_number_literal();
    let offset = if self.match_token_and_consume(TokenType::Comma).is_some() {
      Some(self.parse_number_literal())
    } else {
      None
    };

    let mut range = range_from(&limit_range, &count.range);
    if let Some(offset) = &offset {
      range = range_from(&range, &offset.range);
    }

    LimitClause::new(count, offset, range)
  }

  fn parse_expression(&mut self) -> Expression {
    let token = self.lexer.peek_token();
    match token.kind {
      TokenType::Identifier => self.parse_column_or_function_call(),
      TokenType::Number => {
        let literal = self.parse_number_literal();
        Expression::create_literal(Literal::Number(literal))
      }
      TokenType::String => {
        let literal = self.parse_string_literal();
        Expression::create_literal(Literal::String(literal))
      }
      TokenType::LeftParen => self.parse_subquery_expression(),
      _ => self.report_unexpected_token(token),
    }
  }

  fn parse_column_or_function_call(&mut self) -> Expression {
    let identifier = self.consume_expect_token(TokenType::Identifier);

    if self.match_token_and_consume(TokenType::LeftParen).is_some() {
      let mut arguments = vec![];
      while !self.match_token(&TokenType::RightParen) && !self.is_end() {
        let argument = self.parse_expression();
        arguments.push(argument);
        self.match_token_and_consume(TokenType::Comma);
      }
      self.consume_expect_token(TokenType::RightParen);
      Expression::create_function_call(identifier, arguments, self.current_range())
    } else {
      let mut column = None;
      if self.match_token_and_consume(TokenType::Dot).is_some() {
        column = Some(self.consume_expect_token(TokenType::Identifier));
      }
      Expression::create_column(column, identifier)
    }
  }

  fn parse_subquery_expression(&mut self) -> Expression {
    let left_paren_range = self.consume_expect_token(TokenType::LeftParen).range;
    let statement = self.parse_statement();
    // if self.match_token(&TokenType::Select) {
    //   let select = self.parse_select_statement();
    //   let right_paren_range = self.consume_expect_token(TokenType::RightParen).range;
    //   let range = range_from(&left_paren_range, &right_paren_range);
    //   Expression::create_subquery(select, range)
    // } else {
    //   let expression = self.parse_expression();
    //   let right_paren_range = self.consume_expect_token(TokenType::RightParen).range;
    //   let range = range_from(&left_paren_range, &right_paren_range);
    //   expression
    // }
    let right_paren_range = self.consume_expect_token(TokenType::RightParen).range;
    let range = range_from(&left_paren_range, &right_paren_range);
    Expression::create_subquery(statement, range)
  }

  fn parse_column_expression(&mut self) -> Expression {
    let column_name = self.consume_expect_token(TokenType::Identifier);
    let mut table_name = None;
    if self.match_token_and_consume(TokenType::Dot).is_some() {
      table_name = Some(self.consume_expect_token(TokenType::Identifier));
    }
    Expression::create_column(table_name, column_name)
  }

  fn parse_condition_expression(&mut self) -> Expression {
    let left = self.parse_expression();
    let operator = self.parse_operator();
    let right = self.parse_expression();
    Expression::create_condition(left, operator, right)
  }

  fn parse_operator(&mut self) -> Operator {
    let token = self.consume_token();
    match token.kind {
      TokenType::Equal => Operator::Equal,
      TokenType::NotEqual => Operator::NotEqual,
      TokenType::LessThan => Operator::LessThan,
      TokenType::GreaterThan => Operator::GreaterThan,
      TokenType::LessThanOrEqual => Operator::LessThanOrEqual,
      TokenType::GreaterThanOrEqual => Operator::GreaterThanOrEqual,
      TokenType::And => Operator::And,
      TokenType::Or => Operator::Or,
      _ => self.report_unexpected_token(token),
    }
  }

  fn parse_order_column(&mut self) -> OrderColumn {
    let column = self.parse_column_expression();
    let direction = if self.match_token_and_consume(TokenType::Desc).is_some() {
      OrderDirection::Desc
    } else {
      OrderDirection::Asc
    };
    OrderColumn::new(column, direction)
  }

  fn parse_number_literal(&mut self) -> NumberLiteral {
    let token = self.consume_expect_token(TokenType::Number);
    if token.lexeme.is_none() {
      self.report_token_with_message("expected number literal".to_string(), token);
    }
    let value = token.lexeme.unwrap();
    NumberLiteral::new(value, token.range)
  }

  fn parse_string_literal(&mut self) -> StringLiteral {
    let token = self.consume_expect_token(TokenType::String);
    if token.lexeme.is_none() {
      self.report_token_with_message("expected string literal".to_string(), token);
    }
    let value = token.lexeme.unwrap();
    StringLiteral::new(value, token.range)
  }

  fn consume_expect_token(&mut self, kind: TokenType) -> Token {
    let token = self.consume_token();
    if token.kind != kind {
      let message = format!("expected '{}' but found '{}'", kind.to_string(), token.kind.to_string());
      self.report_error(message, token);
    }
    token
  }

  fn consume_token(&mut self) -> Token {
    self.lexer.next_token()
  }

  fn match_token(&mut self, kind: &TokenType) -> bool {
    self.lexer.peek_token().kind == *kind
  }

  fn match_any_token(&mut self, kinds: &[TokenType]) -> bool {
    let token = self.lexer.peek_token();
    kinds.contains(&token.kind)
  }

  fn match_token_and_consume(&mut self, kind: TokenType) -> Option<Token> {
    if self.match_token(&kind) {
      Some(self.consume_token())
    } else {
      None
    }
  }

  fn is_end(&mut self) -> bool {
    self.match_token(&TokenType::EOF)
  }

  fn skip_comments(&mut self) {
    while self.lexer.peek_token().is_comment() {
      self.lexer.next_token();
    }
  }

  fn current_range(&self) -> Range {
    Range::default()
  }

  fn report_unexpected_token(&self, token: Token) -> ! {
    let message = format!("unexpected token '{}'", token.kind.to_string());
    self.report_error(message, token)
  }

  fn report_token_with_message(&self, message: String, token: Token) -> ! {
    self.report_error(message, token)
  }

  fn report_error(&self, message: String, token: Token) -> ! {
    report_and_exit(&message, &token.range, &self.lexer.get_source())
  }

  pub fn get_source(&self) -> &Source<'a> {
    self.lexer.get_source()
  }
}
