use std::time::{Duration, Instant};

use ascacou::{Board, Color::*, Move};

use crate::transposition_table::{self, Bound, TranspositionTable};

pub struct Solver {
	explored_positions: u128,
	transposition_table: TranspositionTable,
	/// When set, the search periodically checks the wall clock
	/// against this deadline and bails out early (see
	/// `check_time_up`) once it is passed.
	deadline: Option<Instant>,
	/// Sticky flag set once `check_time_up` observes we're past the
	/// deadline. Reading a `bool` is essentially free, so every
	/// recursive call can check it before doing any work, while the
	/// (comparatively expensive) `Instant::now()` call itself is
	/// only done every `TIME_CHECK_INTERVAL` explored positions.
	time_up: bool,
}

pub use std::primitive::i16 as EvaluationScore;

const MIN_SCORE: EvaluationScore = -100;
const MAX_SCORE: EvaluationScore = 100;

/// How often (in explored positions) we check the wall clock against
/// the deadline, when one is set. Checking on every node would make
/// `Instant::now()` calls a measurable chunk of the per-node cost;
/// checking too rarely would make the deadline imprecise.
const TIME_CHECK_INTERVAL: u128 = 4096;

/// Number of slots in the transposition table. Chosen as a prime
/// number so that, combined with the modulo indexing, positions
/// spread more evenly across the table and collisions between
/// unrelated keys are less likely to line up. This size targets
/// roughly 100MB of memory (each entry being a handful of bytes).
const TRANSPOSITION_TABLE_SIZE: usize = 8_388_593;

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
const FORCED_MOVE_DEPTH: u8 = 3;

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
	fn new(deadline: Option<Instant>) -> Solver {
		Solver {
			explored_positions: 0,
			transposition_table: TranspositionTable::new(TRANSPOSITION_TABLE_SIZE),
			deadline,
			time_up: false,
		}
	}

	/// Returns whether the search deadline (if any) has passed. Only
	/// touches the wall clock every `TIME_CHECK_INTERVAL` positions;
	/// relies on `self.explored_positions` having already been
	/// incremented by the caller for this node.
	fn check_time_up(&mut self) -> bool {
		if self.time_up {
			return true;
		}
		let Some(deadline) = self.deadline else {
			return false;
		};
		if self.explored_positions % TIME_CHECK_INTERVAL == 0 && Instant::now() >= deadline {
			self.time_up = true;
		}
		self.time_up
	}

	fn negamax0<'a>(
		&mut self,
		board: &'a Board,
		mut alpha: EvaluationScore,
		beta: EvaluationScore,
		depth: u8,
		hint: Option<Move>,
	) -> (EvaluationScore, Option<Move>) {
		self.explored_positions += 1;

		if self.check_time_up() {
			return (evaluation(board), None);
		}

		if depth == 0 {
			return (evaluation(board), None);
		}

		let mut best_mov: Option<Move> = None;
		let mut terminal = true;

		// Try the previous iteration's best move first: if it's
		// still the best, we get an immediate beta cutoff on
		// everything else at this node.
		if let Some(mov) = hint {
			if let Some(child) = board.next(&mov) {
				terminal = false;
				let score = -self.negamax(&child, -beta, -alpha, depth - 1);
				if score >= beta {
					return (score, Some(mov));
				}
				if score > alpha {
					alpha = score;
					best_mov = Some(mov);
				}
			}
		}

		let boards_and_moves = next_boards::<(Board, Move)>(&board, false);

		for (board, mov) in boards_and_moves {
			if Some(mov) == hint {
				continue;
			}
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
		alpha: EvaluationScore,
		beta: EvaluationScore,
		depth: u8,
	) -> EvaluationScore {
		debug_assert!(alpha < beta);
		self.explored_positions += 1;

		if self.check_time_up() {
			return evaluation(board);
		}

		let key = transposition_table::key(board);
		let original_alpha = alpha;
		let mut alpha = alpha;
		let mut beta = beta;
		let mut preferred_move: Option<Move> = None;

		// Try to use a previous search's result, either as a direct
		// cutoff (when it was computed at least as deep as we need
		// here), or to narrow our alpha-beta window. Even when the
		// stored depth isn't enough for a cutoff, its best move is
		// still a good move-ordering hint.
		if let Some(entry) = self.transposition_table.get(key) {
			preferred_move = entry.best_move;
			if entry.depth >= depth {
				match entry.bound {
					Bound::Exact => return entry.score,
					Bound::Lower => {
						if entry.score >= beta {
							return entry.score;
						}
						if entry.score > alpha {
							alpha = entry.score;
						}
					}
					Bound::Upper => {
						if entry.score <= alpha {
							return entry.score;
						}
						if entry.score < beta {
							beta = entry.score;
						}
					}
				}
				if alpha >= beta {
					return entry.score;
				}
			}
		}

		if depth == 0 {
			let score = evaluation(board);
			if !self.time_up {
				self.transposition_table
					.insert(key, score, depth, Bound::Exact, None);
			}
			return score;
		}

		let forced = depth <= FORCED_MOVE_DEPTH;
		// During the forced-move phase the candidate set is already
		// small and has a different meaning (a specific pattern, not
		// a heuristic ranking), so we don't reorder it.
		let preferred_move = if forced { None } else { preferred_move };

		let mut terminal = true;
		// Fail-soft: track the actual best score found, rather than
		// only the (possibly still un-raised) alpha bound. This
		// gives tighter, more reusable transposition table entries.
		let mut best_score = MIN_SCORE - 1;
		let mut best_move: Option<Move> = None;

		if let Some(mov) = preferred_move {
			if let Some(next_board) = board.next(&mov) {
				terminal = false;
				let score = -self.negamax(&next_board, -beta, -alpha, depth - 1);

				if score > best_score {
					best_score = score;
					best_move = Some(mov);
				}

				if score >= beta {
					if !self.time_up {
						self.transposition_table
							.insert(key, score, depth, Bound::Lower, Some(mov));
					}
					return score;
				}

				if score > alpha {
					alpha = score;
				}
			}
		}

		let boards_and_moves = next_boards::<(Board, Move)>(&board, forced);

		for (next_board, mov) in boards_and_moves {
			if Some(mov) == preferred_move {
				continue;
			}
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
			let score = -self.negamax(&next_board, -beta, -alpha, depth - 1);

			if score > best_score {
				best_score = score;
				best_move = Some(mov);
			}

			if score >= beta {
				if !self.time_up {
					self.transposition_table
						.insert(key, score, depth, Bound::Lower, Some(mov));
				}
				return score;
			}

			if score > alpha {
				alpha = score;
			}
		}

		if terminal {
			let score = evaluation(board);
			if !self.time_up {
				self.transposition_table
					.insert(key, score, depth, Bound::Exact, None);
			}
			return score;
		}

		let bound = if best_score <= original_alpha {
			Bound::Upper
		} else {
			Bound::Exact
		};
		if !self.time_up {
			self.transposition_table
				.insert(key, best_score, depth, bound, best_move);
		}

		return best_score;
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

// TTs are addressed with a 50 bit key built in `transposition_table::key`.

fn max_depth_for(board: &Board) -> u8 {
	let move_count = board.possible_moves().count() as u8;
	// Adding FORCED_MOVE_DEPTH to the max depth to ensure we
	// explore non-forcing moves up to the maximum if we can
	// and only rely on forced moves if we cannot explore
	// to full depth. Otherwise, we may end up not exploring
	// some non-forced last moves.
	(move_count + 1) / 2 + FORCED_MOVE_DEPTH
}

/// Iterative deepening driver used by the time-bounded solve
/// variants: searches depth 1, 2, ... up to `depth` (capped at the
/// position's max useful depth), reusing each completed iteration's
/// best move as a move-ordering hint for the next one, so that a
/// deadline expiring mid-iteration still leaves a usable answer from
/// the last fully completed depth.
///
/// When `time_limit` is exceeded while searching a given depth, that
/// (possibly incomplete) iteration is discarded and the result of the
/// last fully completed iteration is returned instead of blocking
/// until the full target depth is reached.
///
/// When there is no time limit, the target depth is already known
/// upfront, so we skip straight to it instead of redundantly
/// re-searching every shallower depth first: unlike a real-time
/// budget, here the shallower iterations would only add overhead
/// with no benefit, since within-search transposition table reuse
/// already reorders moves at every node.
fn solve_iterative(
	board: &Board,
	depth: Option<u8>,
	root_alpha: EvaluationScore,
	root_beta: EvaluationScore,
	time_limit: Option<Duration>,
) -> (EvaluationScore, Option<Move>, u128) {
	let max_depth = max_depth_for(board);
	let target_depth = depth.unwrap_or(max_depth).min(max_depth);

	let Some(time_limit) = time_limit else {
		let mut solver = Solver::new(None);
		let (score, mov) = solver.negamax0(board, root_alpha, root_beta, target_depth, None);
		return (score, mov, solver.explored_positions);
	};

	let mut solver = Solver::new(Some(Instant::now() + time_limit));

	// Fallback for `target_depth == 0` (or a deadline expiring before
	// depth 1 even completes): the position's static evaluation with
	// no move, matching what a direct `depth == 0` search would give.
	let mut best_score = evaluation(board);
	let mut best_move: Option<Move> = None;
	let mut hint: Option<Move> = None;

	for current_depth in 1..=target_depth {
		let (score, mov) = solver.negamax0(board, root_alpha, root_beta, current_depth, hint);
		if solver.time_up {
			break;
		}
		best_score = score;
		best_move = mov;
		hint = mov;
	}

	(best_score, best_move, solver.explored_positions)
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
	solve_iterative(board, depth, MIN_SCORE, MAX_SCORE, None)
}

pub fn partial_solve(board: &Board, depth: Option<u8>) -> (EvaluationScore, Option<Move>, u128) {
	solve_iterative(board, depth, -1, 1, None)
}

/// Like `solve`, but instead of only supporting a fixed target depth,
/// stops iterative deepening once `time_limit` has elapsed and
/// returns the best move found by the last depth that finished
/// searching in time.
pub fn solve_with_time_limit(
	board: &Board,
	depth: Option<u8>,
	time_limit: Duration,
) -> (EvaluationScore, Option<Move>, u128) {
	solve_iterative(board, depth, MIN_SCORE, MAX_SCORE, Some(time_limit))
}

/// Like `partial_solve`, with the same time budget semantics as
/// `solve_with_time_limit`.
pub fn partial_solve_with_time_limit(
	board: &Board,
	depth: Option<u8>,
	time_limit: Duration,
) -> (EvaluationScore, Option<Move>, u128) {
	solve_iterative(board, depth, -1, 1, Some(time_limit))
}


#[cfg(test)]
mod tests {
	use super::*;
	use std::assert_matches;

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
