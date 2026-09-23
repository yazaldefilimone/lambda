use std::fs;
use walkdir::WalkDir;

use lambdac::{cli::options::Options, context::Context, driver::Driver, printer::Printer};

#[test]
fn golden_eval_tests() {
  let mut entries: Vec<_> = WalkDir::new("tests/golden")
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.path().extension().is_some_and(|ext| ext == "lam"))
    .collect();

  entries.sort_by_key(|e| e.path().to_path_buf());

  assert!(!entries.is_empty(), "No golden test cases found in tests/golden");

  for entry in entries {
    let path = entry.path();
    let rel_path = path.strip_prefix("tests/golden").unwrap();
    let test_name = rel_path
      .to_string_lossy()
      .replace(['/', '\\'], "__")
      .trim_end_matches(".lam")
      .to_string();

    let source = fs::read_to_string(path).expect("Failed to read test file");

    let options = Options::default();
    let mut ctx = Context::new(options);
    let mut driver = Driver::new(&mut ctx);

    let term = driver
      .compile_source(&test_name, &source)
      .expect("Compilation failed");

    let (evaluated, stats) = driver.evaluate_term(term).expect("Evaluation failed");

    let term_stats = lambdac::stats::TermStats::compute(term, &ctx);

    let evaluated_str = {
      let mut decoder = lambdac::decode::Decoder::new(&mut ctx);
      match decoder.church(evaluated) {
        Ok(value_id) => decoder.print(value_id),
        Err(_) => {
          let printer = Printer::new(&ctx);
          printer.print_term(evaluated)
        },
      }
    };

    let snapshot_content = format!(
      "{}\n\nterm\n  nodes: {}\n  depth: {}\n\neval\n  steps:    {}\n  reduces:  {}\n  allocs:   {}\n  time:     [time]\n",
      evaluated_str.trim(),
      term_stats.nodes,
      term_stats.depth,
      stats.steps,
      stats.reductions,
      stats.created_terms,
    );

    insta::with_settings!({
      snapshot_path => "snapshots",
      prepend_module_to_snapshot => false,
      input_file => path,
    }, {
      insta::assert_snapshot!(test_name, snapshot_content);
    });
  }
}
