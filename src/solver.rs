use crate::board::Board;
use crate::mov::Move;

struct Solver {
	explored_positions: u128
}

impl Solver {
	pub fn solve(board: Board, depth: Option<u8>) -> (i8, Option<Move>, u128) {
		let mut solver = Solver { explored_positions: 0 };

		let (score, mov) = solver.negamax0(board, depth.unwrap_or(5));

		(score, mov, solver.explored_positions)
	}

	// TODO: alpha beta pruning at least, maybe tr tables.
	fn negamax0(&mut self, board: Board, depth: u8) -> (i8, Option<Move>) {
		self.explored_positions += 1;

		if depth == 0 {
			return (board.current_score(), None)
		}

		let mut max_score = i8::MIN;
		let mut best_mov: Option<Move> = None;

		for mov in board.possible_moves() {
			let score = -self.negamax(board.next(mov), depth - 1);
			// println!("{:?} - {}", mov, score);
			if score > max_score {
				max_score = score;
				best_mov = Some(mov);
			}
		}

		return (max_score, best_mov);
	}

	fn negamax(&mut self, board: Board, depth: u8) -> i8 {
		self.explored_positions += 1;

		if depth == 0 {
			return board.current_score()
		}

		let mut max_score = i8::MIN;

		for mov in board.possible_moves() {
			let score = -self.negamax(board.next(mov), depth - 1);
			if score > max_score {
				max_score = score;
			}
		}

		return max_score;
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
			let board = Board::from_fen("5/5/5/5/5").unwrap();
			let now = std::time::Instant::now();
			let (.., explored_positions) = Solver::solve(board, Some(i));
			let duration = now.elapsed().as_secs_f32();
			let message = format!(
				"Depth {} took {} seconds to explore {} positions.",
				i, duration, explored_positions
			);
			assert!(
				duration < 5.0,
				"{}", message
			);
			println!("{}", message);
		}

	}
}
