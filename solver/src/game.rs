use crate::board::Board;
use crate::mov::Move;
use crate::solver::Solver;

pub struct Game {
	board: Board
}


impl Game {
	pub fn run_new(from_board: Option<Board>) {
		let mut game = Game { board: from_board.unwrap_or(Board::empty()) };

		game.run();
	}

	fn run(&mut self) {
		let game_over = self.board.possible_moves().is_empty();
		while !game_over {
			println!("\x1bc\x1b[3J"); /* clear screen */
			println!("{}", self.board);
			// println!("IA suggested moves:");
			// let move_scores = Solver::move_scores(&self.board, Some(4));
			// let mut i = 3;
			// for (mov, score) in move_scores {
			// 	i -= 1;
			// 	println!("— {} ({})", mov, score);
			// 	if i == 0 { break }
			// }
			if let (score, Some(mov), _) = Solver::solve(&self.board, Some(5)) {
				println!("IA top move: {} ({})", mov, score);
			}

			println!("\nYour move (ba1 = black to the first row/col):");

			let mov = self.read_move();

			self.board = self.board.next(mov);
		}
	}

	fn read_move(&self) -> Move {
		let mut mov_str = String::new();
		while let None = std::io::stdin().read_line(&mut mov_str).ok() {};
		match Move::try_from(mov_str) {
			Ok(mov) if self.board.possible_moves().contains(&mov) => mov,
			_ => self.read_move(),
		}
	}
}
