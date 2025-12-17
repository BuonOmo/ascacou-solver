use crate::color::Color;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Move {
	pub mask: u64,
	pub color: Color,
}

impl Move {
	pub const fn from_mask(mask: u64, color: Color) -> Move {
		Move { mask, color }
	}

	pub const fn new(x: u8, y: u8, color: Color) -> Move {
		Move {
			mask: Move::mask_at(x, y),
			color,
		}
	}

	pub const fn black(x: u8, y: u8) -> Move {
		Move::new(x, y, Color::Black)
	}

	pub const fn white(x: u8, y: u8) -> Move {
		Move::new(x, y, Color::White)
	}

	pub fn is_black(&self) -> bool {
		self.color == Color::Black
	}

	pub fn is_white(&self) -> bool {
		self.color == Color::White
	}

	/// Masks are 7x7 rather than 5x5 to allow simpler bitboard
	/// computation. This means that when we shift bits by N row
	/// or column, they will fall in an unchecked area we don't
	/// care about. Note that this was an implementation choice
	/// beforehand, and it may be reduced to optimize storage.
	///
	/// With that said, our first (x, y) should start at bit
	/// 8 = 7 + 1. And each next row should start at bit 1 + (n-1) * 7.
	pub const fn mask_at(x: u8, y: u8) -> u64 {
		1 << ((1 + y as u64) * 7 + (1 + x as u64))
	}

	/// The column of the move (a, b, c, d, e).
	/// It must be in the 0..5 range, and is
	/// checked on debug builds.
	pub const fn x(&self) -> u8 {
		let zeros = self.mask.trailing_zeros() - 7;
		((zeros - 1) % 7) as u8
	}

	/// The row of the move (1, 2, 3, 4, 5).
	/// It must be in the 0..5 range, and is
	/// checked on debug builds.
	pub const fn y(&self) -> u8 {
		let zeros = self.mask.trailing_zeros() - 7;
		(zeros / 7) as u8
	}

	pub const fn shift(&self) -> u32 {
		self.mask.trailing_zeros() - 8
	}
}

impl std::ops::Not for Move {
	type Output = Move;

	fn not(self) -> Move {
		Move {
			mask: self.mask,
			color: !self.color,
		}
	}
}

/**
 * Moves should be written with two characters.
 *
 * 1. the piece is represented by b for black,
 *    w for white.
 * 2. the x axis is represented with a letter, its
 *    alphabetical appearance is the x axis.
 * 3. A digit, representing the y axis.
 *
 * Some valid moves: `wA1`, `Be5`, `bd2`.
 * Some invalid moves: `w1a`, `wf5`, `bd6`.
 */
impl std::convert::TryFrom<&str> for Move {
	type Error = &'static str;

	fn try_from(s: &str) -> Result<Move, Self::Error> {
		let mut chars = s.chars();

		let color = match chars.next() {
			Some('b' | 'B') => Color::Black,
			Some('w' | 'W') => Color::White,
			Some(_) => return Err("wrong color"),
			None => return Err("missing information"),
		};

		let x = match chars.next() {
			Some('a' | 'A') => 0,
			Some('b' | 'B') => 1,
			Some('c' | 'C') => 2,
			Some('d' | 'D') => 3,
			Some('e' | 'E') => 4,
			Some(_) => return Err("wrong x"),
			None => return Err("missing information"),
		};

		let y = match chars.next() {
			Some('1') => 0,
			Some('2') => 1,
			Some('3') => 2,
			Some('4') => 3,
			Some('5') => 4,
			Some(_) => return Err("wrong y"),
			None => return Err("missing information"),
		};

		Ok(Move::new(x, y, color))
	}
}
impl std::convert::TryFrom<String> for Move {
	type Error = &'static str;
	fn try_from(input: String) -> Result<Self, Self::Error> {
		Move::try_from(input.as_ref())
	}
}

impl std::fmt::Display for Move {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let piece = match self.color {
			Color::Black => 'b',
			Color::White => 'w',
		};

		let col = match self.x() {
			0 => 'a',
			1 => 'b',
			2 => 'c',
			3 => 'd',
			4 => 'e',
			_ => return Err(std::fmt::Error),
		};

		let row = match self.y() {
			0 => '1',
			1 => '2',
			2 => '3',
			3 => '4',
			4 => '5',
			_ => return Err(std::fmt::Error),
		};

		write!(f, "{}{}{}", piece, col, row)
	}
}

impl From<&Move> for String {
	fn from(mov: &Move) -> Self {
		format!("{}", mov)
	}
}

impl From<Move> for String {
	fn from(mov: Move) -> Self {
		format!("{}", mov)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::convert::TryFrom;

	#[test]
	fn test_try_from_string() {
		assert_eq!(Move::try_from("Ba1"), Ok(Move::new(0, 0, Color::Black)))
	}

	#[test]
	fn test_from_mask() {
		for (x, y, mask) in [
			(0u8, 0u8, 0b00001_0_0000000u64),
			(1u8, 0u8, 0b00010_0_0000000u64),
			(2u8, 3u8, 0b00100_0_0000000_0000000_0000000_0000000u64),
			(
				4u8,
				4u8,
				0b10000_0_0000000_0000000_0000000_0000000_0000000u64,
			),
		] {
			let mov_black = Move::from_mask(mask, Color::Black);
			let mov_white = Move::from_mask(mask, Color::White);
			assert_eq!(
				mov_black,
				Move::black(x as u8, y as u8),
				"position ({}, {}) is incorrect",
				x,
				y
			);
			assert_eq!(
				mov_white,
				Move::white(x as u8, y as u8),
				"position ({}, {}) is incorrect",
				x,
				y
			);
		}
	}

	#[test]
	fn test_mask_at() {
		assert_eq!(
			Move::mask_at(0, 0),
			1u64 << 8,
			"position (0, 0) is incorrect"
		);
		assert_eq!(
			Move::mask_at(1, 0),
			1u64 << 9,
			"position (1, 0) is incorrect"
		);
		assert_eq!(
			Move::mask_at(0, 1),
			1u64 << 15,
			"position (0, 1) is incorrect"
		)
	}
}
