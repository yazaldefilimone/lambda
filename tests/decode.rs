use lambdac::{
  cli::options::{DecodePreference, Encoding, Options},
  context::Context,
  driver::Driver,
};

fn eval_and_decode(code: &str, prefer: Option<DecodePreference>) -> String {
  let mut options = Options::default();
  options.decode = Some(Encoding::Church);
  options.prefer = prefer;

  let mut ctx = Context::new(options);
  let mut driver = Driver::new(&mut ctx);
  let term = driver.compile_source("test", code).expect("compile error");
  let (evaluated, _) = driver.evaluate_term(term).expect("evaluation error");

  let mut decoder = lambdac::decode::Decoder::new(&mut driver.ctx);
  let value_id = decoder.church(evaluated).expect("decode error");
  decoder.print(value_id)
}

#[test]
fn test_church_zero_ambiguity_resolution() {
  // Church zero / false representation: \f x. x
  let code = "(λf x. x)";

  // Default preference (bool first)
  assert_eq!(eval_and_decode(code, None), "false");

  // Prefer number
  assert_eq!(eval_and_decode(code, Some(DecodePreference::Number)), "0");

  // Prefer boolean
  assert_eq!(eval_and_decode(code, Some(DecodePreference::Boolean)), "false");
}

#[test]
fn test_fallback_when_preferred_type_fails() {
  // Church true: \t f. t (cannot be a church number)
  let true_code = "(λt f. t)";

  // Even when preferring number, it should fall back to boolean
  assert_eq!(eval_and_decode(true_code, Some(DecodePreference::Number)), "true");

  // Church one: \f x. f x (cannot be a boolean)
  let one_code = "(λf x. f x)";

  // Even when preferring boolean, it should fall back to number
  assert_eq!(eval_and_decode(one_code, Some(DecodePreference::Boolean)), "1");
}

#[test]
fn test_nested_decode_with_preference() {
  // A pair of Church zeroes: (\a b x. x a b) (\f x. x) (\f x. x)
  let pair_code = "(λa b x. x a b) (λf x. x) (λf x. x)";

  // With number preference, pair contents are decoded as numbers
  assert_eq!(eval_and_decode(pair_code, Some(DecodePreference::Number)), "(0, 0)");

  // With boolean preference, pair contents are decoded as booleans
  assert_eq!(eval_and_decode(pair_code, Some(DecodePreference::Boolean)), "(false, false)");
}

#[test]
fn test_cli_prefer_parsing() {
  use clap::Parser;
  use lambdac::cli::Cli;

  let cli =
    Cli::try_parse_from(["lambdac", "file.lam", "--decode", "church", "--prefer", "number"])
      .unwrap();
  let opts = cli.into_options();
  assert_eq!(opts.prefer, Some(DecodePreference::Number));
  assert_eq!(opts.decode, Some(Encoding::Church));

  let cli = Cli::try_parse_from(["lambdac", "file.lam", "--prefer", "int"]).unwrap();
  let opts = cli.into_options();
  assert_eq!(opts.prefer, Some(DecodePreference::Number));
  assert_eq!(opts.decode, Some(Encoding::Church));

  let cli = Cli::try_parse_from(["lambdac", "file.lam", "--prefer", "bool"]).unwrap();
  let opts = cli.into_options();
  assert_eq!(opts.prefer, Some(DecodePreference::Boolean));

  let cli = Cli::try_parse_from(["lambdac", "file.lam", "--prefer", "boolean"]).unwrap();
  let opts = cli.into_options();
  assert_eq!(opts.prefer, Some(DecodePreference::Boolean));
}
