#![feature(assert_matches)]
#![feature(gen_blocks)]

mod evaluation;
mod solver;

use evaluation::evaluate;
pub use solver::{Solver, partial_solve, solve};
