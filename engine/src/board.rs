use rand::Rng;

use crate::color::Color;
use crate::mov::Move;
use crate::player::Player;
use crate::tileset::TileSet;
use std::iter::FromIterator;

// TODO: rip it off!!!!
#[derive(Clone, Copy)]
pub struct Board {
	pub pieces_mask: u64,
	pub black_mask: u64,
	pub current_player: Player,
	opponent: Player,
	pub played_tiles: TileSet,
	// pub possible_moves: &'a Vec<u8>
}

impl Board {
	pub fn empty() -> Board {
		let (current_player, opponent) = Player::default_set();
		Board {
			pieces_mask: 0,
			black_mask: 0,
			current_player,
			opponent,
			played_tiles: TileSet::new_unchecked(0),
		}
	}

	pub fn random_empty<R: Rng>(rng: &mut R) -> Board {
		let (current_player, opponent) = Player::random_set(rng);
		Board {
			pieces_mask: 0,
			black_mask: 0,
			current_player,
			opponent,
			played_tiles: TileSet::new_unchecked(0),
		}
	}

	/**
	 * Any Ascacou position may quite simply be described by a PGN
	 * like notation. For instance:
	 *
	 * ```text
	 *   a  b  c  d  e
	 * 1    W  B  W
	 * 2       B
	 * 3    W  W
	 * 4
	 * 5
	 * ```
	 *
	 * Could be refered to as `c2 B3 B1 C3 c1 D1`. This strings
	 * gives us the information of move order, and the case tells
	 * the color (white is upper cased).
	 *
	 * If we don't care about move order we can also have a FEN
	 * like reference: `1wbw/2b/1ww/5/5`. Note that we are starting
	 * from the top-left corner.
	 *
	 * An ascacou position must also be aware of which tiles the
	 * current player has. Since we are able to represent tiles
	 * with number between 0 and 15, we can represent a set of
	 * 8 tiles with 8 hexadecimal numbers. Those shall be the
	 * tiles of the current player, their opponent will have
	 * the 8 numbers left.
	 *
	 * So a full FEN may look like: 1wbw/2b/1ww/5/5 03478bcd.
	 *
	 * A Tile number is made by looking at the black pieces in a tile
	 * representation. These black pieces will represent bits that are
	 * on in our number, in the order indicated below.
	 *
	 * ```text
	 * 1 2
	 * 3 4
	 * ```
	 */
	pub fn from_fen(fen: &str) -> Result<Board, &'static str> {
		let mut black_mask = 0u64;
		let mut white_mask = 0u64;

		let mut x = 0; // Column
		let mut y = 0; // Row

		let mut chars = fen.chars().peekable();

		// Board part.
		while let Some(chr) = chars.next() {
			match chr {
				' ' if y < 4 => return Err("Not enough rows"),
				' ' => break,
				'/' if y == 4 => return Err("Too much rows"),
				'/' if x > 5 => return Err("Too much cols"),
				'/' => {
					y += 1;
					x = 0;
				}
				'1' => x += 1,
				'2' => x += 2,
				'3' => x += 3,
				'4' => x += 4,
				'5' => x += 5,
				'b' => {
					black_mask |= Board::position_mask(x, y);
					x += 1;
				}
				'w' => {
					white_mask |= Board::position_mask(x, y);
					x += 1;
				}
				_ => return Err("invalid character"),
			}
		}

		if chars.peek().is_none() {
			return Err("Incomplete FEN");
		}

		// Tiles part.

		let mut tiles = [0; 8];
		let mut i = 0;
		while let Some(chr) = chars.next() {
			match chr.to_digit(16) {
				// No need for a guard there, digit may not be greater than 15.
				Some(digit) => tiles[i] = digit as u8,
				None => return Err("invalid character"),
			}
			if i > 7 {
				return Err("Too much tiles");
			}
			i += 1;
		}

		if i < 7 {
			return Err("Not enough tiles");
		}

		let (current_player, opponent) = Player::from_current_tiles(tiles);
		let pieces_mask = black_mask | white_mask;

