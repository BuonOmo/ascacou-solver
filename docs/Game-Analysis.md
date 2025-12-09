# Game Analysis

## Average winner score

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
println!("total={}ns average={}ns", durations, durations / total);
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

With that said, playing random games doesn't show much.

## Average number of moves

A game can stop either when the board is full (25 moves)
or when moves are no longer possible due to patterns already
formed on the board. We can estimate the average number of
moves in a game with random play:

```rust
use ascacou::Board;
use rand::seq::IteratorRandom;
use std::collections::HashMap;

let mut rng = rand::rng();
let iterations = 10_000;
let mut moves = Vec::with_capacity(iterations);
for _ in 0..iterations {
	let mut board = Board::random_empty(&mut rng);
	let mut current_moves = 0;
	while !board.is_terminal() {
		let mov = board.possible_moves().into_iter().choose(&mut rng).unwrap();
		board = board.next(&mov);
		current_moves += 1;
	}
	moves.push(current_moves.clone());
}
let moves: &[usize] = &moves;
let total: usize = moves.iter().sum();
let avg = total as f64 / iterations as f64;
let std_dev = (moves
	.iter()
	.map(|&m| {
		let diff = (m as f64) - avg;
		(diff * diff)
	})
	.sum::<f64>()
	/ total as f64)
	.sqrt();

println!("Average number of moves: {avg} ± {std_dev:.4}");
assert!(22.70 <= avg && avg <= 22.74);
```

The average number of moves in a random game is around 22.7.

Of coures, players strategies will have a huge impact on that number.
