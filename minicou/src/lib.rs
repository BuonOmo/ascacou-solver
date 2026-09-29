#![feature(assert_matches)]
#![feature(gen_blocks)]

mod solver;
mod transposition_table;

pub use solver::{
	Solver, partial_solve, partial_solve_with_time_limit, solve, solve_with_time_limit,
};
