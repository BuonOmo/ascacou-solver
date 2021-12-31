#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Color {
	Black,
	White
}

impl std::ops::Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black
        }
    }
}
