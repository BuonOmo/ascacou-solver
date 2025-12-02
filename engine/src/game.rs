use crate::board::Board;
use crate::mov::Move;

pub struct Game {
	board: Board
}

enum Action {
	Move(Move),
	Rewind,
	Quit,
}

impl Game {
	pub fn run_new(from_board: Option<Board>) {
		let mut game = Game {
			board: from_board.unwrap_or(Board::empty()),
		};

		game.run();
	}

	fn run(&mut self) {
		let mut prev = vec![];
		loop {
			println!("\x1bc\x1b[3J"); /* clear screen */
			println!("{}", self.board.for_console());

			if self.board.is_terminal() {
				println!("Game over!");
				break;
			}
			println!(
				"Possible moves: {}",
				self.board
					.possible_moves()
					.into_iter()
					.map(String::from)
					.collect::<Vec<String>>()
					.join(", ")
			);
			println!("\nYour move (ba1 = black to the first row/col), [r]ewind or [q]uit:");

			let action = self.read_move();

			match action {
				Action::Rewind => {
					if let Some(prev_board) = prev.pop() {
						self.board = prev_board;
					}
				}
				Action::Move(mov) => {
					prev.push(self.board.clone());
					self.board = self.board.next(mov);
				}
				Action::Quit => {
					break;
				}
			}
		}
	}

	fn read_move(&self) -> Action {
		let mut mov_str = String::new();
		while let None = std::io::stdin().read_line(&mut mov_str).ok() {}
		mov_str = mov_str.trim().to_lowercase();
		match mov_str.as_str() {
			"r" | "rewind" => Action::Rewind,
			"q" | "quit" => Action::Quit,
			_ => match Move::try_from(mov_str) {
				Ok(mov) if self.board.possible_moves().contains(&mov) => Action::Move(mov),
				_ => self.read_move(),
			},
		}
	}
}
