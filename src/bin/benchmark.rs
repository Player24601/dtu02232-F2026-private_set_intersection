use psi::threaded::run_threaded;
use psi::types::Element;
use psi::verify::verify_intersection;

use std::time::Instant;

fn time_protocol(X: Vec<Element>, Y: Vec<Element>) -> f64 {
  let X_copy = X.clone();
  let Y_Copy = Y.clone();

  let start = Instant::now();

  let I = run_threaded(X, Y);

  let elapsed = start.elapsed();

  assert!(
    verify_intersection(X_copy, Y_Copy, I),
    "PSI protocol returned incorrect intersection"
  );

  elapsed.as_secs_f64()
}
