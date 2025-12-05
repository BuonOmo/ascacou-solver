use ascacou::Board;
use clap::Parser;
use std::time::Instant;

#[derive(Parser)]
#[clap(about, author, version)]
struct Args {
	// Board in FEN format, default to empty board
	#[clap(parse(try_from_str = Board::from_fen), default_value_t = Board::empty())]
	board: Board,

	#[clap(short, long, default_value_t = 5)]
	depth: u8,
}

fn main() {
	let args = Args::parse();
	let t0 = Instant::now();
	if let (score, Some(mov), explored_positions) = minicou::solve(&args.board, Some(args.depth)) {
		println!("Move: {}", mov);
		println!("Time: {:.2?}", Instant::now() - t0);
		println!("Score: {}", score);
		println!("Explored positions: {}", explored_positions);
	}
}
