use crate::color::Color;
use crate::mov::Move;
use crate::player::Player;
use crate::tileset::TileSet;
use std::iter::FromIterator;

/**
 * The size of a key representing a unique position.
 */
pub use std::primitive::u128 as BoardKey;

// TODO: rip it off!!!!
#[derive(Clone, Copy)]
pub struct Board {
	pieces_mask: u64,
	black_mask: u64,
	pub current_player: Player,
	opponent: Player,
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
		}
	}

	pub fn random_empty() -> Board {
		let (current_player, opponent) = Player::random_set();
		Board {
			pieces_mask: 0,
			black_mask: 0,
			current_player,
			opponent,
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

		Ok(Board {
			pieces_mask: (black_mask | white_mask),
			black_mask,
			current_player,
			opponent,
		})
	}

	pub fn fen(&self) -> String {
		let mut idx = 0;
		let mut str = String::new();
		for y in 0..5 {
			if y > 0 {
				str.push_str("/")
			}
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
			if idx > 0 {
				str += &idx.to_string();
				idx = 0
			}
		}

		format!("{} {}", str, self.current_player.fen_part())
	}

	pub fn key(&self) -> BoardKey {
		// TODO(memory perf): find a way to store key in a smaller size (see patch ':/u64 key').
		(self.pieces_mask as BoardKey) | ((self.black_mask as BoardKey) << 64)
	}

	pub fn possible_moves(&self) -> Vec<Move> {
		let mut result: Vec<Move> = Vec::with_capacity(50);

		for color in [
			self.current_player.favorite_color,
			!self.current_player.favorite_color,
		] {
			// First, the center, most likely to change the score.
			for x in 1..4 {
				for y in 1..4 {
					self.add_move(x, y, color, &mut result);
				}
			}

			// Then the edges.
			for i in 1..4 {
				for j in [0, 4] {
					self.add_move(i, j, color, &mut result);
					self.add_move(j, i, color, &mut result);
				}
			}

			// Finally the corners.
			for x in [0, 4] {
				for y in [0, 4] {
					self.add_move(x, y, color, &mut result);
				}
			}
		}

		result
	}

	fn add_move(&self, x: i8, y: i8, color: Color, vec: &mut Vec<Move>) {
		if Board::position_mask(x, y) & self.pieces_mask != 0 {
			return;
		}
		let mov = Move::new(x, y, color);
		if self.next(mov).is_invalid() {
			return;
		}
		vec.push(mov);
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

	// TODO: a smarter score computation could be done by taking into
	// account each player's score, and give a greater edge to a position
	// close to terminal. More interesting even is the idea of taking into
	// account partially filled tiles.
	pub fn current_score(&self) -> i8 {
		let mut score = 0;

		for tile in self.filled_tiles() {
			if self.current_player.has_tile(tile) {
				score += 1;
			} else {
				score -= 1;
			}
		}

		score
	}

	pub fn is_terminal(&self) -> bool {
		let tiles = &TileSet::from_iter(self.filled_tiles());

		for x in 0..5 {
			for y in 0..5 {
				// Already a piece there.
				if Board::position_mask(x, y) & self.pieces_mask != 0 {
					continue;
				}

				for color in [Color::Black, Color::White] {
					let mov = Move::new(x, y, color);
					if self.already_played_or_dup_move(&mov, tiles) {
						continue;
					}

					return false;
				}
			}
		}

		true
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
	fn filled_tiles(&self) -> impl Iterator<Item = u8> + '_ {
		// First, we retrieve every top-left corners of filled tiles
		let mut mask = self.pieces_mask & (self.pieces_mask >> 1);
		mask = mask & (mask >> 7);
		std::iter::from_fn(move || {
			if mask == 0 {
				return None;
			}
			let prev = mask;
			mask &= mask - 1;
			let top_left = prev - mask;
			// Then we find the tile value associated
			// for each tile.
			Some(self.tile_at(top_left))
		})
	}

	/**
	 * Apply a move without checking for its validity.
	 */
	pub fn next(&self, mov: Move) -> Board {
		let pos = Board::position_mask(mov.x, mov.y);
		Board {
			pieces_mask: self.pieces_mask | pos,
			black_mask: if mov.color == Color::Black {
				self.black_mask | pos
			} else {
				self.black_mask
			},
			current_player: self.opponent,
			opponent: self.current_player,
		}
	}

	// TODO(perf): maybe a simpler way to implement this algorithm is to play
	// the move and only then check for duplicates.
	/**
	 * A dup move is a move that generates two times the same tile, hence it is invalid.
	 */
	fn already_played_or_dup_move<'a>(
		&'a self,
		mov: &'a Move,
		exhausted_tiles: &'a TileSet,
	) -> bool {
		let mut dup_list: [bool; 16] = [false; 16];
		for tile in self.tiles_from(&mov) {
			if exhausted_tiles.has(tile) {
				return true;
			}
			if dup_list[tile as usize] {
				return true;
			}

			dup_list[tile as usize] = true;
		}
		return false;
	}

	/**
	 * From a empty square, checks which tiles could be created if we play
	 * a given move. This function doesn't check for emptiness of the square.
	 */
	fn tiles_from<'a>(&self, mov: &'a Move) -> Vec<u8> {
		let mut tiles = Vec::with_capacity(4);
		let pos = Board::position_mask(mov.x, mov.y);
		let mask_with_new_piece = self.pieces_mask | pos;

		// TODO(refacto): consider using bottom right rather than top left to work only with unsigned?
		for dy in [-1i8, 0] {
			for dx in [-1i8, 0] {
				let tile_mask = Board::tile_mask(mov.x + dx, mov.y + dy);
				if tile_mask & mask_with_new_piece == tile_mask {
					let mut tile = self.tile_at(Board::position_mask(mov.x + dx, mov.y + dy));
					// We have to compute the missing value of our tile.
					if mov.color == Color::Black {
						tile |= 1 << -2 * dy << -dx;
					}
					tiles.push(tile);
				}
			}
		}

		tiles
	}

	/**
	 * Return the tile number at a given position. This method
	 * assumes that the tile is full of pieces.
	 */
	const fn tile_at(&self, top_left: u64) -> u8 {
		let mut tile = 0u8;
		if top_left & self.black_mask != 0 {
			tile |= 1
		}
		if (top_left << 1) & self.black_mask != 0 {
			tile |= 2
		}
		if (top_left << 7) & self.black_mask != 0 {
			tile |= 4
		}
		if (top_left << 8) & self.black_mask != 0 {
			tile |= 8
		}
		tile
	}

	fn position_mask(x: i8, y: i8) -> u64 {
		/* Masks are 7x7 rather than 5x5 to allow simpler bitboard
		 * computation. This means that when we shift bits by N row
		 * or column, they will fall in an unchecked area we don't
		 * care about. Note that this was an implementation choice
		 * beforehand, and it may be reduced to optimize storage.
		 *
		 * With that said, our first (x, y) should start at bit 8.
		 */
		Board::shift_rows(Board::shift_cols(256u64, x), y)
	}

	fn tile_mask(x: i8, y: i8) -> u64 {
		/* A tile has four bits on, hence
		 * we can represent it with a number
		 * that we'll have to move exactly
		 * like a position.
		 *
		 *    0  1  2  3  4  5  6
		 *    7  8  9 10 11 12 13
		 *   14 15 16 ...
		 *
		 * From the above grid, bits 8, 9, 15 and 16
		 * will correspond to a tile on the first
		 * place of our grid. Hence 99072 is the
		 * tile to move.
		 */
		Board::shift_rows(Board::shift_cols(99072u64, x), y)
	}

	fn shift_rows(mask: u64, num_rows: i8) -> u64 {
		#[cfg(debug_assertions)]
		{
			assert!(-5 < num_rows && num_rows < 5)
		}
		// See position_mask for the 7x7 board size explanation.
		if num_rows < 0 {
			mask >> -7 * num_rows
		} else {
			mask << 7 * num_rows
		}
	}

	fn shift_cols(mask: u64, num_cols: i8) -> u64 {
		#[cfg(debug_assertions)]
		{
			assert!(-5 < num_cols && num_cols < 5)
		}
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
					if self.next(Move::black(x, y)).is_invalid()
						&& self.next(Move::white(x, y)).is_invalid()
					{
						str.push_str("\x1b[30mx");
					} else {
						str.push_str("\x1b[30m·");
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

#[cfg(test)]
mod tests {
	use super::*;

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
		assert_eq!(board.tiles_from(&mov), vec![0, 10, 8, 14])
	}

	#[test]
	fn position_mask() {
		assert_eq!(
			Board::position_mask(0, 0),
			1u64 << 8,
			"position (0, 0) is incorrect"
		);
		assert_eq!(
			Board::position_mask(1, 0),
			1u64 << 9,
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
		assert_eq!(board.possible_moves(), vec![]);
	}
}
