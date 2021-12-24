use crate::color::Color;
use crate::mov::Move;
use crate::player::Player;

pub struct Board {
	pieces_mask: u64,
	black_mask:  u64,
	white_mask:  u64,
	pub current_player: Player,
	player_1: Player,
	player_2: Player
}

impl Board {
	/**
	 * Any Ascacou position may quite simply be described by a PGN
	 * like notation. For instance:
	 *
	 *     a  b  c  d  e
	 *   1    W  B  W
	 *   2       B
	 *   3    W  W
	 *   4
	 *   5
	 *
	 * Could be refered to as `c2 B3 B1 C3 c1 D1`. This strings
	 * gives us the information of move order, and the case tells
	 * the color (white is upper cased).
	 *
	 * If we don't care about move order we can also have a FEN
	 * like reference: `1wbw/2b/1ww/5/5`.
	 *
	 * TODO: a structure that handles players positions.
	 */
	pub fn from_fen(fen: &str) -> Result<Board, &'static str> {
		let mut black_mask =  0u64;
		let mut white_mask =  0u64;

		let mut x = 0;
		let mut y: u64;
		let mut moves = 0;

		for line in fen.split('/') {
			if x > 4 { return Err("out of bound") }

			y = 0;
			for chr in line.chars() {
				if y > 4 { return Err("out of bound") }

				match chr {
					'b' => { moves += 1; black_mask |= Board::position(x, y) },
					'w' => { moves += 1; white_mask |= Board::position(x, y) },
					'1' => y += 0,
					'2' => y += 1,
					'3' => y += 2,
					'4' => y += 3,
					'5' => y += 4,
					_ => return Err("invalid character")
				}

				y += 1;
			}
			x += 1;
		}

		let (player_1, player_2) = Player::news();
		let current_player = if moves % 2 == 0 { player_1 } else { player_2 };

		Ok(Board { pieces_mask: (black_mask | white_mask),
			black_mask, white_mask, player_1, player_2, current_player })
	}

	pub fn possible_moves(&self) -> impl Iterator<Item=Move> + '_ {
		let mut result: Vec<Move> = Vec::with_capacity(50);

		for x in 0..5 {
			for y in 0..5 {
				for color in [Color::Black, Color::White] {
					if self.can_play(x, y, color) {
						result.push(Move::new(x, y, color))
					}
				}
			}
		}

		result.into_iter()
	}

	pub fn current_score(&self) -> i8 {
		let mut score = 0;

		let mut filled_tiles: std::collections::LinkedList<u8> = std::collections::LinkedList::new();
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
			let mut tile = 0u8;
			if top_left & self.black_mask != 0 { tile |= 1 }
			if (top_left << 1) & self.black_mask != 0 { tile |= 2 }
			if (top_left << 7) & self.black_mask != 0 { tile |= 4 }
			if (top_left << 8) & self.black_mask != 0 { tile |= 8 }
			filled_tiles.push_back(tile);
		}

		for tile in filled_tiles {
			if self.current_player.has_tile(tile) {
				score += 1;
			} else {
				score -= 1;
			}
		}

		score
	}

	pub fn other_player(&self) -> Player {
		if self.current_player == self.player_1 {
			self.player_2
		} else {
			self.player_1
		}
	}

	/**
	 * Apply a move without checking for its validity.
	 */
	pub fn next(&self, mov: Move) -> Board {
		let pos = Board::position(mov.x, mov.y);
		Board {
			pieces_mask: self.pieces_mask | pos,
			white_mask: if mov.color == Color::White { self.white_mask | pos } else { self.white_mask },
			black_mask: if mov.color == Color::Black { self.black_mask | pos } else { self.black_mask },
			current_player: if self.current_player == self.player_1 { self.player_2 } else { self.player_1 },
			player_1: self.player_1,
			player_2: self.player_2,
		}
	}

	fn can_play(&self, x: u64, y: u64, color: Color) -> bool {
		// TODO: also check for moves that cannot be played
		//   due to color restrictions.
		Board::position(x, y) & self.pieces_mask == 0
	}

	fn position(x: u64, y: u64) -> u64 {
		// TODO: detail why mask are 7x7 rather than 5x5
		1u64 << (x + 7 + 1) << (7 * y)
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
		writeln!(f, "{}", self.other_player())?;

		writeln!(f, "   a b c d e")?;
		for x in 0..5 {
			let mut line = String::new();

			for y in 0..5 {
				let position = Board::position(x, y);

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

		writeln!(f, "\n{}", self.current_player)?;
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
