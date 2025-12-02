use ascacou::Board;
use minicou::Solver;

mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn solve(fen: &str, depth: u8) -> Option<String> {
	Board::from_fen(fen).ok().and_then(|board| {
		let (_, mov_opt, _) = Solver::solve(&board, Some(depth));
		mov_opt.and_then(|mov| Some(mov.into()))
	})
}
