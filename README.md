# Ascacou Solver

## Benchmark

<!-- AUTOMAGICALLY ADDED, SEE BENCHMARKS CRATE -->
| set       | depth           |  avg time |   avg n pos | pos/ms |
| --------- | --------------- | --------: | ----------: | -----: |
| endgame   | 10/10 (partial) |  940.46µs |      12 241 | 13.02K |
| midgame   | 15/15 (partial) |     2.44s |  36 588 858 | 15.02K |
| earlygame | 14/20 (partial) |    26.25s | 228 486 099 |  8.70K |
| startgame | 11/25 (partial) |     9.42s |  68 092 052 |  7.23K |
| endgame   | 10/10           |    1.89ms |      25 153 | 13.33K |
| midgame   | 15/15           |     1.95s |  29 023 017 | 14.87K |
| earlygame | 14/20           |    29.49s | 257 393 258 |  8.73K |
| startgame | 11/25           |     9.79s |  69 756 909 |  7.12K |
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
