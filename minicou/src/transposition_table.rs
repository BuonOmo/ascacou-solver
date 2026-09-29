use ascacou::{Board, Color, Move};

use crate::solver::EvaluationScore;

/// Whether a stored score is exact, or only a bound
/// (because the search was cut short by alpha-beta
/// pruning).
///
/// See <https://www.chessprogramming.org/Node_Types>.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Bound {
	/// The stored score is the exact minimax value of the position.
	Exact,
	/// The real score is at least the stored value (a beta cutoff
	/// occurred, i.e. a fail-high).
	Lower,
	/// The real score is at most the stored value (every move failed
	/// to raise alpha, i.e. a fail-low).
	Upper,
}

/// A compact encoding of a `Move`: the square index (0..25) in the
/// low 5 bits, and the color in the 6th bit. `NONE` is used as a
/// sentinel for "no move".
#[derive(Copy, Clone, PartialEq, Eq)]
struct MoveCode(u8);

impl MoveCode {
	const NONE: MoveCode = MoveCode(0xFF);

	fn from_move(mov: Move) -> MoveCode {
		let square = mov.y() * 5 + mov.x();
		let color_bit = if mov.is_black() { 0 } else { 1 << 5 };
		MoveCode(square | color_bit)
	}

	fn to_move(self) -> Option<Move> {
		if self == MoveCode::NONE {
			return None;
		}
		let square = self.0 & 0b11111;
		let color = if self.0 & (1 << 5) == 0 {
			Color::Black
		} else {
			Color::White
		};
		Some(Move::new(square % 5, square / 5, color))
	}
}

impl Default for MoveCode {
	fn default() -> Self {
		MoveCode::NONE
	}
}

/// A single transposition table entry.
///
/// Storing the search depth and the bound type (in addition to the
/// plain score) lets us reuse an entry as a real alpha-beta cutoff
/// (not only to narrow the search window), and storing the best
/// move lets us reorder moves at a node to search the most
/// promising one first (see move ordering in the search).
#[derive(Copy, Clone)]
struct Entry {
	key: Key,
	score: EvaluationScore,
	depth: u8,
	bound: Bound,
	best_move: MoveCode,
}

impl Entry {
	const EMPTY: Entry = Entry {
		key: Key::EMPTY,
		score: 0,
		depth: 0,
		bound: Bound::Exact,
		best_move: MoveCode::NONE,
	};
}

/// A packed version of our 50 bit key (see `key()`), split so that
/// `Entry` doesn't need padding to stay small. The high bit of the
/// second half is always set for real keys (our keys only use the
/// low 50 bits), so it doubles as an "occupied" marker: it lets us
/// tell an empty slot (`Key::EMPTY`, both halves zero) apart from an
/// actual stored key of `0` (e.g. the empty board), which would
/// otherwise collide with the sentinel.
#[derive(Copy, Clone, PartialEq, Eq)]
struct Key(u32, u32);

impl Key {
	const EMPTY: Key = Key(0, 0);
	const OCCUPIED_BIT: u32 = 1 << 31;

	fn from_u64(key: u64) -> Key {
		Key((key & 0xFFFF_FFFF) as u32, ((key >> 32) as u32) | Key::OCCUPIED_BIT)
	}
}

pub struct TranspositionTable {
	table: Vec<Entry>,
	size: usize,
}

pub struct ProbeResult {
	pub score: EvaluationScore,
	pub depth: u8,
	pub bound: Bound,
	// Not consumed yet: move ordering (searching this move first)
	// will be wired in as part of iterative deepening.
	#[allow(dead_code)]
	pub best_move: Option<Move>,
}

impl TranspositionTable {
	pub fn new(size: usize) -> Self {
		TranspositionTable {
			table: vec![Entry::EMPTY; size],
			size,
		}
	}

	fn index(&self, key: u64) -> usize {
		(key as usize) % self.size
	}