		Ok(Board {
			pieces_mask,
			black_mask,
			current_player,
			opponent,
			played_tiles: TileSet::from_iter(filled_tiles(&pieces_mask, &black_mask)),
		})
	}

	pub fn fen(&self) -> String {
		let mut str = String::new();
		for y in 0..5 {
			if y > 0 {
				str.push_str("/")
			}
			let mut idx = 0;
			for x in 0..5 {
				if Board::position_mask(x, y) & self.pieces_mask == 0 {
					idx += 1;
					continue;
				}

				if idx > 0 {
					str += &idx.to_string();
					idx = 0
				}
				str.push_str(if Board::position_mask(x, y) & self.black_mask == 0 {
					"w"
				} else {
					"b"
				})
			}
		}

		format!("{} {}", str, self.current_player.fen_part())
	}

	pub gen fn possible_moves(&self) -> Move {
		for x in 0..5 {
			for y in 0..5 {
				for color in [Color::Black, Color::White] {
					let mov = Move::new(x, y, color);
					if self.is_move_possible(&mov) {
						yield mov;
					}
				}
			}
		}
	}

	pub fn is_move_possible(&self, mov: &Move) -> bool {
		if mov.mask() & self.pieces_mask != 0 {
			return false;
		}
		if self.already_played_or_dup_move(mov) {
			return false;
		}
		true
	}

	pub fn is_invalid(&self) -> bool {
		let mut filled_tiles: [bool; 16] = [false; 16];
		for tile in self.filled_tiles() {
			if filled_tiles[tile as usize] {
				return true;
			}
			filled_tiles[tile as usize] = true;
		}
		false
	}

	pub fn current_score(&self) -> i8 {
		self.filled_tiles().fold(0, |acc, tile| {
			if self.current_player.has_tile(tile) {
				acc + 1
			} else {
				acc - 1
			}
		})
	}

	// TODO: what is the best option: using `already_played_or_dup_move()`
	// or `next().is_invalid()` ?
	pub fn is_terminal(&self) -> bool {
		self.played_tiles.full() || self.possible_moves().next().is_none()
	}

	pub fn is_winning(&self) -> bool {
		self.is_terminal() && self.current_score() > 0
	}

	/**
	 * Find every filled tiles using bit computation.
	 * If n is the number of filled tiles, this method
	 * is o(n), it is still quite computation heavy since
	 * we have to call `tile_at()` for each filled tile.
	 * Hence cost is roughly: 2 + 6 * n operations.
	 */
	fn filled_tiles(&self) -> impl Iterator<Item = u8> {
		filled_tiles(&self.pieces_mask, &self.black_mask)
	}

	/**
	 * Apply a move and check for validity.
	 */
	pub fn next(&self, mov: &Move) -> Option<Board> {
		let pos = mov.mask();
		if pos & self.pieces_mask != 0 {
			return None;
		}
		let tiles_from_move = self.tiles_from(mov)?;
		let played_tiles = self.played_tiles.try_union(&tiles_from_move)?;
		Some(Board {
			pieces_mask: self.pieces_mask | pos,
			black_mask: if mov.color == Color::Black {
				self.black_mask | pos
			} else {
				self.black_mask
			},
			current_player: self.opponent,
			opponent: self.current_player,
			played_tiles,
		})
	}

	// TODO(perf): maybe a simpler way to implement this algorithm is to play
	// the move and only then check for duplicates.
	/**
	 * A dup move is a move that generates two times the same tile, hence it is invalid.
	 */
	fn already_played_or_dup_move<'a>(&'a self, mov: &'a Move) -> bool {
		let Some(tiles_from_move) = self.tiles_from(&mov) else {
			return true; // dup move
		};
		match self.played_tiles.try_union(&tiles_from_move) {
			Some(_) => false,
			None => true, // already played
		}
	}

	/**
	 * From a empty square, checks which tiles could be created if we play
	 * a given move. This function doesn't check for emptiness of the square.
	 */
	fn tiles_from<'a>(&self, mov: &'a Move) -> Option<TileSet> {
		let mut tiles = TileSet::new_unchecked(0);
		let pos = mov.mask();
		// A new move impacts up to a 3x3 area.
		// We can represent it with a number
		// that we'll have to move exactly
		// like a position.
		//
		//    0  1  2  3  4  5  6
		//    7  8  9 10 11 12 13
		//   14 15 16 ...
		//
		// From the above grid, we need to set
		// bits  0, 1, 2, 7, 8, 9, 14, 15, 16.
		// Hence 0b0000111_0000111_0000111 is
		// the mask to move around.
		let impacted_area = Board::shift_rows(
			Board::shift_cols(0b0000111_0000111_0000111u64, mov.x as i8),
			mov.y as i8,
		);
		let new_mask = (self.pieces_mask | pos) & impacted_area;
		let new_black_mask = if mov.color == Color::Black {
			self.black_mask | pos
		} else {
			self.black_mask
		};

		for tile in filled_tiles(&new_mask, &new_black_mask) {
			tiles = tiles.try_add(tile)?;
		}

		Some(tiles)
	}

	fn position_mask(x: u8, y: u8) -> u64 {
		assert!(x < 5);
		assert!(y < 5);
		// Board::shift_rows(Board::shift_cols(256u64, x), y)
		Move::mask_at(x, y)
	}

	fn shift_rows(mask: u64, num_rows: i8) -> u64 {
		debug_assert!(-5 < num_rows && num_rows < 5);
		// See position_mask for the 7x7 board size explanation.
		if num_rows < 0 {
			mask >> -7 * num_rows
		} else {
			mask << 7 * num_rows
		}
	}

	fn shift_cols(mask: u64, num_cols: i8) -> u64 {
		debug_assert!(-5 < num_cols && num_cols < 5);
		// See position_mask for the 7x7 board size explanation.
		if num_cols < 0 {
			mask >> -num_cols
		} else {
			mask << num_cols
		}
	}

	pub fn for_console(&self) -> String {
		let mut str = String::new();
		str.push_str(&self.fen());
		str.push('\n');

		let filled_tiles: Vec<u8> = self.filled_tiles().collect();
		str.push_str(&self.opponent.for_console(&filled_tiles));

		let spacing = " ".repeat((46/* tiles len */ - 12/* board len */) / 2);

		str.push('\n');
		str.push_str(&spacing);
		str.push_str("   a b c d e\n");
		for y in 0..5 {
			str.push_str(&spacing);
			str.push_str(&(y + 1).to_string());
			str.push_str(" \x1b[47m");

			for x in 0..5 {
				let position = Board::position_mask(x, y);

				str.push_str(" ");

				if self.pieces_mask & position == 0 {
					str.push_str("\x1b[30m");
					match (
						self.next(&Move::black(x, y)).is_none(),
						self.next(&Move::white(x, y)).is_none(),
					) {
						(false, false) => str.push('·'),
						(false, true) => str.push('b'),
						(true, false) => str.push('w'),
						(true, true) => str.push('x'),
					}
				} else if self.black_mask & position != 0 {
					str.push_str(format!("{}●", Color::Black).as_str());
				} else {
					str.push_str(format!("{}●", Color::White).as_str());
				}
			}
			str.push_str(" \x1b[0m\n");
		}
		str.push('\n');
		str.push('\n');
		str.push_str(&self.current_player.for_console(&filled_tiles));

		str
	}
}

