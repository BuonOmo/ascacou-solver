use crate::color::Color;
use crate::mov::Move;
use crate::player::Player;
use crate::tileset::TileSet;

/**
 * The size of a key representing a unique position.
 */
pub use u128 as Key;

pub struct Board {
	pieces_mask: u64,
	black_mask:  u64,
	white_mask:  u64,
	pub current_player: Player,
	opponent: Player
}

impl Board {
	pub fn empty() -> Board {
		let (current_player, opponent) = Player::default_set();
		Board { pieces_mask: 0, black_mask: 0, white_mask: 0,
			current_player, opponent }
	}

	/**
	 * Any Ascacou position may quite simply be described by a PGN
	 * like notation. For instance:
	 *
	 *       a  b  c  d  e
	 *     1    W  B  W
	 *     2       B
	 *     3    W  W
	 *     4
	 *     5
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
	 *     1 2
	 *     3 4
	 */
	pub fn from_fen(fen: &str) -> Result<Board, &'static str> {
		let mut black_mask =  0u64;
		let mut white_mask =  0u64;

		let mut x = 0;
		let mut y = 0;

		let mut chars = fen.chars().peekable();

		// Board part.
		while let Some(chr) = chars.next() {
			match chr {
				' ' if x < 4 => return Err("Not enough rows"),
				' ' => break,
				'/' if x == 4 => return Err("Too much rows"),
				'/' if y > 4 => return Err("Too much cols"),
				'/' => { x += 1; y = 0 }
				'1' => y += 0,
				'2' => y += 1,
				'3' => y += 2,
				'4' => y += 3,
				'5' => y += 4,
				'b' => black_mask |= Board::position_mask(x, y),
				'w' => white_mask |= Board::position_mask(x, y),
				_ => return Err("invalid character")
			}
		}

		if chars.peek().is_none() {
			return Err("Incomplete FEN")
		}

		// Tiles part.

		let mut tiles = [0; 8];
		let mut i = 0;
		while let Some(chr) = chars.next() {
			match chr.to_digit(16) {
				// No need for a guard there, digit may not be greater than 15.
				Some(digit) => tiles[i] = digit as u8,
				None => return Err("invalid character")
			}
			if i > 7 { return Err("Too much tiles") }
			i += 1;
		}

		if i < 7 { return Err("Not enough tiles") }

		let (current_player, opponent) = Player::from_current_tiles(tiles);