	pub fn get(&self, key: u64) -> Option<ProbeResult> {
		let entry = &self.table[self.index(key)];
		let entry_key = Key::from_u64(key);
		// An `Entry::EMPTY` slot has a null move and 0 depth, so
		// even a hash collision with an empty slot is harmless:
		// it simply won't be usable as a cutoff and will report
		// no best move.
		if entry.key != entry_key {
			return None;
		}
		Some(ProbeResult {
			score: entry.score,
			depth: entry.depth,
			bound: entry.bound,
			best_move: entry.best_move.to_move(),
		})
	}

	pub fn insert(
		&mut self,
		key: u64,
		score: EvaluationScore,
		depth: u8,
		bound: Bound,
		best_move: Option<Move>,
	) {
		let index = self.index(key);
		// Always-replace strategy: the newest entry is the one
		// resulting from the most up-to-date iterative deepening
		// pass, so it is a reasonable default. Depth-preferred
		// replacement could be explored later.
		self.table[index] = Entry {
			key: Key::from_u64(key),
			score,
			depth,
			bound,
			best_move: best_move.map(MoveCode::from_move).unwrap_or(MoveCode::NONE),
		};
	}
}

/// Our boards are represented internally on a 7 by 7 grid, but we
/// only care about the 5 by 5 center part. So we can fit each mask
/// in 25 consecutive bits.
///
/// See also <https://www.chessprogramming.org/Transposition_Table>.
fn fit(value: u64) -> u64 {
	let a = (value & 0b0111110_0000000) >> (1 * 7 + 1 - 0 * 5);
	let b = (value & 0b0111110_0000000_0000000) >> (2 * 7 + 1 - 1 * 5);
	let c = (value & 0b0111110_0000000_0000000_0000000) >> (3 * 7 + 1 - 2 * 5);
	let d = (value & 0b0111110_0000000_0000000_0000000_0000000) >> (4 * 7 + 1 - 3 * 5);
	let e = (value & 0b0111110_0000000_0000000_0000000_0000000_0000000) >> (5 * 7 + 1 - 4 * 5);
	a | b | c | d | e
}

/// Build a 50 bit transposition table key out of a board: 25 bits
/// for which squares are filled, and 25 bits for which of those are
/// black. This is enough to uniquely identify a board position
/// (ignoring tile ownership, which the caller must account for
/// separately if it can vary within a single search).
pub fn key(board: &Board) -> u64 {
	fit(board.pieces_mask) | (fit(board.black_mask) << 25)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_fit() {
		macro_rules! assert_fits {
			($value:expr, $expected:expr) => {
				let fitted = fit($value);
				assert_eq!(
					fitted, $expected,
					"\n left: {:025b}\nright: {:025b}",
					fitted, $expected
				);
			};
		}
		let value: u64 = 0b0111110_0111110_0111110_0111110_0111110_0000000;
		assert_fits!(value, 0b11111_11111_11111_11111_11111u64);

		let value: u64 = 0b0011110_0101110_0110110_0111010_0111100_0000000;
		assert_fits!(value, 0b01111_10111_11011_11101_11110u64);

		let value: u64 = 0b0000010_0000100_0001000_0010000_0100000_0000000;
		assert_fits!(value, 0b00001_00010_00100_01000_10000u64);

		let value: u64 = 0b1111111_1000001_1000001_1000001_1000001_1000001_1111111;
		assert_fits!(value, 0u64);
	}

	#[test]
	fn test_move_code_roundtrip() {
		for x in 0..5 {
			for y in 0..5 {
				for color in [Color::Black, Color::White] {
					let mov = Move::new(x, y, color);
					let code = MoveCode::from_move(mov);
					assert_eq!(code.to_move(), Some(mov));
				}
			}
		}
	}

	#[test]
	fn test_move_code_none() {
		assert_eq!(MoveCode::NONE.to_move(), None);
	}

	#[test]
	fn test_insert_and_get() {
		let mut table = TranspositionTable::new(97);
		let board = Board::empty();
		let k = key(&board);
		assert!(table.get(k).is_none());

		let mov = Move::black(2, 2);
		table.insert(k, 4, 7, Bound::Exact, Some(mov));

		let result = table.get(k).unwrap();
		assert_eq!(result.score, 4);
		assert_eq!(result.depth, 7);
		assert_eq!(result.bound, Bound::Exact);
		assert_eq!(result.best_move, Some(mov));
	}
}
