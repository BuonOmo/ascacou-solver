# Ascacou

A subset of the rust ascacou game engine available for Python usage.

## Usage

```python
from ascacou import Board
from random import sample

board = Board("5/5/5/5/5 01234567")  # Empty board with first white tiles.


while not board.is_terminal():
	moves = board.possible_moves()
	board = board.next(sample(moves, 1)[0])

board.print()
# Score is the difference between tiles made by the current player
# and tiles made by its opponent.
print(board.score())
```

## Development

```bash
$ python3 -m venv env
$ source env/bin/activate
$ # Hack some files
$ maturin develop
$ python3
> import ascacou
> # Test stuff
$ deactivate
```
