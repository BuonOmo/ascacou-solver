# ascacou-solver-wasm

WASM interface for the ascacou solver. Every function can
throw if the input is invalid (bad fen, impossible move).

```ts
export function moves(fen: string): string[]
export function play(fen: string, move: string): string
export function solve(fen: string, depth: number): string
```

It takes a position FEN (TODO: link to what this means) and returns a number
with only its 7 least bits are interesting. Here's an example usage:

```js
import init, {moves, play, solve} from "minicou"

let minimax = (fen) => {
	return solve(fen, 8)
}

let rand = (fen) => {
	let moves = moves(fen)

	return moves[Math.floor(Math.random()*moves.length)]
}

async function main() {
	await init() // Loads WASM
	let board = "1b3/1wbw1/1wbw1/1wwb1/5 03457abd"
	let moves = moves(board)
	let players = [minimax, rand]
	let player = 0
	while (moves.length > 0) {
		const fn = players[player]
		player = 1 - player
		
		const move = fn(board)
		board = play(board, move)
		moves = moves(board)
	}
	
	console.log("Game over:", board)
}

main()
```

## Building

```
wasm-pack build --target web --out-name minicou
```
