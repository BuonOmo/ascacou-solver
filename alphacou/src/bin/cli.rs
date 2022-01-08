use ascacou::Board;
use alphacou::Solver;
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

	/// Print a sequence of moves rather than a single move.
	#[clap(short, long)]
	sequence: bool,
}

fn main() {
	let args = Args::parse();
	if args.sequence {
		println!(
			"{}",
			Solver::best_continuation(args.board, Duration::from_secs(args.duration)).iter().map(|mov|mov.to_string()).collect::<Vec<String>>().join(", ")
		)
	} else {
		println!(
			"{}",
			Solver::best_move(
				args.board,
				Duration::from_secs(args.duration)
			).and_then(|mov|Some(mov.to_string())).unwrap_or("N.A.".to_string())
		);
	}
}
