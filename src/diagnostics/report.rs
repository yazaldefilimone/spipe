use crate::utils::{
  highlight_text_with_cyan, highlight_text_with_green, highlight_text_with_red, highlight_text_with_white,
  highlight_text_with_yellow,
};

use crate::utils::range::Range;
use crate::utils::source::Source;

use code_highlighter::{highlight_error, highlight_warning};

pub fn report_error(message: &str, hint: &Option<String>, range: &Range, source: &Source, warning: bool) {
  println!("");
  if !warning {
    println!("{} {}", highlight_text_with_red("ERROR >>>"), highlight_text_with_white(message));
  } else {
    let warning = highlight_text_with_yellow("WARNING >>>");
    let message = format!("{} {}", warning, highlight_text_with_white(message));
    println!("{}", message);
  }
  let file_highlight = highlight_text_with_cyan(source.path);
  println!("{}", file_highlight);
  println!("");
  if warning {
    let code_highliter = format!("{}", highlight_warning(range.start, range.end, source.raw));
    println!("{}", code_highliter);
  } else {
    let code_highliter = format!("{}", highlight_error(range.start, range.end, source.raw));
    println!("{}", code_highliter);
  }

  if let Some(hint) = hint {
    println!();

    println!("{} {}", highlight_text_with_green("HELP:"), highlight_text_with_white(hint));
  }
  println!();
}

pub fn report_and_exit(message: &str, range: &Range, source: &Source) -> ! {
  report_error(message, &None, range, source, false);
  std::process::exit(1);
}

fn is_warning(message: &str) -> bool {
  message.contains("WARNING")
}

fn is_parser_or_lexer_error(message: &str) -> bool {
  !message.contains("ERROR") && !message.contains("WARNING")
}
