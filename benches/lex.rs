use std::fs;
use std::hint::black_box;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use lambdac::{parser::lexer::Lexer, source::file::FileId, symbol::interner::Interner};

fn bench_lex(c: &mut Criterion) {
  let mut group = c.benchmark_group("lex");

  let benchmarks = [
    ("pred", "examples/church/pred.lam"),
    ("factorial", "examples/recursion/factorial.lam"),
    ("arithmetic", "examples/church/arithmetic.lam"),
    ("list", "examples/church/list.lam"),
    ("deep", "examples/stress/deep.lam"),
  ];

  for (name, path) in benchmarks {
    let source = fs::read_to_string(path).expect("Failed to read file");
    group.throughput(Throughput::Bytes(source.len() as u64));

    group.bench_function(name, |b| {
      b.iter(|| {
        let mut symbols = Interner::new();
        let mut lexer = Lexer::new(&source, FileId(0), &mut symbols);
        let tokens = lexer.tokenize();
        black_box(tokens);
      });
    });
  }

  group.finish();
}

criterion_group!(benches, bench_lex);
criterion_main!(benches);
