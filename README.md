# Ascacou Solver

## Benchmark

<!-- AUTOMAGICALLY ADDED, SEE BENCHMARKS CRATE -->
| set       | depth           | avg time |   avg n pos | pos/ms |
| --------- | --------------- | -------: | ----------: | -----: |
| endgame   | 10/10 (partial) |   1.92ms |      12 138 |  6.33K |
| midgame   | 15/15 (partial) |    2.16s |  16 392 952 |  7.60K |
| earlygame | 13/20 (partial) |   15.87s |  84 885 061 |  5.35K |
| startgame | 10/25 (partial) |    5.09s |  16 458 108 |  3.24K |
| endgame   | 10/10           |   3.91ms |      24 936 |  6.39K |
| midgame   | 15/15           |    4.04s |  30 189 216 |  7.47K |
| earlygame | 13/20           |   20.86s | 104 587 717 |  5.01K |
| startgame | 10/25           |    5.69s |  18 310 577 |  3.22K |
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
