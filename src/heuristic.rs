use crate::board::Board;
use crate::color::Color;
use crate::mov::Move;
use crate::player::Player;

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

pub fn sorted_moves(moves: impl Iterator<Item=Move>, favorite_color: Color) -> impl Iterator<Item=Move> {
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

const fn square_score(x: u64, y: u64) -> u8 {
	0 // TODO
}
