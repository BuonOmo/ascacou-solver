use ascacou::Board;

fn main() {
	use rand::seq::IteratorRandom;
	use std::collections::HashMap;

	let mut rng = rand::thread_rng();
	let mut map: HashMap<i8, usize> = HashMap::new();
	let mut durations = 0;
	for _i in 0..10_000 {
		let now = std::time::Instant::now();
		let mut board = Board::random_empty();
		let mut current_player = 1i8;
		while !board.is_terminal() {
			current_player = - current_player;
			let mov = board.possible_moves().into_iter().choose(&mut rng).unwrap();
			board = board.next(mov);
		}
		let score = board.current_score();
		durations += now.elapsed().as_nanos();
		*map.entry(score).or_insert(1) += 1;
	}
	println!("total={}ns average={}ns", durations, durations / 10_000);
	println!("{:?}", map);
}
