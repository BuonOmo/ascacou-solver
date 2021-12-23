use crate::color::Color;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Move {
	pub x: u64,
	pub y: u64,
	pub color: Color,
}

impl Move {
	pub fn new(x: u64, y: u64, color: Color) -> Move {
		Move { x, y, color }
	}

	pub fn black(x: u64, y: u64) -> Move {
		Move::new(x, y, Color::Black)
	}

	pub fn white(x: u64, y: u64) -> Move {
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
