pub mod options;

use clap::Parser;
use std::path::PathBuf;

use self::options::{ColorChoice, Emit, Encoding, Options};

#[derive(Debug, Parser)]
#[command(
  name = "lambdac",
  version = env!("CARGO_PKG_VERSION"),
  about = env!("CARGO_PKG_DESCRIPTION"),
  help_template = "\
{about}

Usage: {usage}

Options:
{options}
",
  disable_help_subcommand = true,
)]
pub struct Cli {
  #[arg(value_name = "INPUT", help = "Input source file", required_unless_present = "input_flag")]
  pub input: Option<PathBuf>,

  #[arg(short = 'i', long = "input", value_name = "INPUT", help = "Input source file")]
  pub input_flag: Option<PathBuf>,

  #[arg(short = 'o', long = "output", value_name = "PATH", help = "Output file path")]
  pub output: Option<PathBuf>,

  #[arg(long, help = "Type check only")]
  pub check: bool,

  #[arg(
    long,
    value_name = "KIND",
    hide_possible_values = true,
    value_enum,
    help = "Output representation (term, ast, types, ir)"
  )]
  pub emit: Option<Emit>,

  #[arg(
    long,
    value_name = "ENCODING",
    num_args = 0..=1,
    default_missing_value = "church",
    require_equals = true,
    hide_possible_values = true,
    value_enum,
    help = "Decode lambda encoding (church, scott, boehm)"
  )]
  pub decode: Option<Encoding>,

  #[arg(long, help = "Show evaluation steps")]
  pub trace: bool,

  #[arg(long, help = "Show execution statistics")]
  pub stats: bool,

  #[arg(
    long,
    value_name = "COUNT",
    default_value = "10000",
    help = "Maximum evaluation steps limit"
  )]
  pub limit: usize,

  #[arg(short = 'v', long, help = "Verbose output")]
  pub verbose: bool,

  #[arg(short = 'q', long, help = "Quiet output")]
  pub quiet: bool,

  #[arg(
    long,
    value_name = "WHEN",
    default_value = "auto",
    value_enum,
    help = "Coloring: auto, always, never"
  )]
  pub color: ColorChoice,

  #[arg(long, help = "Disable color output")]
  pub no_color: bool,
}

impl Cli {
  pub fn into_options(self) -> Options {
    let input = self
      .input
      .or(self.input_flag)
      .expect("input file is required");

    let color = if self.no_color {
      ColorChoice::Never
    } else {
      self.color
    };

    Options {
      input,
      output: self.output,
      check: self.check,
      emit: self.emit,
      decode: self.decode,
      trace: self.trace,
      stats: self.stats,
      limit: self.limit,
      verbose: self.verbose,
      quiet: self.quiet,
      color,
    }
  }
}
