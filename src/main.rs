use clap::Parser as ClapParser;
use lambdac::{cli::Cli, context::Context, driver::Driver};

fn main() {
  let cli = Cli::parse();
  let options = cli.into_options();

  let mut ctx = Context::new(options);
  let mut driver = Driver::new(&mut ctx);

  if driver.run().is_err() {
    driver.print_messages();
    std::process::exit(1);
  }

  if !driver.ctx.options.quiet && !driver.ctx.messages.list.is_empty() {
    driver.print_messages();
  }
}
