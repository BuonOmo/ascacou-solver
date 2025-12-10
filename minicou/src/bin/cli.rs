use ascacou::Board;
use clap::Parser;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

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

fn run_solver(board: &Board, depth: u8) -> Option<(i16, ascacou::Move, u128)> {
	match minicou::solve(&board, Some(depth)) {
		(score, Some(mov), explored_positions) => Some((score, mov, explored_positions)),
		_ => None,
	}
}

fn main() {
	let args = Args::parse();
	let t0 = Instant::now();
	let (tx, rx) = mpsc::channel();
	let max_depth = args.depth;
	let board = args.board;
	thread::spawn(move || {
		let mut depth = 4;
		while depth < max_depth {
			tx.send(run_solver(&board, depth).zip(Some(depth))).unwrap();
			depth += 1;
		}
	});

	let mut best_result = None;
	let timeout = std::time::Duration::from_millis(args.timeout);
	let mut time_left = timeout.saturating_sub(Instant::now() - t0);
	while !time_left.is_zero() {
		if let Ok(result) = rx.recv_timeout(time_left) {
			best_result = result;
		}
		time_left = timeout.saturating_sub(Instant::now() - t0);
	}
	let ((score, mov, explored_positions), depth) = best_result.expect(&format!(
		"Could not find a solution.\nCalled with:\n{:#?}",
		args
	));

	println!("Move: {}", mov);
	println!("Time: {:.2?}", Instant::now() - t0);
	println!("Score: {}", score);
	println!("Depth: {}/{}", depth, max_depth);
	println!("Explored positions: {}", explored_positions);
}
