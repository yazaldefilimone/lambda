use std::io::{self, BufWriter, IsTerminal, Stdout, Write};

use crate::{
  cli::options::ColorChoice, context::Context, core::TermId, printer::Printer,
  trace::event::Reduction,
};

const DIM: &str = "\x1b[90m";
const CYAN: &str = "\x1b[36m";
const YELLOW: &str = "\x1b[33m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

pub struct TextTracer {
  color: bool,
  first: bool,
  buffered: Option<TermId>,
  out: BufWriter<Stdout>,
}

impl TextTracer {
  pub fn new(ctx: &Context) -> Self {
    let no_color_env = std::env::var_os("NO_COLOR").is_some();
    let color = match ctx.options.color {
      ColorChoice::Always => true,
      ColorChoice::Never => false,
      ColorChoice::Auto => !no_color_env && io::stdout().is_terminal(),
    };
    let out = BufWriter::new(io::stdout());
    Self { color, first: true, buffered: None, out }
  }

  #[allow(dead_code)]
  pub fn with_color(color: bool) -> Self {
    let out = BufWriter::new(io::stdout());
    Self { color, first: true, buffered: None, out }
  }

  pub fn on_step(&mut self, ctx: &Context, reduction: &Reduction) {
    let printer = Printer::new(ctx);

    if self.first {
      let before = printer.print_term(reduction.before);
      let _ = writeln!(self.out, "{before}");
      self.first = false;
    } else if let Some(prev) = self.buffered.take() {
      let prev_str = printer.print_term(prev);
      let _ = writeln!(self.out, "{prev_str}");
    }

    self.write_transition(ctx, &printer, reduction);
    self.buffered = Some(reduction.after);
  }

  fn write_transition(&mut self, ctx: &Context, printer: &Printer, reduction: &Reduction) {
    let rule = reduction.rule.symbol();

    if let Some(redex) = &reduction.redex {
      let var = redex.variable.map_or("x", |s| ctx.symbols.resolve(s));
      let val = printer.print_term_in_scope(redex.argument, &ctx.redex_scope);

      if self.color {
        let _ =
          writeln!(self.out, "{DIM}│{RESET} {CYAN}{rule}{RESET} ({var} {YELLOW}↦{RESET} {val})");
        let _ = writeln!(self.out, "{DIM}↓{RESET}");
      } else {
        let _ = writeln!(self.out, "│ {rule} ({var} ↦ {val})");
        let _ = writeln!(self.out, "↓");
      }
    } else if self.color {
      let _ = writeln!(self.out, "{DIM}│{RESET} {CYAN}{rule}{RESET}");
      let _ = writeln!(self.out, "{DIM}↓{RESET}");
    } else {
      let _ = writeln!(self.out, "│ {rule}");
      let _ = writeln!(self.out, "↓");
    }
  }

  pub fn on_finish(&mut self, ctx: &Context, final_term: TermId) {
    let printer = Printer::new(ctx);
    let term = printer.print_term(final_term);

    if self.buffered.take().is_some() || self.first {
      if self.color {
        let _ = writeln!(self.out, "{GREEN}{term}{RESET}");
      } else {
        let _ = writeln!(self.out, "{term}");
      }
    }

    let _ = self.out.flush();
  }
}

impl Drop for TextTracer {
  fn drop(&mut self) {
    let _ = self.out.flush();
  }
}
