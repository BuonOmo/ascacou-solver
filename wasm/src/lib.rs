use ascacou::{Board, Move};
use minicou::Solver;

mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn solve(fen: &str, depth: u8) -> Result<String, String> {
	let board = Board::from_fen(fen)?;

	let (_, mov_opt, _) = Solver::solve(&board, Some(depth));

	let Some(mov) = mov_opt else {
		return Err("No solution found".to_string());
	};
	Ok(mov.into())
}

#[wasm_bindgen]
pub fn play(fen: &str, #[wasm_bindgen(js_name = "move")] mov: &str) -> Result<String, String> {
	let board = Board::from_fen(fen)?;
	let mov = Move::try_from(mov.to_string())?;
	let next = board.next(mov);
	if next.is_invalid() {
		return Err("Invalid move".to_string());
	}

	Ok(next.fen())
}

#[wasm_bindgen]
pub fn moves(fen: &str) -> Result<Vec<String>, String> {
	let moves = Board::from_fen(fen)?.possible_moves();
	let mut rv = vec![];

	for mov in moves {
		let mov_str: String = mov.into();
		rv.push(mov_str);
	}

	Ok(rv)
}
