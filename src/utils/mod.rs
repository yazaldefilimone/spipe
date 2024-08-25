pub mod range;

pub struct Source<'a> {
  pub raw: &'a str,
  pub path: &'a str,
}

impl<'a> Source<'a> {
  pub fn new(path: &'a str, raw: &'a str) -> Source<'a> {
    Source { path, raw }
  }
}

pub fn match_number(character: char) -> bool {
  "1234567890.".contains(character)
}
pub fn highlight_text_with_red(text: &str) -> String {
  format!("\x1b[31m{}\x1b[0m", text)
}

pub fn highlight_text_with_yellow(text: &str) -> String {
  format!("\x1b[33m{}\x1b[0m", text)
}
pub fn highlight_text_with_cyan(text: &str) -> String {
  format!("\x1b[36m{}\x1b[0m", text)
}

pub fn highlight_text_with_white(text: &str) -> String {
  format!("\x1b[97m{}\x1b[0m", text)
}
