use psi::types::Element;
use ps::threaded;


use std::time::Instant;

fn time_protocol(X: Vec<Element>, Y: Vec<Element>) -> f64 {
  let start = Instant::now();

  let intersection = threaded::run_threaded(X, Y);

  let elapsed = start.elapsed();

  elapsed.as_secs_f64()
}