		Ok(Board { pieces_mask: (black_mask | white_mask),
			black_mask, white_mask, current_player, opponent })
	}

	fn fen(&self) -> String {
		let mut idx = 0;
		let mut str = String::new();
		for x in 0..5 {
			if x > 0 { str.push_str("/") }
			for y in 0..5 {
				if Board::position_mask(x, y) & self.pieces_mask == 0 {
					idx += 1;
					continue
				}

				if idx > 0 { str += &idx.to_string(); idx = 0 }
				str.push_str(
					if Board::position_mask(x, y) & self.black_mask == 0 { "w" } else { "b" }
				)
			}
			if idx > 0 { str += &idx.to_string(); idx = 0 }
		}
		str
	}


	pub fn key(&self) -> Key { // TODO(memory perf): find a way to store key in a smaller size (see patch ':/u64 key').
		(self.pieces_mask as Key) | ((self.black_mask as Key) << 64)
	}

	pub fn possible_moves(&self) -> Vec<Move> {
		let mut result: Vec<Move> = Vec::with_capacity(50);

		let tiles = TileSet::from(self.filled_tiles());

		for x in 0..5 {
			for y in 0..5 {
				// Already a piece there.
				if Board::position_mask(x, y) & self.pieces_mask != 0 { continue }

				for color in [Color::Black, Color::White] {
					let mov = Move::new(x, y, color);
					if self.already_played(&mov, &tiles) { continue }

					result.push(mov)
				}
			}
		}

		result
	}

	// TODO: a smarter score computation could be done by taking into
	// account each player's score, and give a greater edge to a position
	// close to terminal.
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

	fn filled_tiles(&self) -> Vec<u8> {
		let mut filled_tiles: Vec<u8> = Vec::with_capacity(16);
		// First, we retrieve every top-left corners of filled tiles
		let mut mask = self.pieces_mask & (self.pieces_mask >> 1);
		mask = mask & (mask >> 7);
		let mut prev;
		while mask != 0 {
			prev = mask;
			mask &= mask - 1;
			let top_left = prev - mask;
			// Then we find the tile value associated
			// for each tile.
			filled_tiles.push(self.tile_at(top_left));
		}
		filled_tiles
	}

	/**
	 * Apply a move without checking for its validity.
	 */
	pub fn next(&self, mov: Move) -> Board {
		let pos = Board::position_mask(mov.x, mov.y);
		Board {
			pieces_mask: self.pieces_mask | pos,
			white_mask: if mov.color == Color::White { self.white_mask | pos } else { self.white_mask },
			black_mask: if mov.color == Color::Black { self.black_mask | pos } else { self.black_mask },
			current_player: self.opponent,
			opponent: self.current_player
		}
	}

	// TODO(perf): maybe a simpler way to implement this algorithm is to play
	// the move and only then check for duplicates.
	fn already_played<'a>(&self, mov: &'a Move, exhausted_tiles: &'a TileSet) -> bool {
		for tile in self.tiles_from(&mov) {
			if exhausted_tiles.has(tile) { return true }
		}
		return false
	}

	/**
	 * From a empty square, checks which tiles could be created if we play
	 * a given move. This function doesn't check for emptiness of the square.
	 */
	fn tiles_from<'a>(&self, mov: &'a Move) -> std::collections::LinkedList<u8> {
		let mut tiles = std::collections::LinkedList::new();
		let pos = Board::position_mask(mov.x, mov.y);
		let mask_with_new_piece = self.pieces_mask | pos;

		// TODO(refacto): consider using bottom right rather than top left to work only with unsigned?
		for dx in [-1i8, 0] {
			for dy in [-1i8, 0] {
				let tile_mask = Board::tile_mask(mov.x + dx, mov.y + dy);
				if tile_mask & mask_with_new_piece == tile_mask {
					let mut tile = self.tile_at(Board::position_mask(mov.x + dx, mov.y + dy));
					// We have to compute the missing value of our tile.
					if mov.color == Color::Black {
						tile |= 1 << -dx << -(2*dy); // TODO: make sure this works...
					}
					tiles.push_back(tile);
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
		if top_left & self.black_mask != 0 { tile |= 1 }
		if (top_left << 1) & self.black_mask != 0 { tile |= 2 }
		if (top_left << 7) & self.black_mask != 0 { tile |= 4 }
		if (top_left << 8) & self.black_mask != 0 { tile |= 8 }
		tile
	}

	fn position_mask(x: i8, y: i8) -> u64 {
		// TODO: detail why mask are 7x7 rather than 5x5
		1u64 << (x + 7 + 1) << (7 * (y + 1))
	}

	fn tile_mask(x: i8, y: i8) -> u64 {
		/* A tile has four bits on, hence
		 * we can represent it with a number
		 * that we'll have to move exactly
		 * like a position.
		 *
		 *   0 1 2 3 4 5 6
		 *   7 8
		 *
		 * From the above grid, bits 0, 1, 7 and 8
		 * will correspond to a tile on the first
		 * place of our grid. Hence 387 is the
		 * tile to move.
		 */
		387u64 << (x + 7 + 1) << (7 * (y + 1))
	}
}

impl TryFrom<&str> for Board {
	type Error = &'static str;
	fn try_from(s: &str) -> Result<Self, Self::Error> {
		Board::from_fen(s)
	}
}

impl std::fmt::Display for Board {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "{}", self.fen());

		let filled_tiles = self.filled_tiles();
		self.opponent.fmt_with_filled_tiles(f, &filled_tiles)?;

		writeln!(f, "   a b c d e")?;
		for x in 0..5 {
			let mut line = String::new();

			for y in 0..5 {
				let position = Board::position_mask(x, y);

				if y > 0 { line.push_str(" "); }

				if self.pieces_mask & position == 0 {
					line.push_str("\x1b[30m·");
				} else if self.black_mask & position != 0 {
					line.push_str("\x1b[30m●");
				} else {
					line.push_str("\x1b[31m●");
				}
			}
			writeln!(f, "{} {} {} {}", x + 1, "\x1b[47m", line, "\x1b[0m")?;
		}
		writeln!(f, "");
		self.current_player.fmt_with_filled_tiles(f, &filled_tiles)?;

		Ok(())
	}
}

// https://www.utf8icons.com/character/11044/black-large-circle
#[test]
fn test_show_board() {
// 	assert_eq!(
// 		format!("{}", Board::from_fen("1wbw/2b/1ww/5/5").unwrap()),
// "   a b c d e
// 1 \u{1b}[47m \u{1b}[30m· \u{1b}[31m● \u{1b}[30m● \u{1b}[31m● \u{1b}[30m· \u{1b}[0m
// 2 \u{1b}[47m \u{1b}[30m· \u{1b}[30m· \u{1b}[30m● \u{1b}[30m· \u{1b}[30m· \u{1b}[0m
// 3 \u{1b}[47m \u{1b}[30m· \u{1b}[31m● \u{1b}[31m● \u{1b}[30m· \u{1b}[30m· \u{1b}[0m
// 4 \u{1b}[47m \u{1b}[30m· \u{1b}[30m· \u{1b}[30m· \u{1b}[30m· \u{1b}[30m· \u{1b}[0m
// 5 \u{1b}[47m \u{1b}[30m· \u{1b}[30m· \u{1b}[30m· \u{1b}[30m· \u{1b}[30m· \u{1b}[0m
// "
// 	);


	println!("{}", Board::from_fen("1wbw/2b/1ww/5/5").unwrap());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn current_score() {
		// assert_eq!(
		// 	Board::from_fen("1wbw/2b/1bb/5/5").unwrap().current_score(),
		// 	0
		// );
		assert_eq!(
			Board::from_fen("1wbw/2bw/1bb/5/5").unwrap().current_score(),
			-1, // score for p2
			"Score was not -1 for board below\n{}",
			Board::from_fen("1wbw/2bw/1bb/5/5").unwrap()
		);
		assert_eq!(
			Board::from_fen("1wbw/2bb/1bb/5/5").unwrap().current_score(),
			1 // score for p2
		);
	}
}
