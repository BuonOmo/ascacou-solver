use ascacou::{Board, Move};

pub struct Solver {
	explored_positions: u128,
	transposition_table: std::collections::HashMap<u128, EvaluationScore>,
}

pub use std::primitive::i16 as EvaluationScore;

const MIN_SCORE: EvaluationScore = -100;
const MAX_SCORE: EvaluationScore = 100;

macro_rules! heuristic_moves {
	( $first_color:ident => $last_color:ident [ $( ($x:expr, $y:expr) )* ] ) => {
		[
			$(
				Move::$first_color($x, $y),
			)*
			$(
				Move::$last_color($x, $y),
			)*
		]
	};
}

const HEURISTIC_BLACK_FIRST: [Move; 50] = heuristic_moves!(black => white [
	// First, center
	(2, 2) (2, 1) (1, 2) (2, 3) (3, 2)
	(1, 1) (1, 3) (3, 1) (3, 3)
	// Then edges
	(0, 2) (4, 2) (2, 0) (2, 4)
	(0, 1) (4, 1) (1, 0) (1, 4)
	(0, 3) (4, 3) (3, 0) (3, 4)
	// Then corners
	(0, 0) (0, 4) (4, 0) (4, 4)
]);

const HEURISTIC_WHITE_FIRST: [Move; 50] = heuristic_moves!(white => black [
	// First, center
	(2, 2) (2, 1) (1, 2) (2, 3) (3, 2)
	(1, 1) (1, 3) (3, 1) (3, 3)
	// Then edges
	(0, 2) (4, 2) (2, 0) (2, 4)
	(0, 1) (4, 1) (1, 0) (1, 4)
	(0, 3) (4, 3) (3, 0) (3, 4)
	// Then corners
	(0, 0) (0, 4) (4, 0) (4, 4)
]);

impl Solver {
	fn new() -> Solver {
		Solver {
			explored_positions: 0,
			transposition_table: std::collections::HashMap::new(),
		}
	}

	fn negamax0(
		&mut self,
		board: &Board,
		mut alpha: EvaluationScore,
		beta: EvaluationScore,
		depth: u8,
	) -> (EvaluationScore, Option<&Move>) {
		self.explored_positions += 1;

		if depth == 0 {
			return (evaluation(board), None);
		}

		let moves = possible_moves(&board);

		let mut best_mov: Option<&Move> = None;
		let mut terminal = true;
		for mov in moves {
			terminal = false;
			let score = -self.negamax(&board.next(&mov), -beta, -alpha, depth - 1);
			if score >= beta {
				return (score, Some(&mov));
			}

			if score > alpha {
				alpha = score;
				best_mov = Some(&mov);
			}
		}
		if terminal {
			alpha = evaluation(board);
		}

		return (alpha, best_mov);
	}

	fn negamax(
		&mut self,
		board: &Board,
		mut alpha: EvaluationScore,
		mut beta: EvaluationScore,
		depth: u8,
	) -> EvaluationScore {
		debug_assert!(alpha < beta);
		self.explored_positions += 1;

		let key = key(&board);

		// Reduce window by finding a transposition with a lower beta.
		if let Some(cached_beta) = self.transposition_table.get(&key) {
			if beta > *cached_beta {
				beta = *cached_beta;
				if alpha >= beta {
					return beta;
				}
			}
		}

		if depth == 0 {
			return evaluation(board);
		}

		let moves = possible_moves(&board);

		let mut terminal = true;

		for mov in moves {
			terminal = false;
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
			let score = -self.negamax(&board.next(&mov), -beta, -alpha, depth - 1);

			if score >= beta {
				return score;
			}

			if score > alpha {
				alpha = score;
			}
		}

		if terminal {
			alpha = evaluation(&board);
		}

		self.transposition_table.insert(key, alpha);
		return alpha;
	}
}

fn possible_moves<'a>(board: &Board) -> impl Iterator<Item = &'a Move> {
	let black_fav = board.current_player.favorite_color == ascacou::Color::Black;
	let preferred_heuristic = if black_fav {
		&HEURISTIC_BLACK_FIRST
	} else {
		&HEURISTIC_WHITE_FIRST
	};
	preferred_heuristic
		.iter()
		.filter(|mov| board.is_move_possible(mov))
}

