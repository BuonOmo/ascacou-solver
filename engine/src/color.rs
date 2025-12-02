#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Color {
	Black,
	White,
}

impl std::ops::Not for Color {
	type Output = Self;

	fn not(self) -> Self::Output {
		match self {
			Color::Black => Color::White,
			Color::White => Color::Black,
		}
	}
}

impl std::fmt::Display for Color {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Color::Black => write!(f, "\x1b[31m"),
			Color::White => write!(f, "\x1b[30m"),
		}
	}
}