gen fn filled_tiles<'a>(pieces_mask: &'a u64, black_mask: &'a u64) -> u8 {
	// First, we retrieve every top-left corners of filled tiles
	let mut mask = pieces_mask & (pieces_mask >> 1);
	mask = mask & (mask >> 7);
	while mask != 0 {
		let prev = mask;
		mask &= mask - 1;
		let top_left = prev - mask;
		// Then we find the tile value associated
		// for each tile.
		yield tile_at(black_mask, top_left)
	}
}

fn tile_at(black_mask: &u64, top_left: u64) -> u8 {
	let top_left_shift = top_left.trailing_zeros();

	let a = (black_mask & top_left) >> top_left_shift;
	let b = (black_mask & (top_left << 1)) >> top_left_shift;
	let c = (black_mask & (top_left << 7)) >> (top_left_shift + 5);
	let d = (black_mask & (top_left << 8)) >> (top_left_shift + 5);

	(a | b | c | d) as u8
}

impl std::convert::TryFrom<&str> for Board {
	type Error = &'static str;
	fn try_from(s: &str) -> Result<Self, Self::Error> {
		Board::from_fen(s)
	}
}

impl std::fmt::Display for Board {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.fen())
	}
}

impl std::fmt::Debug for Board {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Board({})", self.fen())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::bench::Bencher;

	macro_rules! assert_tile {
		($board:expr, ($x:expr, $y:expr), $tile:expr) => {
			let result = tile_at(&$board.black_mask, Board::position_mask($x, $y));
			assert_eq!(
				result, $tile,
				"\nExpected: {:0>4b},\n     got: {:0>4b}",
				$tile, result
			);
		};
	}

