use ascacou::{Board, Color::*, Move};

pub struct Solver {
	explored_positions: u128,
	transposition_table: std::collections::HashMap<u128, EvaluationScore>,
}

pub use std::primitive::i16 as EvaluationScore;

const MIN_SCORE: EvaluationScore = -100;
const MAX_SCORE: EvaluationScore = 100;

/// Depth of forced moves search. These moves will
/// be explored when depth is exhausted to make sure
/// we compute an evaluation score as close to the
/// endgame as possible.
///
/// The value 3 was chosen empirically using the
/// `tourney` tests on a small set of boards. Here
/// is the results:
///
/// | forced depth | mid | early | start | total |
/// | -----------: | --: | ----: | ----: | ----: |
/// |            0 |   0 |    -1 |     0 |    -1 |
/// |            1 |  -2 |     1 |     2 |     1 |
/// |            2 |  -1 |     0 |    -1 |    -2 |
/// |            3 |  -1 |     3 |     2 |     4 |
/// |            4 |  -2 |     1 |     1 |     0 |
/// |            5 |  -2 |    -1 |     0 |    -3 |
/// |            6 |  -1 |     0 |     3 |     2 |
/// |            7 |  -1 |    -3 |     4 |     0 |
const FORCED_MOVE_DEPTH: u8 = 6;

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

	fn negamax0<'a>(
		&mut self,
		board: &'a Board,
		mut alpha: EvaluationScore,
		beta: EvaluationScore,
		depth: u8,
	) -> (EvaluationScore, Option<Move>) {
		self.explored_positions += 1;

		if depth == 0 {
			return (evaluation(board), None);
		}

		let boards_and_moves = next_boards::<(Board, Move)>(&board, false);

		let mut best_mov: Option<Move> = None;
		let mut terminal = true;
		for (board, mov) in boards_and_moves {
			terminal = false;
			let score = -self.negamax(&board, -beta, -alpha, depth - 1);
			if score >= beta {
				return (score, Some(mov));
			}

			if score > alpha {
				alpha = score;
				best_mov = Some(mov);
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

		let boards = next_boards::<Board>(&board, depth <= FORCED_MOVE_DEPTH);

		let mut terminal = true;

		for board in boards {
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
			let score = -self.negamax(&board, -beta, -alpha, depth - 1);

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

struct MaskIterator(u64);

impl Iterator for MaskIterator {
	type Item = u64;

	fn next(&mut self) -> Option<Self::Item> {
		if self.0 == 0 {
			return None;
		}
		let prev = self.0;
		self.0 &= self.0 - 1;
		let top_left = prev - self.0;
		Some(top_left)
	}
}

fn next_boards<'a, T>(board: &'a Board, forced: bool) -> MoveIterator<'a, T> {
	if forced {
		let x = board.pieces_mask;
		MoveIterator::Forced(ForcedMoveIterator {
			almost_full_mask: MaskIterator(
				(!x & (x >> 1) & (x >> 7) & (x >> 8))
					| (!x & (x << 1) & (x >> 6) & (x >> 7))
					| (!x & (x >> 1) & (x << 6) & (x << 7))
					| (!x & (x << 1) & (x << 7) & (x << 8)),
			),
			board,
			return_type: std::marker::PhantomData,
		})
	} else {
		let black_fav = board.current_player.favorite_color == ascacou::Color::Black;
		let heuristic = if black_fav {
			&HEURISTIC_BLACK_FIRST
		} else {
			&HEURISTIC_WHITE_FIRST
		};
		MoveIterator::All(AllMoveIterator {
			board,
			index: 0,
			len: heuristic.len(),
			heuristic,
			return_type: std::marker::PhantomData,
		})
	}
}

// NOTE: When MoveIterator was bugged
// and forced moves where actually
// similar to AllMoves, we ended up
// with better performance.
enum MoveIterator<'a, T> {
	Forced(ForcedMoveIterator<'a, T>),
	All(AllMoveIterator<'a, T>),
}

impl<'a> Iterator for MoveIterator<'a, Board> {
	type Item = Board;
	fn next(&mut self) -> Option<Board> {
		match self {
			MoveIterator::Forced(it) => it.next(),
			MoveIterator::All(it) => it.next(),
		}
	}
}

impl<'a> Iterator for MoveIterator<'a, (Board, Move)> {
	type Item = (Board, Move);
	fn next(&mut self) -> Option<(Board, Move)> {
		match self {
			MoveIterator::Forced(it) => it.next(),
			MoveIterator::All(it) => it.next(),
		}
	}
}

struct ForcedMoveIterator<'a, T> {
	almost_full_mask: MaskIterator,
	board: &'a Board,
	return_type: std::marker::PhantomData<T>,
}

impl<'a> Iterator for ForcedMoveIterator<'a, Board> {
	type Item = Board;

	fn next(&mut self) -> Option<Self::Item> {
		match self.almost_full_mask.next() {
			None => None,
			Some(mask) => {
				let mov_black = Move::from_mask(mask, Black);
				let mov_white = Move::from_mask(mask, White);
				match (self.board.next(&mov_black), self.board.next(&mov_white)) {
					(Some(b), None) => Some(b),
					(None, Some(w)) => Some(w),
					_ => self.next(),
				}
			}
		}
	}
}

impl<'a> Iterator for ForcedMoveIterator<'a, (Board, Move)> {
	type Item = (Board, Move);

	fn next(&mut self) -> Option<Self::Item> {
		match self.almost_full_mask.next() {
			None => None,
			Some(mask) => {
				let mov_black = Move::from_mask(mask, Black);
				let mov_white = Move::from_mask(mask, White);
				match (self.board.next(&mov_black), self.board.next(&mov_white)) {
					(Some(b), None) => Some((b, mov_black)),
					(None, Some(w)) => Some((w, mov_white)),
					_ => self.next(),
				}
			}
		}
	}
}

struct AllMoveIterator<'a, T> {
	board: &'a Board,
	index: usize,
	len: usize,
	heuristic: &'a [Move; 50],
	return_type: std::marker::PhantomData<T>,
}

impl<'a> Iterator for AllMoveIterator<'a, Board> {
	type Item = Board;

	fn next(&mut self) -> Option<Self::Item> {
		while self.index < self.len {
			let mov = &self.heuristic[self.index];
			self.index += 1;
			if let Some(b) = self.board.next(mov) {
				return Some(b);
			}
		}
		None
	}
}

impl<'a> Iterator for AllMoveIterator<'a, (Board, Move)> {
	type Item = (Board, Move);

	fn next(&mut self) -> Option<Self::Item> {
		while self.index < self.len {
			let mov = &self.heuristic[self.index];
			self.index += 1;
			if let Some(b) = self.board.next(mov) {
				return Some((b, *mov));
			}
		}
		None
	}
}

// TODO(perf): Design a u64 key, and try partial key matching.
// See https://www.chessprogramming.org/Transposition_Table
fn key(board: &Board) -> u128 {
	(board.pieces_mask as u128) | ((board.black_mask as u128) << 64)
}

// TODO: a smarter score computation could be done by taking into
// account each player's score, and give a greater edge to a position
// close to terminal. More interesting even is the idea of taking into
// account partially filled tiles, e.g. forced moves where only
// one color can be played to fill a tile.
//
// A _close to terminal_ position would be a position with few
// available moves.
fn evaluation(board: &Board) -> EvaluationScore {
	board.current_score() as EvaluationScore
}

pub fn solve(board: &Board, depth: Option<u8>) -> (EvaluationScore, Option<Move>, u128) {
	let mut solver = Solver::new();

	let move_count = board.possible_moves().count() as u8;
	// Adding FORCED_MOVE_DEPTH to the max depth to ensure we
	// explore non-forcing moves up to the maximum if we can
	// and only rely on forced moves if we cannot explore
	// to full depth. Otherwise, we may end up not exploring
	// some non-forced last moves.
	let max_depth = (move_count + 1) / 2 + FORCED_MOVE_DEPTH;
	let depth = depth.unwrap_or(max_depth as u8).min(max_depth as u8);

	let (score, mov) = solver.negamax0(board, MIN_SCORE, MAX_SCORE, depth);

	(score, mov, solver.explored_positions)
}

pub fn partial_solve(board: &Board, depth: Option<u8>) -> (EvaluationScore, Option<Move>, u128) {
	let mut solver = Solver::new();

	let move_count = board.possible_moves().count() as u8;
	// Adding FORCED_MOVE_DEPTH to the max depth to ensure we
	// explore non-forcing moves up to the maximum if we can
	// and only rely on forced moves if we cannot explore
	// to full depth. Otherwise, we may end up not exploring
	// some non-forced last moves.
	let max_depth = (move_count + 1) / 2 + FORCED_MOVE_DEPTH;
	let depth = depth.unwrap_or(max_depth as u8).min(max_depth as u8);

	let (score, mov) = solver.negamax0(board, -1, 1, depth);

	(score, mov, solver.explored_positions)
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::assert_matches::assert_matches;

	#[test]
	fn test_forced_moves() {
		for (fen, expected) in [
			("bwwb/2www/3wb/bbw/1w1wb 025689ad", vec![]),
			(
				"bb1ww/www1w/1bb/1bww/2w 013457df",
				vec!["wc1", "wa3", "wd3"],
			),
			("wwbw/bb2b/1wwb/2b/wb1wb 124589ef", vec!["wb4"]),
			("w3w/wbbbb/2b1b/2bw/bwb1w 013568be", vec!["wd3"]),
			("bww/1bb1w/wb/1wwbw/1www 0149bcde", vec!["ba4"]),
			("wbbbb/b2ww/bbw/b1b/1b1b 13678adf", vec!["bc2"]),
			(
				// This is a case where there are ony two non-forced moves left.
				"bbwwb/bbwww/b3b/bwwww/2w1b 01389cdf",
				vec!["bd3", "bb5", "bd5"],
			),
			("bww/1w1ww/2wwb/1wbb/1b1ww 023679ab", vec!["bb3"]),
			("bw2b/ww2b/bww1w/w1b/w1w1b 12346cdf", vec!["bb4"]),
		] {
			let board = Board::from_fen(fen).unwrap();
			let forced: Vec<String> = next_boards::<(Board, Move)>(&board, true)
				.map(|(_, m)| String::from(m))
				.collect();
			assert_eq!(forced, expected, "for board:\n{}", board.for_console());
		}
	}
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
		// Once This passes, we can consider that Ascacou is
		// strongly solved.
		for i in 1..(25 + FORCED_MOVE_DEPTH) {
			let board = Board::empty();
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
		for i in 1..(25 + FORCED_MOVE_DEPTH) {
			let board = Board::empty();
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
