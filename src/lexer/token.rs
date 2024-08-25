use serde::{Deserialize, Serialize};

use crate::utils::range::Range;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum TokenType {
  //  (Keywords)
  Select,    // SELECT
  From,      // FROM
  Where,     // WHERE
  Order,     // ORDER
  By,        // BY
  Asc,       // ASC
  Desc,      // DESC
  Limit,     // LIMIT
  Offset,    // OFFSET
  Join,      // JOIN
  On,        // ON
  Group,     // GROUP
  Having,    // HAVING
  As,        // AS
  Union,     // UNION
  With,      // WITH
  Case,      // CASE
  End,       // END
  And,       // AND
  Or,        // OR
  Not,       // NOT
  Insert,    // INSERT
  Into,      // INTO
  Values,    // VALUES
  Update,    // UPDATE
  Set,       // SET
  Delete,    // DELETE
  Create,    // CREATE
  Table,     // TABLE
  Alter,     // ALTER
  Drop,      // DROP
  Distinct,  // DISTINCT
  Null,      // NULL
  Is,        // IS
  Like,      // LIKE
  In,        // IN
  Exists,    // EXISTS
  Between,   // BETWEEN
  Aggregate, // AGGREGATE

  // (Aggregation Functions)
  Count, // COUNT
  Sum,   // SUM
  Avg,   // AVG
  Min,   // MIN
  Max,   // MAX

  //  (Literals)
  Identifier, // name of a column, table, or alias
  String,     // "string"
  Boolean,    // true, false
  Number,     // 123, 123.456

  //  (Operators)
  Plus,               // +
  Minus,              // -
  Asterisk,           // *
  Slash,              // /
  Percent,            // %
  Equal,              // =
  NotEqual,           // !=
  LessThan,           // <
  GreaterThan,        // >
  LessThanOrEqual,    // <=
  GreaterThanOrEqual, // >=
  AndOperator,        // AND (operador lógico)
  OrOperator,         // OR (operador lógico)
  NotOperator,        // NOT (operador lógico)
  LikeOperator,       // LIKE
  InOperator,         // IN
  IsOperator,         // IS
  //  (Punctuation)
  Comma,        // ,
  Semicolon,    // ;
  LeftParen,    // (
  RightParen,   // )
  LeftBracket,  // [
  RightBracket, // ]
  Dot,          // .
  Pipe,         // |> (operador de pipe)

  //  (Comments)
  Comment, // -- ou /* ... */

  // (End of File)
  EOF, // end of file
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Token {
  pub kind: TokenType,
  pub lexeme: Option<String>,
  pub range: Range,
}

impl Token {
  pub fn new(kind: TokenType, lexeme: Option<String>, range: Range) -> Token {
    Token { kind, lexeme, range }
  }

  pub fn create_simple_token(kind: TokenType, range: Range) -> Token {
    Token { kind, lexeme: None, range }
  }

  pub fn is_operator(&self) -> bool {
    match self.kind {
      TokenType::Plus => true,
      TokenType::Minus => true,
      TokenType::Asterisk => true,
      TokenType::Slash => true,
      TokenType::Percent => true,
      TokenType::Equal => true,
      TokenType::NotEqual => true,
      TokenType::LessThan => true,
      TokenType::GreaterThan => true,
      TokenType::LessThanOrEqual => true,
      TokenType::GreaterThanOrEqual => true,
      TokenType::And => true,
      TokenType::Or => true,
      _ => false,
    }
  }

  pub fn is_punctuation(&self) -> bool {
    match self.kind {
      TokenType::Comma => true,
      TokenType::Semicolon => true,
      TokenType::LeftParen => true,
      TokenType::RightParen => true,
      TokenType::LeftBracket => true,
      TokenType::RightBracket => true,
      TokenType::Dot => true,
      _ => false,
    }
  }

  pub fn is_eof(&self) -> bool {
    match self.kind {
      TokenType::EOF => true,
      _ => false,
    }
  }
  pub fn is_comment(&self) -> bool {
    match self.kind {
      TokenType::Comment => true,
      _ => false,
    }
  }

  pub fn create_identifier(range: Range, text: String) -> Token {
    match text.as_str() {
      "SELECT" => Token::new(TokenType::Select, None, range),
      "FROM" => Token::new(TokenType::From, None, range),
      "WHERE" => Token::new(TokenType::Where, None, range),
      "ORDER" => Token::new(TokenType::Order, None, range),
      "BY" => Token::new(TokenType::By, None, range),
      "ASC" => Token::new(TokenType::Asc, None, range),
      "DESC" => Token::new(TokenType::Desc, None, range),
      "LIMIT" => Token::new(TokenType::Limit, None, range),
      "OFFSET" => Token::new(TokenType::Offset, None, range),
      "JOIN" => Token::new(TokenType::Join, None, range),
      "ON" => Token::new(TokenType::On, None, range),
      "GROUP" => Token::new(TokenType::Group, None, range),
      "HAVING" => Token::new(TokenType::Having, None, range),
      "AS" => Token::new(TokenType::As, None, range),
      "UNION" => Token::new(TokenType::Union, None, range),
      "WITH" => Token::new(TokenType::With, None, range),
      "CASE" => Token::new(TokenType::Case, None, range),
      "END" => Token::new(TokenType::End, None, range),
      "AGGREGATE" => Token::new(TokenType::Aggregate, None, range),
      "AND" => Token::new(TokenType::And, None, range),
      "OR" => Token::new(TokenType::Or, None, range),
      "true" => Token::new(TokenType::Boolean, Some("true".to_string()), range),
      "false" => Token::new(TokenType::Boolean, Some("false".to_string()), range),
      "COUNT" => Token::new(TokenType::Count, None, range),
      "SUM" => Token::new(TokenType::Sum, None, range),
      "AVG" => Token::new(TokenType::Avg, None, range),
      "MIN" => Token::new(TokenType::Min, None, range),
      "MAX" => Token::new(TokenType::Max, None, range),
      _ => Token::new(TokenType::Identifier, Some(text), range),
    }
  }
}
