# Ascacou Solver

## Benchmark

<!-- AUTOMAGICALLY ADDED, SEE BENCHMARKS CRATE -->
| set       | depth           | avg time |   avg n pos | pos/ms |
| --------- | --------------- | -------: | ----------: | -----: |
| endgame   | 10/10 (partial) |   1.16ms |      12 138 | 10.45K |
| midgame   | 15/15 (partial) |    2.90s |  36 503 590 | 12.61K |
| earlygame | 14/20 (partial) |   30.94s | 226 224 889 |  7.31K |
| startgame | 10/25 (partial) |    5.56s |  17 377 714 |  3.13K |
| endgame   | 10/10           |   2.35ms |      24 936 | 10.61K |
| midgame   | 15/15           |    2.14s |  27 385 688 | 12.77K |
| earlygame | 13/20           |   16.85s | 120 920 055 |  7.18K |
| startgame | 10/25           |    5.80s |  18 310 577 |  3.16K |
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
