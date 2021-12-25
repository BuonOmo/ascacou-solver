use crate::board::Board;
use crate::heuristic;
use crate::mov::Move;

struct Solver {
	explored_positions: u128
}

const MIN_SCORE: i8 = -100;
const MAX_SCORE: i8 = 100;

impl Solver {
	pub fn solve(board: Board, depth: Option<u8>) -> (i8, Option<Move>, u128) {
		let mut solver = Solver { explored_positions: 0 };

		let (score, mov) = solver.negamax0(board, MIN_SCORE, MAX_SCORE, depth.unwrap_or(5));

		(score, mov, solver.explored_positions)
	}

	// TODO: transposition tables.
	fn negamax0(&mut self, board: Board, mut alpha: i8, mut beta: i8, depth: u8) -> (i8, Option<Move>) {
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

		// TODO: detect terminal position.

		if depth == 0 {
			return board.current_score()
		}

		let possible_moves = board.possible_moves();

		if possible_moves.is_empty() {
			return board.current_score();
		}

		let moves = heuristic::sorted_moves(
			possible_moves,
			board.current_player.favorite_color
		);

		for mov in moves {
			let score = -self.negamax(board.next(mov), -beta, -alpha, depth - 1);

			if score >= beta {
				return score
			}

			if score > alpha {
				alpha = score;
			}
		}

		return alpha;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_solve() {
		let board = Board::from_fen("1wbw/2b/1bb/5/5").unwrap();
		println!("{}", board);
		assert_eq!(
			Solver::solve(board, Some(1)),
			(1, Some(Move::white(1, 3)), 39)
		)
	}

	#[test]
	fn depths() {
		for i in 1..25 {
			// let board = Board::from_fen("5/5/5/5/5").unwrap();
			let board = Board::from_fen("1wbw/2b/1bb/5/5").unwrap();
			let now = std::time::Instant::now();
			let (.., explored_positions) = Solver::solve(board, Some(i));
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
