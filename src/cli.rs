use clap::{Arg, Command};

pub fn command_line() -> clap::ArgMatches {
  let matches = Command::new("Hoshi")
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .subcommand_required(true)
    .arg_required_else_help(true)
    .subcommand(
      Command::new("compile")
        .about("compile hoshi sintax to native sql.")
        .arg(Arg::new("file").help("the hoshi file to compile.").required(true)),
    )
    .subcommand(
      Command::new("check")
        .about("check the syntax of the hoshi sql.")
        .arg(Arg::new("file").help("the hoshi sql file to check.").required(true)),
    )
    .subcommand(
      Command::new("run")
        .about("run the compiled hoshi sql.")
        .arg(Arg::new("file").help("the compiled hoshi sql file.").required(true)),
    )
    .get_matches();

  return matches;
}
