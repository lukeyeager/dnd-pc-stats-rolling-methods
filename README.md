# D&D Stat Rolling Analyzer

Simulates different D&D ability score generation methods millions of times and
compares the resulting distributions.

## Rolling Methods

| Name | Description |
|---|---|
| `stdarr` | Standard array: fixed values 15/14/13/12/10/8, no randomness |
| `3d6` | Roll 3d6 and add them together; repeat 6 times for 6 stats |
| `4d6` | 4d6, drop the lowest die |
| `3d6_reroll_under8` | 3d6, reroll the whole stat once if it's below 8 |
| `3d6_reroll_1s` | 3d6, reroll each die that shows a 1 (once) |
| `3d6_1s_are_6s` | 3d6, treat any 1 as a 6 |
| `roll18` | Roll 18d6, keep the top 18, group into 6 stats of 3 |
| `roll24` | Same as roll18 but with 24 dice |
| `3up3down` | Roll a d6, d8, and d10; each die produces one high and one low stat. Pair formulas: `10+d6` & `15−d6`, `10+d8` & `15−d8`, `8+d10` & `17−d10`. |
| `tictactoe` | Roll a 3×3 grid of 4d6-drop-lowest values; pick one row and one column (intersection counts in both) for 6 stats, choosing the best of all 9 combinations |
| `6x6gridMax` | Roll a 6×6 grid of 3d6 values; pick the best row/col/diagonal by lex-max (highest top stat) |
| `6x6gridTotal` | Same grid, but pick the array with the highest total sum |
| `6x6grid4d6` | Same lex-max grid, but each cell is rolled with 4d6-drop-lowest |

## Results

Average stat value by sorted position across 10,000 simulated characters.
`top1` = best stat, `top6` = worst stat, `all` = average across all six.

```
method                all    top1    top2    top3    top4    top5    top6
stdarr              12.00   15.00   14.00   13.00   12.00   10.00    8.00
4d6                 12.25   15.66   14.17   12.96   11.76   10.42    8.51
3d6                 10.50   14.24   12.45   11.11    9.88    8.54    6.77
tictactoe           13.34   16.15   16.15   14.28   12.92   11.33    9.22
6x6gridMax          11.93   16.43   14.69   12.71   11.03    9.38    7.32
6x6gridTotal        12.43   15.79   14.32   13.11   11.96   10.64    8.77
3d6_reroll_under8   11.23   14.49   12.84   11.63   10.58    9.54    8.30
3d6_reroll_1s       11.74   14.96   13.44   12.29   11.22   10.06    8.50
3d6_1s_are_6s       13.00   16.17   14.73   13.59   12.51   11.31    9.67
roll18              10.50   17.34   14.77   11.92    9.08    6.23    3.65
roll24              12.61   17.70   15.94   13.75   11.59    9.42    7.27
3up3down            12.50   16.15   14.60   13.53   11.47   10.40    8.85

```

## Adding Methods

New methods go in `src/methods.rs`. Register the name in `METHOD_NAMES` and add
a match arm in `roll_method`. The stats harness picks them up automatically.

## Building & Running

The dev profile is configured with `opt-level = 3`, so plain `cargo build/run/test` is always fully optimized — no `--release` flag needed.

```
cargo build
cargo run -- stats
cargo run -- stats --iters 100000
```

Other commands:

```
cargo run -- list           # list all available methods
cargo run -- once <method>  # roll once with a given method
```

## Tests

```
cargo test
```
