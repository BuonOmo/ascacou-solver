# Game Analysis

Ascacou is a strategic board game with very little randomness involved.
Since the tiles can be taken randomly, and since there is not yet a
strong solver for this game, we have to guess whether the game is
balanced or not. Here's a very naive approach with moves taken
randomly that shows that is seams to be balanced:

```rust
use ascacou::Board;
use rand::seq::IteratorRandom;
use std::collections::HashMap;

let mut rng = rand::rng();
let mut map: HashMap<i8, usize> = HashMap::new();
let mut durations = 0;
let total = 10_000;
for _i in 0..total {
	let now = std::time::Instant::now();
	let mut board = Board::random_empty(&mut rng);
	let mut current_player = 1i8;
	while !board.is_terminal() {
		current_player = -current_player;
		let mov = board.possible_moves().into_iter().choose(&mut rng).unwrap();
		board = board.next(&mov);
	}
	let score = current_player * board.current_score();
	durations += now.elapsed().as_nanos();
	*map.entry(score).or_insert(0) += 1;
}
println!("total={}ns average={}ns", durations, durations / 10_000);
println!("{:?}", map);
let avg = map
	.iter()
	.map(|(k, v)| (*k as isize) * (*v as isize))
	.sum::<isize>() as f64
	/ total as f64;
let std_dev = (map
	.iter()
	.map(|(k, v)| {
		let diff = (*k as f64) - avg;
		(diff * diff) * (*v as f64)
	})
	.sum::<f64>()
	/ total as f64)
	.sqrt();
println!("Average score: {avg:e} ± {std_dev:.4}");
assert!(avg.abs() < 0.1);
```
