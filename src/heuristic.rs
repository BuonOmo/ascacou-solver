// TODO: add moves left to complete a card to the IA.
use crate::color::Color;
use crate::mov::Move;

struct MoveScore(Move, (u8, u8));

impl std::cmp::PartialEq for MoveScore {
    fn eq(&self, other: &Self) -> bool { self.1.eq(&other.1) }
}

impl std::cmp::PartialOrd for MoveScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { self.1.partial_cmp(&other.1) }
}

impl std::cmp::Eq for MoveScore {}

impl std::cmp::Ord for MoveScore {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.1.cmp(&other.1) }
}

pub fn sorted_moves(moves: Vec<Move>, favorite_color: Color) -> impl Iterator<Item=Move> {
	// TODO(perf): consider a simpler data structure than a BinaryHeap since our estimate
	// scores are fairly simple.
	let mut heap =  std::collections::BinaryHeap::<MoveScore>::new();
	for mov in moves {
		heap.push(
			MoveScore(mov, (
				if mov.color == favorite_color { 1 } else { 0 },
				square_score(mov.x, mov.y)
			))
		)
	}

	std::iter::from_fn(move || {
		match heap.pop() {
			Some(MoveScore(mov, _)) => Some(mov),
			None => None
		}
	})
}

const fn square_score(x: i8, y: i8) -> u8 {
	// Corners, only part of one square.
	if x == 0 && y == 0 { return 0 }
	if x == 4 && y == 4 { return 0 }
	if x == 0 && y == 4 { return 0 }
	if x == 4 && y == 0 { return 0 }

	// Edges, part of two squares.
	if x == 0 || y == 0 || x == 4 || y == 4 { return 1 }

	// Otherwise, the position may have 4 squares, hence
	// more likely to change the score.
	return 2
	// Absolute center does not matter that much, at least
	// at depth 8.
}