	#[test]
	fn test_tile_at() {
		let mut board = Board::from_fen("bw/ww/5/5/5 01234567").unwrap();
		assert_tile!(board, (0, 0), 0b0001);
		board = Board::from_fen("bb/ww/5/5/5 01234567").unwrap();
		assert_tile!(board, (0, 0), 0b0011);
		board = Board::from_fen("bb/bw/5/5/5 01234567").unwrap();
		assert_tile!(board, (0, 0), 0b0111);
		board = Board::from_fen("bb/bb/5/5/5 01234567").unwrap();
		assert_tile!(board, (0, 0), 0b1111);
		board = Board::from_fen("wwbb/wwbbb/wb/5/5 01234567").unwrap();
		assert_tile!(board, (0, 0), 0b0000);
		assert_tile!(board, (1, 0), 0b1010);
		assert_tile!(board, (2, 0), 0b1111);
		assert_tile!(board, (2, 0), 0b1111);
		assert_tile!(board, (0, 1), 0b1000);
	}

	#[test]
	fn test_next() {
		let board = Board::from_fen("bb1ww/www1w/1bbw/1bww/2w 2689abce").unwrap();
		let mov = Move::try_from("bc1").unwrap();
		assert!(board.next(&mov).is_none());
	}

	#[test]
	fn test_from_fen_set_played_tiles() {
		let board = Board::from_fen("bb1ww/www1w/1bbw/1bww/2w 2689abce").unwrap();
		let expected_played_tiles: TileSet = TileSet::new_unchecked(0b0001_0000_1000_1010);
		assert_eq!(board.played_tiles, expected_played_tiles);
	}

	#[test]
	fn current_score() {
		assert_eq!(
			Board::from_fen("1wbw/2b/1bb/5/5 01234567")
				.unwrap()
				.current_score(),
			0
		);
		assert_eq!(
			Board::from_fen("1wbw/2bw/1bb/5/5 89abcdef")
				.unwrap()
				.current_score(),
			-1, // score for p2
			"Score was not -1 for board below\n{}",
			Board::from_fen("1wbw/2bw/1bb/5/5 89abcdef").unwrap()
		);
		assert_eq!(
			Board::from_fen("1wbw/2bb/1bb/5/5 89abcdef")
				.unwrap()
				.current_score(),
			1 // score for p2
		);
	}

	#[test]
	fn tiles_from() {
		let mov = Move::try_from("wd2").unwrap();
		let board = Board::from_fen("2wwb/2w1b/2wbb/5/5 01234567").unwrap();
		println!("{}\nChecking tiles from move {}", board, mov);
		assert_eq!(board.tiles_from(&mov).unwrap(), vec![0, 10, 8, 14].into())
	}

	#[test]
	fn position_mask() {
		assert_eq!(
			Board::position_mask(0, 0),
			1u64 << 8,
			"position (0, 0) is incorrect"
		);
		let expected = 1u64 << 9;
		assert_eq!(
			Board::position_mask(1, 0),
			expected,
			"position (1, 0) is incorrect"
		);
		assert_eq!(
			Board::position_mask(0, 1),
			1u64 << 15,
			"position (0, 1) is incorrect"
		)
	}

	#[test]
	fn impossible_move_generating_two_times_the_same_tile() {
		let board = Board::from_fen("wbbww/wbwbw/b1w1b/bbwww/wwwwb 034567ef").unwrap();
		println!("{}", board.for_console());
		assert_eq!(board.possible_moves().collect::<Vec<Move>>(), vec![]);
	}

	#[test]
	fn fen_is_consistent() {
		let mut fen = "2b1b/wwb1w/w1bw/bw1w/bw2b 137abcdf";
		assert_eq!(Board::from_fen(fen).unwrap().fen(), fen);
		fen = "5/5/5/5/5 01234567";
		assert_eq!(Board::from_fen(fen).unwrap().fen(), "//// 01234567");
	}

	#[bench]
	fn bench_is_move_possible(b: &mut Bencher) {
		for fen in vec![
			"2b1b/wwb1w/w1bw/bw1w/bw2b 137abcdf",
			"1bwwb/wwwwb/b/w2wb/2b1w 14579ace",
			"bw/1ww/1bb1w//b1ww 0367abce",
			"bb/wb1b/2w/4w/1w1bw 146789bf",
			"/4b/4w/b2w/4b 35679acd",
			"4b//b1b1b//3b 013578ab",
			"//// 2456abcd",
			"//// 456abcdf",
		] {
			let board = Board::from_fen(fen).unwrap();
			b.iter(|| {
				for x in 0..5 {
					for y in 0..5 {
						for color in [Color::Black, Color::White] {
							let mov = Move::new(x, y, color);
							let _ = board.is_move_possible(&mov);
						}
					}
				}
			});
		}
	}
}
