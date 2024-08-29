use checker::Checker;
use lexer::Lexer;
use parser::Parser;
use utils::source::Source;

mod checker;
mod cli;
mod diagnostics;
mod emiter;
mod format;
mod lexer;
mod parser;
mod utils;
fn main() {
  let matches = cli::command_line();
  match matches.subcommand() {
    Some(("compile", matches)) => {
      let path_name = matches.get_one::<String>("file").unwrap();
      run_compile(path_name);
    }
    Some(("check", matches)) => {
      let path_name = matches.get_one::<String>("file").unwrap();
      run_check(path_name);
    }
    Some(("run", matches)) => {
      let path_name = matches.get_one::<String>("file").unwrap();
      run_execute(path_name);
    }
    _ => {}
  }
}

fn load_file(path_name: &str) -> String {
  let raw = std::fs::read_to_string(path_name).expect(format!("ERROR: cannot open file '{}'", path_name).as_str());
  raw
}
fn run_compile(path_name: &str) {
  let raw = load_file(path_name);
  let source = Source::new(path_name, &raw);
  let mut lexer = Lexer::new(&source);
  let mut parser = Parser::new(&mut lexer);
  let program = parser.parse();
  // println!("{:#?}", program);
  let mut checker = Checker::new();
  checker.check(&program);
  checker.report(&source);
  if checker.contains_error() {
    std::process::exit(1);
  }
  let native = program.to_sql();
  println!("{}", native);

  // println!("{:#?}", program);
}
fn run_check(path_name: &str) {
  println!("{:?}", path_name);
}

fn run_execute(path_name: &str) {
  println!("{:?}", path_name);
}
