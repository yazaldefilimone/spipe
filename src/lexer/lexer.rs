#![allow(dead_code)]

use crate::utils::range::Range;
use crate::utils::source::Source;
use crate::{diagnostics::report::report_and_exit, utils::match_number};

use super::token::{Token, TokenType};

pub struct Lexer<'a> {
  source: &'a Source<'a>,
  cursor: usize,
  cached: Option<Token>,
  range_start: usize,
}

impl<'a> Lexer<'a> {
  pub fn new(source: &'a Source<'a>) -> Self {
    Self { source, cursor: 0, cached: None, range_start: 0 }
  }

  pub fn peek_token(&mut self) -> Token {
    if self.cached.is_none() {
      self.cached = Some(self.read_next_token());
    }
    self.cached.clone().unwrap()
  }

  pub fn next_token(&mut self) -> Token {
    if let Some(token) = self.cached.take() {
      return token;
    }
    self.read_next_token()
  }

  pub fn get_source(&self) -> &Source<'a> {
    self.source
  }

  fn read_next_token(&mut self) -> Token {
    self.skip_whitespace();
    self.update_current_range();
    if self.is_end() {
      return Token::create_simple_token(TokenType::EOF, self.create_range());
    }
    let current_char = self.peek_one();
    match current_char {
      '[' => self.read_simple_token(TokenType::LeftBracket),
      ']' => self.read_simple_token(TokenType::RightBracket),
      '+' => self.read_simple_token(TokenType::Plus),
      '-' => self.read_line_comment(),
      '*' => self.read_simple_token(TokenType::Asterisk),
      '/' => self.read_simple_token(TokenType::Slash),
      '%' => self.read_simple_token(TokenType::Percent),
      '=' => self.read_simple_token(TokenType::Equal),
      '(' => self.read_simple_token(TokenType::LeftParen),
      ')' => self.read_simple_token(TokenType::RightParen),
      ',' => self.read_simple_token(TokenType::Comma),
      ';' => self.read_simple_token(TokenType::Semicolon),
      '.' => self.read_simple_token(TokenType::Dot),
      '<' => self.read_check_ahead("<=", TokenType::LessThan, TokenType::LessThanOrEqual),
      '>' => self.read_check_ahead(">=", TokenType::GreaterThan, TokenType::GreaterThanOrEqual),
      '0'..='9' => self.read_number(),
      '"' => self.read_string_with_double_quote(),
      '\'' => self.read_string_with_single_quote(),
      '!' => self.read_bang(),
      '|' => self.read_pipe(),
      'a'..='z' | 'A'..='Z' | '_' => self.read_identifier(),
      _ => {
        let range = self.create_range();
        let message = format!("unexpected character '{}'", current_char);
        report_and_exit(&message, &range, &self.source);
      }
    }
  }

  fn read_bang(&mut self) -> Token {
    if self.starts_with("!=") {
      self.advance_many(2);
      let range = self.create_range();
      Token::new(TokenType::NotEqual, None, range)
    } else {
      let range = self.create_range();
      let message = format!("expected `!=` but got `{}`", self.peek_many(2));
      report_and_exit(&message, &range, &self.source);
    }
  }
  fn read_pipe(&mut self) -> Token {
    if self.starts_with("|>") {
      self.advance_many(2);
      let range = self.create_range();
      Token::new(TokenType::Pipe, None, range)
    } else {
      let range = self.create_range();
      let message = format!("expected `|>` but got `{}`", self.peek_many(2));
      report_and_exit(&message, &range, &self.source);
    }
  }
  fn read_simple_token(&mut self, kind: TokenType) -> Token {
    self.advance_one();
    let range = self.create_range();
    Token::create_simple_token(kind, range)
  }

  fn read_check_ahead(&mut self, expected: &str, single_kind: TokenType, double_kind: TokenType) -> Token {
    if self.starts_with(expected) {
      self.advance_many(expected.len());
      let range = self.create_range();
      return Token::create_simple_token(double_kind, range);
    }
    self.read_simple_token(single_kind)
  }

  fn read_line_comment(&mut self) -> Token {
    if self.starts_with("--") {
      self.consume_expect("--");
      let text = self.read_while(|c| c != '\n');
      let range = self.create_range();
      Token::new(TokenType::Comment, Some(text), range)
    } else {
      self.read_simple_token(TokenType::Minus)
    }
  }

  fn read_identifier(&mut self) -> Token {
    let text = self.read_while(|c| c.is_ascii_alphabetic() || c == '_' || c == '$' || c.is_ascii_digit());
    let range = self.create_range();
    Token::create_identifier(range, text)
  }

  fn read_number(&mut self) -> Token {
    let number = self.read_while(match_number);
    let range = self.create_range();
    Token::new(TokenType::Number, Some(number), range)
  }

  fn read_string_with_double_quote(&mut self) -> Token {
    self.consume_expect("\"");
    let string = self.read_while(|c| c != '"' && c != '\n');
    self.consume_expect_with_custom_error("\"", "unterminated string literal");
    let range = self.create_range();
    Token::new(TokenType::String, Some(string), range)
  }

  fn read_string_with_single_quote(&mut self) -> Token {
    self.consume_expect("'");
    let string = self.read_while(|c| c != '\'' && c != '\n');
    self.consume_expect_with_custom_error("'", "unterminated string literal");
    let range = self.create_range();
    Token::new(TokenType::String, Some(string), range)
  }

  fn read_while(&mut self, mut test: impl FnMut(char) -> bool) -> String {
    let range_start = self.cursor;
    while !self.is_end() && test(self.peek_one()) {
      self.advance_one();
    }
    self.source.raw[range_start..self.cursor].to_string()
  }

  fn advance_one(&mut self) {
    if let Some(c) = self.source.raw[self.cursor..].chars().next() {
      self.cursor += c.len_utf8();
    }
  }
  fn create_range(&mut self) -> Range {
    let start = self.range_start;
    self.range_start = self.cursor;
    Range { start, end: self.cursor }
  }

  fn update_current_range(&mut self) {
    self.range_start = self.cursor;
  }

  fn consume_expect(&mut self, text: &str) {
    if self.starts_with(text) {
      self.advance_many(text.len());
    } else {
      let range = self.create_range();
      let got = self.peek_many(text.len());
      let message = format!("expected `{}` but got `{}`", text, got);
      report_and_exit(&message, &range, &self.source);
    }
  }

  fn consume_expect_with_custom_error(&mut self, text: &str, error_message: &str) {
    if self.starts_with(text) {
      self.advance_many(text.len());
    } else {
      let range = self.create_range();
      report_and_exit(&error_message, &range, &self.source);
    }
  }

  pub fn is_end(&self) -> bool {
    self.cursor >= self.source.raw.len()
  }

  fn peek_one(&self) -> char {
    self.source.raw[self.cursor..].chars().next().unwrap_or('\0')
  }

  fn peek_many(&self, count: usize) -> String {
    self.source.raw[self.cursor..].chars().take(count).collect()
  }

  fn advance_many(&mut self, count: usize) {
    for _ in 0..count {
      self.advance_one();
    }
  }

  fn starts_with(&self, s: &str) -> bool {
    self.source.raw[self.cursor..].starts_with(s)
  }

  fn skip_whitespace(&mut self) {
    while !self.is_end() && self.peek_one().is_whitespace() {
      self.advance_one();
    }
  }
}
