use std::fs;
use std::hint::black_box;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use lambdac::{
  core::{Term, Type},
  messages::Messages,
  parser::{lexer::Lexer, parser::Parser},
  source::file::FileId,
  symbol::interner::Interner,
};

fn bench_parser(c: &mut Criterion) {
  let mut group = c.benchmark_group("parser");

  let benchmarks = [
    ("pred", "examples/church/pred.lam"),
    ("factorial", "examples/recursion/factorial.lam"),
    ("arithmetic", "examples/church/arithmetic.lam"),
    ("list", "examples/church/list.lam"),
    ("deep", "examples/stress/deep.lam"),
  ];

  for (name, path) in benchmarks {
    let source = fs::read_to_string(path).expect("Failed to read file");
    let mut symbols = Interner::new();
    let mut lexer = Lexer::new(&source, FileId(0), &mut symbols);
    let tokens = lexer.tokenize();

    group.throughput(Throughput::Elements(tokens.len() as u64));

    group.bench_function(name, |b| {
      b.iter(|| {
        let mut messages = Messages::new();
        let mut terms = Term::new();
        let mut types = Type::new();
        let mut parser = Parser::new(&tokens, &mut messages, &mut terms, &mut types);
        let term = parser.parse_module().expect("parse failed");
        black_box(term);
      });
    });
  }

  group.finish();
}

criterion_group!(benches, bench_parser);
criterion_main!(benches);