// TODO(perf): Design a u64 key, and try partial key matching.
// See https://www.chessprogramming.org/Transposition_Table
fn key(board: &Board) -> u128 {
	(board.pieces_mask as u128) | ((board.black_mask as u128) << 64)
}

// TODO: a smarter score computation could be done by taking into
// account each player's score, and give a greater edge to a position
// close to terminal. More interesting even is the idea of taking into
// account partially filled tiles, e.g. forcing moves where only
// one color can be played to fill a tile.
//
// A _close to terminal_ position would be a position with few
// available moves.
fn evaluation(board: &Board) -> EvaluationScore {
	board.current_score() as EvaluationScore
}

pub fn solve(board: &Board, depth: Option<u8>) -> (EvaluationScore, Option<Move>, u128) {
	let mut solver = Solver::new();

	let move_count: usize = board.possible_moves().count();
	let max_depth = (move_count + 1) / 2;
	let depth = depth.unwrap_or(max_depth as u8).min(max_depth as u8);

	let (score, mov) = solver.negamax0(board, MIN_SCORE, MAX_SCORE, depth);

	(score, mov.cloned(), solver.explored_positions)
}

pub fn partial_solve(board: &Board, depth: Option<u8>) -> (EvaluationScore, Option<Move>, u128) {
	let mut solver = Solver::new();

	let move_count: usize = board.possible_moves().count();
	let max_depth = (move_count + 1) / 2;
	let depth = depth.unwrap_or(max_depth as u8).min(max_depth as u8);

	let (score, mov) = solver.negamax0(board, -1, 1, depth);

	(score, mov.cloned(), solver.explored_positions)
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::assert_matches::assert_matches;

	#[test]
	fn it_finds_winning_continuations() {
		let board = Board::from_fen("2bbw/bww1w/w1w1w/1w1bw/wbb1b 013679ce").unwrap();
		println!("{}", board.for_console());
		println!("{:?}", solve(&board, Some(8)));
		assert!(matches!(
			solve(&board, Some(8)),
			(x, Some(_), _) if x > 0
		));
		let board = Board::from_fen("1wbw/2b/1bb/5/5 01234567").unwrap();
		println!("{}", board.for_console());
		assert_eq!(solve(&board, Some(1)), (1, Some(Move::white(3, 1)), 39))
	}

	#[test]
	fn it_solves_endings_quickly() {
		let board = Board::from_fen("wwwbb/bwbwb/bbbww/bbwww/w 01234567").unwrap();
		println!("{}", board.for_console());
		let expected_move = Move::white(3, 4);
		let solved = solve(&board, Some(100));
		assert_matches!(solved, (3, Some(mov), _) if mov == expected_move,
			"expected {}, got {}",
			expected_move,
			solved.1.as_ref().unwrap(),
		)
	}

	#[test]
	#[ignore = "too slow, shall be used as a benchmark."]
	fn depths() {
		for i in 1..25 {
			// let board = Board::from_fen("5/5/5/5/5").unwrap();
			let board = Board::from_fen("1wbw/2b/1bb/5/5 01234567").unwrap();
			let now = std::time::Instant::now();
			let (.., explored_positions) = solve(&board, Some(i));
			let duration = now.elapsed().as_secs_f32();
			let message = format!(
				"Depth {} took {:.3} seconds to explore {} positions. ({:.2}M positions/sec)",
				i,
				duration,
				explored_positions,
				(explored_positions as f32) / (duration * 1_000_000.0)
			);
			assert!(duration < 10.0, "{}", message);
			println!("{}", message);
		}
	}

	#[test]
	#[ignore = "too slow, shall be used as a benchmark."]
	fn depths_partial() {
		for i in 1..25 {
			// let board = Board::from_fen("5/5/5/5/5").unwrap();
			let board = Board::from_fen("1wbw/2b/1bb/5/5 01234567").unwrap();
			let now = std::time::Instant::now();
			let (.., explored_positions) = partial_solve(&board, Some(i));
			let duration = now.elapsed().as_secs_f32();
			let message = format!(
				"Depth {} took {:.3} seconds to explore {} positions. ({:.2}M positions/sec)",
				i,
				duration,
				explored_positions,
				(explored_positions as f32) / (duration * 1_000_000.0)
			);
			assert!(duration < 10.0, "{}", message);
			println!("{}", message);
		}
	}
}
