use ascacou::{Board, BoardKey};
use crate::heuristic;
use ascacou::Move;

pub struct Solver {
	explored_positions: u128,
	transposition_table: std::collections::HashMap<BoardKey, i8>
}

const MIN_SCORE: i8 = -100;
const MAX_SCORE: i8 = 100;

impl Solver {
	fn new() -> Solver {
		Solver { explored_positions: 0, transposition_table: std::collections::HashMap::new() }
	}
	pub fn solve(board: &Board, depth: Option<u8>) -> (i8, Option<Move>, u128) {
		let mut solver = Solver::new();

		let (score, mov) = solver.negamax0(board, MIN_SCORE, MAX_SCORE, depth.unwrap_or(5));
		// let (score, mov) = solver.n0(board, depth.unwrap_or(5));

		(score, mov, solver.explored_positions)
	}

	// pub fn move_scores(board: &Board, depth: Option<u8>) -> Vec<(&Move, i8)> {
	// 	let mut solver = Solver::new();

	// 	let mut move_scores = Vec::with_capacity(50);

	// 	for mov in board.possible_moves() {
	// 		move_scores.push(
	// 			(mov, -solver.negamax(board.next(mov), MIN_SCORE, MAX_SCORE, depth.unwrap_or(5)))
	// 		);
	// 	}

	// 	move_scores.sort_by_key(|(_mov, score)| -score);
	// 	move_scores
	// }

	fn n0(&mut self, board: &Board, depth: u8) -> (i8, Option<Move>) {
		self.explored_positions += 1;

		if depth == 0 {
			panic!("that makes no sense");
		}

		let possible_moves = board.possible_moves();

		if possible_moves.is_empty() {
			return (board.current_score(), None);
		}

		let mut best_mov: Option<Move> = None;
		let mut best_score = MIN_SCORE;

		for mov in possible_moves {
			let score = -self.negamax(board.next(mov), MIN_SCORE, MAX_SCORE, depth - 1);
			if score > best_score {
				best_mov = Some(mov);
				best_score = score;
			}
		}

		return (best_score, best_mov)
	}

	fn negamax0(&mut self, board: &Board, mut alpha: i8, beta: i8, depth: u8) -> (i8, Option<Move>) {
		self.explored_positions += 1;

		// let current_score = board.current_score();

		if depth == 0 {
			return (board.current_score(), None)
		}

		let possible_moves = board.possible_moves();

		if possible_moves.is_empty() {
			return (board.current_score(), None);
		}

		let mut best_mov: Option<Move> = None;

		let moves = heuristic::sorted_moves(
			possible_moves,
			board.current_player.favorite_color
		);

		for mov in moves {
			let score = -self.negamax(board.next(mov), -beta, -alpha, depth - 1);
			// println!("{:?} - {}", mov, score);
			if score >= beta {
				return (score, Some(mov))
			}

			if score > alpha {
				alpha = score;
				best_mov = Some(mov);
			}
		}

		return (alpha, best_mov);
	}

	fn negamax(&mut self, board: Board, mut alpha: i8, mut beta: i8, depth: u8) -> i8 {
		self.explored_positions += 1;

		let key = board.key();
		// Reduce window by finding a transposition with a lower beta.
		if let Some(cached_beta) = self.transposition_table.get(&key) {
			if beta > *cached_beta {
				beta = *cached_beta;
			}
		}

		if depth == 0 {
			return board.current_score()
		}

		let possible_moves = board.possible_moves();

		if possible_moves.is_empty() { /* terminal position */
			return board.current_score();
		}

		let moves = heuristic::sorted_moves(
			possible_moves,
			board.current_player.favorite_color
		);

		for mov in moves {
			// TODO(perf): we could have the board being part of the solver as mutable, and
			//  have a function to make a move and unmake a move. This way we would not
			//  instanciate any new board, may be much more performant.
			//
			//  eg:
			//  self.board.make_move(move)
			//  let score = ...
			//  self.board.rewind_move(move)
			//
			//  a simple implementation of this idea only yields a quite small improvement (from 1.9ms to 1.7ms for a
		    //  full random game simulation)
			let score = -self.negamax(board.next(mov), -beta, -alpha, depth - 1);

			if score >= beta {
				return score
			}

			if score > alpha {
				alpha = score;
			}
		}

		self.transposition_table.insert(key, alpha);
		return alpha;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_finds_winning_continuations() {
		let board = Board::from_fen("2bbw/bww1w/w1w1w/1w1bw/wbb1b 2458abdf").unwrap();
		println!("{}", board.for_console());
		println!("{:?}", Solver::solve(&board, Some(8)));
		assert!(
			matches!(
				Solver::solve(&board, Some(8)),
				(x, Some(_), _) if x > 0
			)
		);
		let board = Board::from_fen("1wbw/2b/1bb/5/5 01234567").unwrap();
		println!("{}", board.for_console());
		assert_eq!(
			Solver::solve(&board, Some(1)),
			(1, Some(Move::white(3, 1)), 39)
		)
	}

	#[test]
	fn it_solves_endings_quickly() {
		let board = Board::from_fen("wwwbb/bwbwb/bbbww/bbwww/w 01234567").unwrap();
		println!("{}", board.for_console());
		assert_eq!(
			Solver::solve(&board, Some(100)),
			(1, Some(Move::white(1, 3)), 39)
		)
	}

	#[test]
	#[ignore = "too slow, shall be used as a benchmark."]
	fn depths() {
		for i in 1..25 {
			// let board = Board::from_fen("5/5/5/5/5").unwrap();
			let board = Board::from_fen("1wbw/2b/1bb/5/5 01234567").unwrap();
			let now = std::time::Instant::now();
			let (.., explored_positions) = Solver::solve(&board, Some(i));
			let duration = now.elapsed().as_secs_f32();
			let message = format!(
				"Depth {} took {} seconds to explore {} positions.",
				i, duration, explored_positions
			);
			assert!(
				duration < 10.0,
				"{}", message
			);
			println!("{}", message);
		}

	}
}
