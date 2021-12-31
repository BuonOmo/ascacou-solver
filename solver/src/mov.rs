use crate::color::Color;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Move {
	// We use signed numbers to be able to
	// have -1 as a move when doing bit ops
	// since the bitboard is 7x7.
	pub x: i8,
	pub y: i8,
	pub color: Color,
}

impl Move {
	pub fn new(x: i8, y: i8, color: Color) -> Move {
		Move { x, y, color }
	}

	pub fn black(x: i8, y: i8) -> Move {
		Move::new(x, y, Color::Black)
	}

	pub fn white(x: i8, y: i8) -> Move {
		Move::new(x, y, Color::White)
	}

	pub fn is_black(&self) -> bool {
		self.color == Color::Black
	}

	pub fn is_white(&self) -> bool {
		self.color == Color::White
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
impl TryFrom<&str> for Move {
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

		Ok(Move { x: x, y: y, color: color })
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
			Color::White => 'w'
		};

		let col = match self.x {
			0 => 'a',
			1 => 'b',
			2 => 'c',
			3 => 'd',
			4 => 'e',
			_ => return Err(std::fmt::Error)
		};

		let row = match self.y {
			0 => '1',
			1 => '2',
			2 => '3',
			3 => '4',
			4 => '5',
			_ => return Err(std::fmt::Error)
		};

		write!(f, "{}{}{}", piece, col, row)
	}
}

impl Into<String> for Move {
	fn into(self) -> String {
		format!("{}", self)
	}
}

#[test]
fn test_try_from_string() {
	assert_eq!(
		Move::try_from("Ba1"),
		Ok(Move { color: Color::Black, x: 0, y: 0 })
	)
}
