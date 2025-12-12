#![feature(assert_matches)]
#![feature(gen_blocks)]
#![feature(test)]

mod solver;
pub(crate) mod transposition_table;

pub use solver::{Solver, partial_solve, solve};
