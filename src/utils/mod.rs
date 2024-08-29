pub mod range;
pub mod source;

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

pub fn highlight_text_with_green(text: &str) -> String {
  format!("\x1b[32m{}\x1b[0m", text)
}
