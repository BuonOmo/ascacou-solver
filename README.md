# Ascacou Solver

## Benchmark

<!-- AUTOMAGICALLY ADDED, SEE BENCHMARKS CRATE -->
| set       | depth           | avg time |   avg n pos | pos/ms |
| --------- | --------------- | -------: | ----------: | -----: |
| endgame   | 10/10 (partial) |   6.91ms |      15 737 |  2.28K |
| midgame   | 10/15 (partial) | 757.92ms |   5 513 004 |  7.27K |
| earlygame | 9/20 (partial)  |    2.09s |  24 448 344 | 11.71K |
| startgame | 9/25 (partial)  |    7.40s | 150 004 100 | 20.28K |
| endgame   | 10/10           |   5.67ms |      29 951 |  5.28K |
| midgame   | 11/15           |    2.55s |  18 757 289 |  7.37K |
| earlygame | 9/20            |    2.23s |  31 628 285 | 14.20K |
| startgame | 9/25            |    8.25s | 155 323 168 | 18.82K |
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
