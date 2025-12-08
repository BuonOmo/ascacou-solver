# Ascacou Solver

## Benchmark

<!-- AUTOMAGICALLY ADDED, SEE BENCHMARKS CRATE -->
| sample    | depth           | avg time | avg n pos | pos/ms |
| --------- | --------------- | -------: | --------: | -----: |
| endgame   | 21/21 (partial) |   5.34ms |     17382 |  3.26K |
| midgame   | 12/31 (partial) |   16.40s |  97532192 |  5.95K |
| earlygame | 9/41 (partial)  |    1.78s |  25564138 | 14.36K |
| startgame | 8/51 (partial)  | 539.62ms |  11925085 | 22.10K |
| endgame   | 21/21           |   8.84ms |     29981 |  3.39K |
| midgame   | 12/31           |   20.97s | 125434927 |  5.98K |
| earlygame | 9/41            |    4.75s |  68691538 | 14.47K |
| startgame | 8/51            | 583.14ms |  12974900 | 22.25K |
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
