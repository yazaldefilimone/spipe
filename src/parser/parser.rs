#![allow(dead_code)]

use super::ast::*;
use crate::diagnostics::maneger::Diagnostic;
use crate::diagnostics::report::report_and_exit;
use crate::lexer::Token;
use crate::lexer::{Lexer, TokenType};
use crate::utils::range::Range;

type Result<T> = std::result::Result<T, Diagnostic>;

pub struct Parser<'a> {
  lexer: &'a mut Lexer<'a>,
}

impl<'a> Parser<'a> {
  pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
    Self { lexer }
  }

  pub fn parse_program(&mut self) -> Result<Program> {
    let mut statements = Vec::new();
    while !self.is_end() {
      let stmt = self.parse_statement();
      match stmt {
        Ok(stmt) => statements.push(stmt),
        Err(err) => {
          report_and_exit(&err.message, &err.range, &self.lexer.get_source());
        }
      }
      if self.match_token_and_consume(TokenType::Pipe).is_some() {
        continue;
      }
      if !self.is_end() {
        self.consume_expect_token(TokenType::Semicolon);
      }
    }

    Ok(Program::new(statements))
  }

  fn parse_statement(&mut self) -> Result<Statement> {
    self.skip_comments();
    let token = self.lexer.peek_token();
    match token.kind {
      TokenType::Select => self.parse_select_statement(),
      TokenType::From => self.parse_from_clause(),
      TokenType::Join => self.parse_join_clause(),
      TokenType::Where => self.parse_where_clause(),
      TokenType::GroupBy => self.parse_group_by_clause(),
      TokenType::Aggregate => self.parse_aggregate_clause(),
      TokenType::OrderBy => self.parse_order_by_clause(),
      TokenType::Limit => self.parse_limit_clause(),
      _ => Err(self.report_unexpected_token(token)),
    }
  }

  fn parse_select_statement(&mut self) -> Result<Statement> {
    self.consume_expect_token(TokenType::Select);
    let distinct = self.match_token_and_consume(TokenType::Distinct).is_some();
    let mut columns = Vec::new();
    loop {
      columns.push(self.parse_column_expression(None)?);
      if self.match_token_and_consume(TokenType::Comma).is_none() {
        break;
      }
    }

    Ok(Statement::Select(SelectStatement {
      distinct,
      columns,
      range: Range::default(), // You'll need to implement proper range tracking
    }))
  }

  fn parse_from_clause(&mut self) -> Result<Statement> {
    self.consume_expect_token(TokenType::From);
    let table = self.parse_identifier()?;

    Ok(Statement::From(FromClause {
      table,
      range: Range::default(), // You'll need to implement proper range tracking
    }))
  }

  fn parse_join_clause(&mut self) -> Result<Statement> {
    let join_type = match self.lexer.peek_token().kind {
      TokenType::Join => JoinType::Inner,
      // TokenType::LeftJoin => JoinType::Left,
      // TokenType::RightJoin => JoinType::Right,
      // TokenType::FullJoin => JoinType::Full,
      _ => {
        return {
          let token = self.lexer.peek_token();
          Err(self.report_unexpected_token(token))
        }
      }
    };
    self.consume_token(); // Consume the join token

    let table = self.parse_identifier()?;
    self.consume_expect_token(TokenType::On);
    let on = self.parse_condition_expression()?;

    Ok(Statement::Join(JoinClause {
      join_type,
      table,
      on,
      range: Range::default(), // You'll need to implement proper range tracking
    }))
  }

  fn parse_where_clause(&mut self) -> Result<Statement> {
    self.consume_expect_token(TokenType::Where);
    let condition = self.parse_condition_expression()?;

    Ok(Statement::Where(WhereClause {
      condition,
      range: Range::default(), // You'll need to implement proper range tracking
    }))
  }

  fn parse_group_by_clause(&mut self) -> Result<Statement> {
    self.consume_expect_token(TokenType::GroupBy);

    let mut columns = Vec::new();
    loop {
      columns.push(self.parse_column_expression(None)?);
      if self.match_token_and_consume(TokenType::Comma).is_none() {
        break;
      }
    }

    Ok(Statement::GroupBy(GroupByClause {
      columns,
      range: Range::default(), // You'll need to implement proper range tracking
    }))
  }

  fn parse_aggregate_clause(&mut self) -> Result<Statement> {
    self.consume_expect_token(TokenType::Aggregate);

    let mut functions = Vec::new();
    loop {
      functions.push(self.parse_aggregate_function()?);
      if self.match_token_and_consume(TokenType::Comma).is_none() {
        break;
      }
    }

    Ok(Statement::Aggregate(AggregateClause {
      functions,
      range: Range::default(), // You'll need to implement proper range tracking
    }))
  }

  fn parse_aggregate_function(&mut self) -> Result<AggregateFunction> {
    let function = match self.lexer.peek_token().kind {
      TokenType::Count => AggregateFunctionType::Count,
      TokenType::Sum => AggregateFunctionType::Sum,
      TokenType::Avg => AggregateFunctionType::Avg,
      TokenType::Min => AggregateFunctionType::Min,
      TokenType::Max => AggregateFunctionType::Max,
      _ => {
        let token = self.lexer.peek_token();
        return Err(self.report_unexpected_token(token));
      }
    };
    self.consume_token(); // Consume the function name

    self.consume_expect_token(TokenType::LeftParen);
    let argument = self.parse_expression()?;
    self.consume_expect_token(TokenType::RightParen);

    let alias = if self.match_token_and_consume(TokenType::As).is_some() {
      Some(self.parse_identifier()?)
    } else {
      None
    };

    Ok(AggregateFunction {
      function,
      argument,
      alias,
      range: Range::default(), // You'll need to implement proper range tracking
    })
  }

  fn parse_order_by_clause(&mut self) -> Result<Statement> {
    self.consume_expect_token(TokenType::OrderBy);

    let mut columns = Vec::new();
    loop {
      let column = self.parse_column_expression(None)?;
      let direction = if self.match_token_and_consume(TokenType::Asc).is_some() {
        OrderDirection::Asc
      } else if self.match_token_and_consume(TokenType::Desc).is_some() {
        OrderDirection::Desc
      } else {
        OrderDirection::Asc // Default to ascending if not specified
      };

      columns.push(OrderByColumn {
        column,
        direction,
        range: Range::default(), // You'll need to implement proper range tracking
      });

      if self.match_token_and_consume(TokenType::Comma).is_none() {
        break;
      }
    }

    Ok(Statement::OrderBy(OrderByClause {
      columns,
      range: Range::default(), // You'll need to implement proper range tracking
    }))
  }

  fn parse_limit_clause(&mut self) -> Result<Statement> {
    self.consume_expect_token(TokenType::Limit);
    let count = self.parse_number_literal()?;

    let offset = if self.match_token_and_consume(TokenType::Offset).is_some() {
      Some(self.parse_number_literal()?)
    } else {
      None
    };

    Ok(Statement::Limit(LimitClause {
      count,
      offset,
      range: Range::default(), // You'll need to implement proper range tracking
    }))
  }

  fn parse_column_expression(&mut self, table_: Option<Identifier>) -> Result<ColumnExpression> {
    let name = if table_.is_none() { self.parse_identifier()? } else { table_.unwrap() };

    let table = if self.lexer.peek_token().kind == TokenType::Dot {
      self.consume_expect_token(TokenType::Dot);
      let table = self.parse_identifier()?;
      Some(table)
    } else {
      None
    };
    Ok(ColumnExpression {
      table,
      name,
      range: Range::default(), // You'll need to implement proper range tracking
    })
  }

  fn parse_condition_expression(&mut self) -> Result<ConditionExpression> {
    let left = Box::new(self.parse_expression()?);
    let operator = self.parse_operator()?;
    let right = Box::new(self.parse_expression()?);

    Ok(ConditionExpression {
      left,
      operator,
      right,
      range: Range::default(), // You'll need to implement proper range tracking
    })
  }

  fn parse_expression(&mut self) -> Result<Expression> {
    let token = self.lexer.peek_token();
    match token.kind {
      TokenType::Identifier => {
        let identifier = self.parse_identifier()?;
        if self.lexer.peek_token().kind == TokenType::LeftParen {
          self.parse_function_call(Some(identifier)).map(Expression::FunctionCall)
        } else {
          self.parse_column_expression(Some(identifier)).map(Expression::Column)
        }
      }
      TokenType::LeftParen => self.parse_grouo_expression(),
      TokenType::String => self.parse_string_literal().map(Literal::String).map(Expression::Literal),
      TokenType::Number => self.parse_number_literal().map(Literal::Number).map(Expression::Literal),
      TokenType::Boolean => self.parse_boolean_literal().map(Literal::Boolean).map(Expression::Literal),
      _ => Err(self.report_unexpected_token(token)),
    }
  }

  fn parse_grouo_expression(&mut self) -> Result<Expression> {
    // (...)
    self.consume_expect_token(TokenType::LeftParen);
    let expression = self.parse_expression()?;
    self.consume_expect_token(TokenType::RightParen);
    Ok(expression)
  }

  fn parse_function_call(&mut self, name: Option<Identifier>) -> Result<FunctionCall> {
    self.consume_expect_token(TokenType::LeftParen);
    let name = if name.is_none() { self.parse_identifier()? } else { name.unwrap() };

    let mut arguments = Vec::new();
    if !self.match_token(&TokenType::RightParen) {
      loop {
        arguments.push(self.parse_expression()?);
        if self.match_token_and_consume(TokenType::Comma).is_none() {
          break;
        }
      }
    }

    self.consume_expect_token(TokenType::RightParen);

    Ok(FunctionCall {
      name,
      arguments,
      range: Range::default(), // You'll need to implement proper range tracking
    })
  }

  fn parse_identifier(&mut self) -> Result<Identifier> {
    let token = self.consume_expect_token(TokenType::Identifier);
    Ok(Identifier { value: token.lexeme.unwrap(), range: token.range })
  }

  fn parse_string_literal(&mut self) -> Result<StringLiteral> {
    let token = self.consume_expect_token(TokenType::String);
    Ok(StringLiteral {
      value: token.lexeme.unwrap().to_string(), // Remove quotes
      range: token.range,
    })
  }

  fn parse_number_literal(&mut self) -> Result<NumberLiteral> {
    let token = self.consume_expect_token(TokenType::Number);
    Ok(NumberLiteral { value: token.lexeme.unwrap(), range: token.range })
  }

  fn parse_boolean_literal(&mut self) -> Result<BooleanLiteral> {
    let token = self.consume_token();
    Ok(BooleanLiteral { value: token.lexeme.unwrap() == "true", range: token.range })
  }

  fn parse_operator(&mut self) -> Result<Operator> {
    let token = self.consume_token();
    match token.kind {
      TokenType::Equal => Ok(Operator::Equal),
      TokenType::NotEqual => Ok(Operator::NotEqual),
      TokenType::LessThan => Ok(Operator::LessThan),
      TokenType::GreaterThan => Ok(Operator::GreaterThan),
      TokenType::LessThanOrEqual => Ok(Operator::LessThanOrEqual),
      TokenType::GreaterThanOrEqual => Ok(Operator::GreaterThanOrEqual),
      TokenType::And => Ok(Operator::And),
      TokenType::Or => Ok(Operator::Or),
      _ => Err(self.report_unexpected_token(token)),
    }
  }

  fn consume_expect_token(&mut self, kind: TokenType) -> Token {
    let token = self.lexer.next_token();
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

  fn match_any_token(&mut self, kinds: &[TokenType]) -> Option<Token> {
    let token = self.lexer.peek_token();
    if kinds.contains(&token.kind) {
      Some(self.consume_token())
    } else {
      None
    }
  }

  fn match_token_and_consume(&mut self, kind: TokenType) -> Option<Token> {
    if self.match_token(&kind) {
      Some(self.consume_token())
    } else {
      None
    }
  }

  fn contains_token(&mut self, kinds: &[TokenType]) -> bool {
    if self.is_end() {
      return false;
    }
    let next_token = self.lexer.peek_token();
    kinds.iter().any(|kind| &next_token.kind == kind)
  }

  fn is_end(&mut self) -> bool {
    self.match_token(&TokenType::EOF)
  }

  fn skip_comments(&mut self) {
    while self.lexer.peek_token().is_comment() {
      self.lexer.next_token();
    }
  }

  fn report_unexpected_token(&self, token: Token) -> Diagnostic {
    let message = format!("unexpected token '{}'", token.kind.to_string());
    return Diagnostic { message, range: token.range };
    // self.report_error(message, token)
  }

  fn report_error(&self, message: String, token: Token) -> ! {
    let diagnostic = Diagnostic { message, range: token.range };
    report_and_exit(&diagnostic.message, &diagnostic.range, &self.lexer.get_source());
  }

  fn report_diagnostic(&self, diagnostic: Diagnostic) -> ! {
    report_and_exit(&diagnostic.message, &diagnostic.range, &self.lexer.get_source());
  }
}
