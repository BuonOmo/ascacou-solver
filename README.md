# Ascacou Solver

## Benchmark

<!-- AUTOMAGICALLY ADDED, SEE BENCHMARKS CRATE -->
| sample    | depth         | avg time | avg n pos | pos/ms |
| --------- | ------------- | -------- | --------- | ------ |
| endgame   | max (partial) | 87.62µs  | 118       | 1.35M  |
| midgame   | 11 (partial)  | 2.18s    | 13149938  | 6.04M  |
| earlygame | 7 (partial)   | 147.58ms | 1865500   | 12.64M |
| startgame | 8 (partial)   | 917.79ms | 11483095  | 12.51M |
| endgame   | max           | 47.07µs  | 147       | 3.12M  |
| midgame   | 9             | 650.03ms | 4876159   | 7.50M  |
| earlygame | 7             | 305.77ms | 3993918   | 13.06M |
| startgame | 8             | 922.10ms | 12656961  | 13.73M |
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
