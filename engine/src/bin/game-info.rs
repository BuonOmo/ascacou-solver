use ascacou::{Board, Move};

fn main() {
	match std::env::args().len() {
		1 => println!("{}", Board::random_empty()),
		2 => {
			let board = std::env::args()
				.nth(1)
				.and_then(|str| Board::from_fen(&str).ok())
				.unwrap();
			println!("{} {}", board.is_terminal(), board.current_score());
		}
		3 => {
			let board = std::env::args()
				.nth(1)
				.and_then(|str| Board::from_fen(&str).ok())
				.unwrap();
			let mov = std::env::args()
				.nth(2)
				.and_then(|str| Move::try_from(str).ok())
				.unwrap();
			println!("{}", board.next(&mov));
		}
		_ => panic!("nop"),
	}
}
