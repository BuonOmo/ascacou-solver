# Ascacou Solver

## Benchmark

<!-- AUTOMAGICALLY ADDED, SEE BENCHMARKS CRATE -->
| sample    | depth           | avg time | avg n pos | pos/ms |
| --------- | --------------- | -------: | --------: | -----: |
| endgame   | 11/11 (partial) |  58.25µs |       118 |  2.03K |
| midgame   | 11/31 (partial) |    1.82s |  13149938 |  7.21K |
| earlygame | 8/41 (partial)  |    1.42s |  24665530 | 17.35K |
| startgame | 8/50 (partial)  | 530.83ms |  11483095 | 21.63K |
| endgame   | 11/11           |  47.53µs |       147 |  3.09K |
| midgame   | 10/31           |    1.74s |  14116085 |  8.09K |
| earlygame | 8/41            |    1.53s |  26513159 | 17.37K |
| startgame | 8/50            | 592.28ms |  12656961 | 21.37K |
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
