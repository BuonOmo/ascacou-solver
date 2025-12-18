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
	pub played_moves: u8,
}

impl Board {
	pub fn empty() -> Board {
		let (current_player, opponent) = Player::default_set();
		Board {
			pieces_mask: 0,
			black_mask: 0,
			current_player,
			opponent,
			played_tiles: TileSet::empty(),
			played_moves: 0,
		}
	}

	pub fn random_empty<R: Rng>(rng: &mut R) -> Board {
		let (current_player, opponent) = Player::random_set(rng);
		Board {
			pieces_mask: 0,
			black_mask: 0,
			current_player,
			opponent,
			played_tiles: TileSet::empty(),
			played_moves: 0,
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
					black_mask |= Move::mask_at(x, y);
					x += 1;
				}
				'w' => {
					white_mask |= Move::mask_at(x, y);
					x += 1;
				}
				_ => return Err("invalid character"),
			}
		}

		if chars.peek().is_none() {
			return Err("Incomplete FEN");
		}

		// Tiles part.

		let mut tiles = TileSet::empty();
		let mut i = 0;
		while let Some(chr) = chars.next() {
			match chr.to_digit(16) {
				// No need for a guard there, digit may not be greater than 15.
				Some(digit) => tiles = tiles.try_add(digit as u8).ok_or("Duplicate tile")?,
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

		let (current_player, opponent) = tiles.into();
		let pieces_mask = black_mask | white_mask;

		Ok(Board {
			pieces_mask,
			black_mask,
			current_player,
			opponent,
			played_tiles: TileSet::from_iter(filled_tiles(pieces_mask, black_mask)),
			played_moves: pieces_mask.count_ones() as u8,
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
				if Move::mask_at(x, y) & self.pieces_mask == 0 {
					idx += 1;
					continue;
				}

				if idx > 0 {
					str += &idx.to_string();
					idx = 0
				}
				str.push_str(if Move::mask_at(x, y) & self.black_mask == 0 {
					"w"
				} else {
					"b"
				})
			}
		}

		format!("{} {}", str, self.current_player.fen_part())
	}

	pub gen fn possible_moves(&self) -> Move {
		let board_full_mask = 0b0000000_0111110_0111110_0111110_0111110_0111110_0000000u64;
		let mut available_spots = !self.pieces_mask & board_full_mask;
		while available_spots != 0 {
			// TODO(perf): benchmark the most efficient way to iterate over bits
			// and extract that in a util crate.
			//
			// See https://graphics.stanford.edu/~seander/bithacks.html
			let i = available_spots.trailing_zeros();
			let mov_mask = 1u64 << i;
			available_spots ^= mov_mask;

			for color in [Color::Black, Color::White] {
				let mov = Move::from_mask(mov_mask, color);
				if self.is_move_possible(&mov) {
					yield mov;
				}
			}
		}
	}

	pub fn is_move_possible(&self, mov: &Move) -> bool {
		if mov.mask & self.pieces_mask != 0 {
			return false;
		}
		if self.already_played_or_dup_move(mov) {
			return false;
		}
		true
	}

	// TODO(perf): We could use the fact that we now have
	// _played_tiles_ to speed up the score computation.
	// We could use bitmap comparison for filled tiles
	// and for played tiles, and compare those two
	// bitmaps.
	pub fn current_score(&self) -> i8 {
		self.filled_tiles().fold(0, |acc, tile| {
			if self.current_player.has_tile(tile) {
				acc + 1
			} else {
				acc - 1
			}
		})
	}

	pub fn is_terminal(&self) -> bool {
		self.played_tiles.is_full() || self.possible_moves().next().is_none()
	}

	pub fn is_winning(&self) -> bool {
		self.is_terminal() && self.current_score() > 0
	}

	fn filled_tiles(&self) -> FilledTilesIterator {
		filled_tiles(self.pieces_mask, self.black_mask)
	}

	/// Apply a move and check for validity.
	pub fn next(&self, mov: &Move) -> Option<Board> {
		let pos = mov.mask;
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
			played_moves: self.played_moves + 1,
		})
	}

	/// A dup move is a move that generates two times the same tile, hence it is invalid.
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
		let mut tiles = TileSet::empty();
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
		let impacted_area = 0b0000111_0000111_0000111u64 << mov.shift();
		let new_mask = (self.pieces_mask | mov.mask) & impacted_area;
		let new_black_mask = if mov.color == Color::Black {
			self.black_mask | mov.mask
		} else {
			self.black_mask
		};

		for tile in filled_tiles(new_mask, new_black_mask) {
			tiles = tiles.try_add(tile)?;
		}

		Some(tiles)
	}

	pub fn for_console(&self) -> String {
		let mut str = String::new();
		str.push_str(&self.fen());
		str.push('\n');

		let filled_tiles: TileSet = self.filled_tiles().collect();
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
				let position = Move::mask_at(x, y);

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

/// Find every filled tiles using bit computation.
/// If n is the number of filled tiles, this method
/// is o(n), it is still quite computation heavy since
/// we have to call `tile_at()` for each filled tile.
/// Hence cost is roughly: 2 + (2 + 4 + 6) * n operations.
/// The 6 being the cost of `trailing_zeros`.
fn filled_tiles(pieces_mask: u64, black_mask: u64) -> FilledTilesIterator {
	let mut mask = pieces_mask;
	// 1 1 & 1 0 = 1 0
	// 1 1   1 0   1 0
	mask = mask & (mask >> 1);
	// 1 0 & 1 0 = 1 0
	// 1 0   0 0   0 0
	mask = mask & (mask >> 7);
	FilledTilesIterator { mask, black_mask }
}

struct FilledTilesIterator {
	mask: u64,
	black_mask: u64,
}

impl Iterator for FilledTilesIterator {
	type Item = u8;

	fn next(&mut self) -> Option<Self::Item> {
		if self.mask == 0 {
			return None;
		}
		let prev = self.mask;
		self.mask &= self.mask - 1;
		let top_left = prev - self.mask;
		Some(tile_at(&self.black_mask, top_left))
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
			let result = tile_at(&$board.black_mask, Move::mask_at($x, $y));
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
		let expected_played_tiles: TileSet = TileSet::new(0b0001_0000_1000_1010);
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
	fn test_tiles_from() {
		let mov = Move::try_from("wd2").unwrap();
		let board = Board::from_fen("2wwb/2w1b/2wbb/5/5 01234567").unwrap();
		println!("{}\nChecking tiles from move {}", board, mov);
		assert_eq!(board.tiles_from(&mov).unwrap(), vec![0, 10, 8, 14].into())
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
