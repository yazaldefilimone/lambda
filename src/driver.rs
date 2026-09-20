use crate::{
  cli::options::{Emit, Encoding},
  context::Context,
  core::TermId,
  evaluator::Evaluator,
  parser::{Parser, lexer::Lexer, tokens::Token},
  printer::Printer,
  result,
  source::file::FileId,
};

pub struct Driver<'a> {
  pub ctx: &'a mut Context,
}

impl<'a> Driver<'a> {
  pub fn new(ctx: &'a mut Context) -> Self {
    Self { ctx }
  }

  pub fn run(&mut self) -> result::Result<()> {
    let file = self.load()?;
    let tokens = self.lex(file)?;

    let term = self.parse(file, &tokens)?;

    if self.ctx.options.check {
      return Ok(());
    }

    let term_stats = if self.ctx.options.stats {
      let stats = crate::stats::TermStats::compute(term, self.ctx);
      Some(stats)
    } else {
      None
    };

    if let Some(emit) = self.ctx.options.emit {
      match emit {
        Emit::Term => {
          let printer = Printer::new(self.ctx);
          println!("{}", printer.print_term(term));
          return Ok(());
        },
        Emit::Ast => {
          eprintln!("ast: {:?}", term);
          return Ok(());
        },
        Emit::Types => {
          return Ok(());
        },
        Emit::Ir => {
          return Ok(());
        },
      }
    }

    let trace = self.ctx.options.trace;
    let mut strategy_options = crate::evaluator::StrategyOptions::default();
    strategy_options.limit = self.ctx.options.limit;
    let mut evaluator = Evaluator::new(self.ctx, strategy_options, trace);
    let evaluated = evaluator.eval(term)?;
    let eval_stats = evaluator.stats;

    if let Some(encoding) = self.ctx.options.decode {
      match encoding {
        Encoding::Church => {
          let mut decoder = crate::decode::Decoder::new(self.ctx);
          match decoder.church(evaluated) {
            Ok(value_id) => {
              let printed = decoder.print(value_id);
              println!("{printed}");
            },
            Err(_) => {
              let printer = Printer::new(self.ctx);
              let printed = printer.print_term(evaluated);
              println!("{printed}");
            },
          }
        },
        Encoding::Scott | Encoding::Boehm => {
          let printer = Printer::new(self.ctx);
          let printed = printer.print_term(evaluated);
          println!("{printed}");
        },
      }

      if let Some(term_stats) = term_stats {
        println!();
        let stats = crate::stats::Stats::new(term_stats, eval_stats);
        stats.print();
      }

      return Ok(());
    }

    if let Some(term_stats) = term_stats {
      if self.ctx.options.verbose {
        let printer = Printer::new(self.ctx);
        let printed = printer.print_term(evaluated);
        println!("{printed}");
        println!();
      }
      let stats = crate::stats::Stats::new(term_stats, eval_stats);
      stats.print();
      return Ok(());
    }

    let printer = Printer::new(self.ctx);
    let printed = printer.print_term(evaluated);
    println!("{printed}");

    Ok(())
  }

  pub fn load(&mut self) -> result::Result<FileId> {
    let input = self.ctx.options.input.clone();
    self.ctx.loader.load(input, &mut self.ctx.messages)
  }

  pub fn lex(&mut self, file: FileId) -> result::Result<Vec<Token>> {
    let source = self.ctx.loader.file_source(file)?;
    let mut lexer = Lexer::new(&source, file, &mut self.ctx.symbols);
    Ok(lexer.tokenize())
  }

  pub fn parse(&mut self, _file: FileId, tokens: &[Token]) -> result::Result<TermId> {
    let mut parser =
      Parser::new(tokens, &mut self.ctx.messages, &mut self.ctx.terms, &mut self.ctx.types);
    let term = parser.parse_module()?;

    if self.ctx.messages.has_errors() {
      return Err(result::Failed::Abort);
    }

    Ok(term)
  }

  pub fn print_messages(&self) {
    for message in &self.ctx.messages.list {
      if let Some(span) = message.span {
        if let Some(file) = self.ctx.loader.sources.get(span.file) {
          let loc = self.ctx.loader.sources.location(span);
          eprintln!(
            "{}:{}:{}: {}: {}",
            file.path.display(),
            loc.line,
            loc.column + 1,
            message.severity.name(),
            message.text
          );

          if let Some(line_start) = file.line_start(loc.line as usize - 1) {
            let line_end = file
              .line_start(loc.line as usize)
              .unwrap_or(file.source.len() as u32);
            let line_str =
              &file.source[line_start as usize..line_end as usize].trim_end_matches(['\r', '\n']);
            eprintln!("  |");
            eprintln!("{:4} | {}", loc.line, line_str);
            let indent = " ".repeat(loc.column as usize);
            eprintln!("  | {}^", indent);
          }
          continue;
        }
      }
      eprintln!("{}: {}", message.severity.name(), message.text);
    }
  }
}
