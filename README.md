# Ascacou Solver

## Benchmark

<!-- AUTOMAGICALLY ADDED, SEE BENCHMARKS CRATE -->
| sample    | depth           | avg time |   avg n pos | pos/ms |
| --------- | --------------- | -------: | ----------: | -----: |
| endgame   | 21/21 (partial) |   2.63ms |      14 357 |  5.45K |
| midgame   | 23/31 (partial) |    5.55s |  31 110 946 |  5.60K |
| earlygame | 11/41 (partial) |   21.44s | 250 089 159 | 11.66K |
| startgame | 8/51 (partial)  | 476.99ms |  10 588 998 | 22.20K |
| endgame   | 21/21           |   4.20ms |      22 295 |  5.30K |
| midgame   | 31/31           |    8.28s |  46 599 666 |  5.63K |
| earlygame | 10/41           |    6.07s |  78 737 375 | 12.97K |
| startgame | 8/51            | 537.14ms |  11 683 286 | 21.75K |
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
