# Ascacou Solver

## Benchmark

<!-- AUTOMAGICALLY ADDED, SEE BENCHMARKS CRATE -->
| sample    | depth           | avg time | avg n pos | pos/ms |
| --------- | --------------- | -------: | --------: | -----: |
| endgame   | 21/21 (partial) |   2.91ms |      8687 |  2.98K |
| midgame   | 31/31 (partial) |    3.79s |  21225601 |  5.60K |
| earlygame | 9/41 (partial)  |    2.43s |  35172577 | 14.45K |
| startgame | 8/51 (partial)  | 465.88ms |  10588998 | 22.73K |
| endgame   | 21/21           |   3.26ms |     15984 |  4.91K |
| midgame   | 31/31           |    7.31s |  41551300 |  5.68K |
| earlygame | 8/41            | 718.26ms |  12152550 | 16.92K |
| startgame | 8/51            | 508.51ms |  11683286 | 22.98K |
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
