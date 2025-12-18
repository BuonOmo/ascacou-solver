use ascacou::Board;
use clap::Parser;
use rand::prelude::IndexedRandom;

#[derive(Parser, Debug)]
#[clap(about, author, version)]
struct Args {
	/// Board in FEN format
	#[clap(value_parser = Board::from_fen, default_value_t = Board::empty())]
	board: Board,

	/// Maximum search depth (5-25)
	#[clap(short, long, default_value_t = 8, value_parser = clap::value_parser!(u8).range(5..=25))]
	depth: u8,

	/// Strict timeout in milliseconds
	#[clap(short, long, default_value_t = 500, value_parser = clap::value_parser!(u64).range(1..))]
	timeout: u64,
}

fn main() {
	let args = Args::parse();
	let moves = args.board.possible_moves().collect::<Vec<_>>();
	let mov = moves.choose(&mut rand::rng()).unwrap();
	println!("Move: {}", mov);
}
