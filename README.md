# Ascacou Solver

## Benchmark

<!-- AUTOMAGICALLY ADDED, SEE BENCHMARKS CRATE -->
| sample    | depth         | avg time | avg n pos | pos/ms |
| --------- | ------------- | -------: | --------: | -----: |
| endgame   | max (partial) |  54.01µs |       118 |  2.18M |
| midgame   | 11 (partial)  |    1.83s |  13149938 |  7.18M |
| earlygame | 8 (partial)   |    1.41s |  24665530 | 17.50M |
| startgame | 8 (partial)   | 532.63ms |  11483095 | 21.56M |
| endgame   | max           |  43.97µs |       147 |  3.34M |
| midgame   | 10            |    1.76s |  14116085 |  8.04M |
| earlygame | 8             |    1.50s |  26513159 | 17.62M |
| startgame | 8             | 594.41ms |  12656961 | 21.29M |
<!-- AUTOMAGICALLY ADDED, SEE BENCHMARKS CRATE -->

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
