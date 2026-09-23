use std::hint::black_box;
use std::path::PathBuf;

use criterion::{Criterion, criterion_group, criterion_main};
use lambdac::{cli::options::Options, context::Context, driver::Driver};

fn bench_eval(c: &mut Criterion) {
  let mut group = c.benchmark_group("eval");

  let benchmarks = [
    ("pred", "examples/church/pred.lam"),
    ("skk", "examples/combinators/skk.lam"),
    ("arithmetic", "examples/church/arithmetic.lam"),
    ("booleans", "examples/church/booleans.lam"),
    ("factorial", "examples/recursion/factorial.lam"),
    ("deep", "examples/stress/deep.lam"),
  ];

  for (name, path) in benchmarks {
    let mut options = Options::default();
    options.input = PathBuf::from(path);
    let mut ctx = Context::new(options);
    let mut driver = Driver::new(&mut ctx);
    let term = driver.compile_file().expect("compilation failed");
    let initial_terms = driver.ctx.terms.clone();

    group.bench_function(name, |b| {
      b.iter(|| {
        driver.ctx.terms = initial_terms.clone();
        let (evaluated, _) = driver.evaluate_term(term).expect("eval failed");
        black_box(evaluated);
      });
    });
  }

  group.finish();
}

criterion_group!(benches, bench_eval);
criterion_main!(benches);
