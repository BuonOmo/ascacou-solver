# Ascacou Solver

## Benchmark

<!-- AUTOMAGICALLY ADDED, SEE BENCHMARKS CRATE -->
| set       | depth           | avg time |  avg n pos | pos/ms |
| --------- | --------------- | -------: | ---------: | -----: |
| endgame   | 10/10 (partial) |   2.65ms |     12 138 |  4.58K |
| midgame   | 12/15 (partial) | 752.73ms |  3 600 987 |  4.78K |
| earlygame | 13/20 (partial) |   20.07s | 68 945 459 |  3.43K |
| startgame | 10/25 (partial) |    6.15s | 12 951 739 |  2.11K |
| endgame   | 10/10           |   5.56ms |     24 936 |  4.49K |
| midgame   | 12/15           |    1.61s |  7 678 015 |  4.78K |
| earlygame | 12/20           |    7.76s | 29 681 502 |  3.83K |
| startgame | 10/25           |    5.81s | 13 905 478 |  2.39K |
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
