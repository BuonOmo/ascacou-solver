# Forced Moves Exploration

We created 9 branches to play against each other, each one
with a different forced move depth, from 0 to 9.

It is clear that fm depth should not be more than 3.

## TL;DR

| match      | average score |  std dev |
| :--------- | ------------: | -------: |
| fm0 vs fm8 |           .16 | 0.619175 |
| fm0 vs fm7 |           .08 | 0.550717 |
| fm0 vs fm6 |           .18 | 0.535371 |
| fm0 vs fm5 |           .24 | 0.793193 |
| fm0 vs fm4 |           .14 | 0.568859 |
| fm0 vs fm3 |           .24 | 0.688186 |
| fm0 vs fm2 |           .22 |  0.67951 |
| fm0 vs fm1 |          -.03 | 0.640321 |

## fm0 vs fm8

| set         | average score |      std dev |
| :---------- | ------------: | -----------: |
| endgame     |             0 |            0 |
| midgame     |             0 |     0.316228 |
| earlygame   |           .50 |       1.0247 |
| startgame   |           .50 |     0.806226 |
| **Overall** |       **.16** | **0.619175** |

## fm0 vs fm7

| set         | average score |      std dev |
| :---------- | ------------: | -----------: |
| endgame     |             0 |            0 |
| midgame     |             0 |     0.316228 |
| earlygame   |           .40 |     0.969536 |
| startgame   |             0 |     0.632456 |
| **Overall** |       **.08** | **0.550717** |

## fm0 vs fm6

| set         | average score |      std dev |
| :---------- | ------------: | -----------: |
| endgame     |             0 |            0 |
| midgame     |             0 |     0.316228 |
| earlygame   |           .55 |     0.739932 |
| startgame   |           .60 |          0.8 |
| **Overall** |       **.18** | **0.535371** |

## fm0 vs fm5

| set         | average score |      std dev |
| :---------- | ------------: | -----------: |
| endgame     |             0 |            0 |
| midgame     |             0 |     0.316228 |
| earlygame   |           .70 |      1.18743 |
| startgame   |           .80 |        1.249 |
| **Overall** |       **.24** | **0.793193** |

## fm0 vs fm4

| set         | average score |      std dev |
| :---------- | ------------: | -----------: |
| endgame     |             0 |            0 |
| midgame     |             0 |     0.316228 |
| earlygame   |           .50 |      0.74162 |
| startgame   |           .30 |          1.1 |
| **Overall** |       **.14** | **0.568859** |

## fm0 vs fm3

| set         | average score |      std dev |
| :---------- | ------------: | -----------: |
| endgame     |             0 |            0 |
| midgame     |           .10 |          0.3 |
| earlygame   |           .65 |      1.10793 |
| startgame   |           .70 |          0.9 |
| **Overall** |       **.24** | **0.688186** |

## fm0 vs fm2

| set         | average score |     std dev |
| :---------- | ------------: | ----------: |
| endgame     |             0 |           0 |
| midgame     |           .10 |         0.3 |
| earlygame   |           .60 |     1.06771 |
| startgame   |           .60 |      1.0198 |
| **Overall** |       **.22** | **0.67951** |

## fm0 vs fm1

| set         | average score |      std dev |
| :---------- | ------------: | -----------: |
| endgame     |             0 |            0 |
| midgame     |             0 |     0.316228 |
| earlygame   |          -.20 |     0.812404 |
| startgame   |           .10 |      1.44568 |
| **Overall** |      **-.03** | **0.640321** |

## Scripts

```sh
# Rebase on main
for i in {0..8}; do; git checkout fm$i && git rebase main && git push --force; done
# Run every tourney
for i in {1..8}; do; gh workflow run Tourney --field old_version=fm0 --field new_version=fm$i; done
```

To grab the results there is no good way to script it.
See https://github.com/orgs/community/discussions/169117
