# Ascacou Solver

## Benchmark

<!-- AUTOMAGICALLY ADDED, SEE BENCHMARKS CRATE -->
| sample    | depth           | avg time | avg n pos | pos/ms |
| --------- | --------------- | -------: | --------: | -----: |
| endgame   | 11/11 (partial) |  37.33µs |       118 |  3.16K |
| midgame   | 11/31 (partial) |    1.91s |  13149938 |  6.89K |
| earlygame | 8/41 (partial)  |    1.44s |  24665530 | 17.17K |
| startgame | 8/50 (partial)  | 537.89ms |  11483095 | 21.35K |
| endgame   | 11/11           |  47.84µs |       147 |  3.07K |
| midgame   | 10/31           |    1.83s |  14116085 |  7.73K |
| earlygame | 8/41            |    1.55s |  26513159 | 17.15K |
| startgame | 8/50            | 599.40ms |  12656961 | 21.12K |
<!-- AUTOMAGICALLY ADDED, SEE BENCHMARKS CRATE -->

20000 chiffre/chiffre
## Crates

- `engine` contains the core logic for the Ascacou game.
- `minicou` is the main solver, a minimax implementation for Ascacou.
- `wasm` builds a WebAssembly version of the Ascacou solver for
  use in web applications. Available on
  [NPM](https://www.npmjs.com/package/ascacou-solver-wasm)
- `py` is the Python bindings for the `engine` crate. Available on
  [PyPI](https://pypi.org/project/ascacou/).
- `benchmarks` contains benchmark tests for the minicou solver.

## Utilities

- `bitfiddle.html` lets you play with bitwise operations in the Ascacou grid.
- `Makefile` contains a few convenience commands.
