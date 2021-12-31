# ascacou-solver-wasm

WASM interface for the ascacou solver. Only one function is exposed:

```ts
export function solve(fen: string, depth: number): number;
```

It takes a position FEN (TODO: link to what this means) and returns a number
with only its 7 least bits are interesting. Here's an example usage:

```js
import * as solver from "ascacou-solver-wasm"

solver.solve("1b3/1wbw1/1wbw1/1wwb1/5 7b3d5a40", 8) // => "bc1"
```
