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
}

/**
 * Moves should be written with two characters.
 *
 * 1. the color and x axis is represented with a
 *    letter. Uppercase means white, lower black,
 *    its alphabetical appearance is the x axis.
 * 2. A digit, representing the y axis.
 *
 * Some valid moves: `a1`, `E5`, `D2`.
 * Some invalid moves: `1a`, `F5`, `d6`.
 */
impl TryFrom<&str> for Move {
	type Error = &'static str;

	fn try_from(s: &str) -> Result<Move, Self::Error> {
		let mut chars = s.chars();

		let (color, x) = match chars.next() {
			Some('a') => (Color::Black, 0), Some('A') => (Color::White, 0),
			Some('b') => (Color::Black, 1), Some('B') => (Color::White, 1),
			Some('c') => (Color::Black, 2), Some('C') => (Color::White, 2),
			Some('d') => (Color::Black, 3), Some('D') => (Color::White, 3),
			Some('e') => (Color::Black, 4), Some('E') => (Color::White, 4),
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
		let chr = match (self.color, self.x) {
			(Color::Black, 0) => 'a', (Color::White, 0) => 'A',
			(Color::Black, 1) => 'b', (Color::White, 1) => 'B',
			(Color::Black, 2) => 'c', (Color::White, 2) => 'C',
			(Color::Black, 3) => 'd', (Color::White, 3) => 'D',
			(Color::Black, 4) => 'e', (Color::White, 4) => 'E',
			_ => return Err(std::fmt::Error)
		};

		let num = match self.y {
			0 => '1',
			1 => '2',
			2 => '3',
			3 => '4',
			4 => '5',
			_ => return Err(std::fmt::Error)
		};

		write!(f, "{}{}", chr, num)
	}
}
