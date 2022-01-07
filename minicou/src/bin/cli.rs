use ascacou::Board;
use minicou::Solver;
use clap::Parser;
use std::time::Duration;

#[derive(Parser)]
#[clap(about, author, version)]
struct Args {
    /// Name of the person to greet
    #[clap(parse(try_from_str = Board::from_fen), default_value_t = Board::empty())]
    board: Board,

    /// Number of times for the IA to find a move (in s)
    #[clap(short, long, default_value_t = 1)]
    duration: u64,
}

fn main() {
	let args = Args::parse();
	if let (_, Some(mov), _) = Solver::solve(
		&args.board,
		Some(8)
	) {
		println!(
			"{}",
			mov
		);
	}
}
