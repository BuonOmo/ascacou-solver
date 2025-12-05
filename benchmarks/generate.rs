use ascacou::Board;
use rand::prelude::*;
use std::io::Write;

pub(crate) mod util;

use crate::util::{FILES, MAX_GROUP_SIZE, MOVES_LEFT};

fn generate_file<P>(filename: P, lines: usize, moves_left: usize) -> Result<(), &'static str>
where
	P: AsRef<std::path::Path>,
{
	let mut buffer = std::fs::File::create(filename).map_err(|_| "Could not create file")?;
	for _ in 0..lines {
		generate_line(&mut buffer, moves_left)?;
	}
	Ok(())
}

fn generate_line(buffer: &mut std::fs::File, moves_left: usize) -> Result<(), &'static str> {
	let mut board = Board::random_empty();
	let mut possible_moves = board.possible_moves();
	let mut rng = rand::rng();
	while possible_moves.len() > moves_left {
		let mov = possible_moves
			.choose(&mut rng)
			.ok_or("There should be at least one possible move")?;
		board = board.next(&mov);
		if board.is_invalid() {
			return Err("Generated an invalid board");
		}
		possible_moves = board.possible_moves();
	}

	buffer
		.write(format!("{}\n", board.fen()).as_bytes())
		.map_err(|_| "Could not write to file")
		.map(|_| ())
}

fn main() -> Result<(), &'static str> {
	let dir = std::path::Path::new("benchmarks/data");
	if !dir.exists() {
		std::fs::create_dir(dir).map_err(|_| "Could not create benchmarks/data directory")?;
	}
	std::env::set_current_dir(dir).map_err(|_| "Could not change directory")?;
	for (&file, &moves_left) in std::iter::zip(FILES, MOVES_LEFT) {
		generate_file(file, MAX_GROUP_SIZE, moves_left)?;
	}
	Ok(())
}
