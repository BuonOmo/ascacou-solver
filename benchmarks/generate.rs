use ascacou::{Board, Move};
use rand::prelude::*;
use std::io::Write;

pub(crate) mod utils;

use crate::utils::{FILES, MAX_GROUP_SIZE, MOVES_LEFT};

fn generate_file<P, R: Rng>(
	filename: P,
	lines: usize,
	moves_left: usize,
	rng: &mut R,
) -> Result<(), &'static str>
where
	P: AsRef<std::path::Path>,
{
	let mut buffer = std::fs::File::create(filename).map_err(|_| "Could not create file")?;
	for _ in 0..lines {
		generate_line(&mut buffer, moves_left, rng)?;
	}
	Ok(())
}

fn generate_line<R: Rng>(
	buffer: &mut std::fs::File,
	moves_left: usize,
	rng: &mut R,
) -> Result<(), &'static str> {
	let mut board = Board::random_empty(rng);
	let mut possible_moves: Vec<Move> = board.possible_moves().collect();
	while possible_moves.len() > moves_left {
		let mov = possible_moves
			.choose(rng)
			.ok_or("There should be at least one possible move")?;
		board = board.next(&mov).expect("should generate a valid board");
		possible_moves = board.possible_moves().collect();
	}

	buffer
		.write(format!("{}\n", board.fen()).as_bytes())
		.map_err(|_| "Could not write to file")
		.map(|_| ())
}

fn main() -> Result<(), &'static str> {
	let mut rng = SmallRng::seed_from_u64(42);
	let dir = std::path::Path::new("benchmarks/data");
	if !dir.exists() {
		std::fs::create_dir(dir).map_err(|_| "Could not create benchmarks/data directory")?;
	}
	std::env::set_current_dir(dir).map_err(|_| "Could not change directory")?;
	for (&file, &moves_left) in std::iter::zip(FILES, MOVES_LEFT) {
		generate_file(file, MAX_GROUP_SIZE, moves_left, &mut rng)?;
	}
	Ok(())
}
