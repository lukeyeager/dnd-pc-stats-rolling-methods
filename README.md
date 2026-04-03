# D&D Stat Rolling Analyzer

Simulates different D&D ability score generation methods millions of times and
compares the resulting distributions.

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

## Rolling Methods

| Name | Description |
|---|---|
| `stdarr` | Standard array: fixed values 15/14/13/12/10/8, no randomness |
| `roll3` | 3d6 straight |
| `roll4` | 4d6, drop the lowest die |
| `roll3_reroll_under8` | 3d6, reroll the whole stat once if it's below 8 |
| `roll3_reroll_1s` | 3d6, reroll each die that shows a 1 (once) |
| `roll3_1s_are_6s` | 3d6, treat any 1 as a 6 |
| `roll18` | Roll 18d6, keep the top 18, group into 6 stats of 3 |
| `roll24` | Same as roll18 but with 24 dice |
| `3up3down` | Three paired stats: one goes up, one goes down, using d6/d8/d10 |
| `6x6gridMax` | Roll a 6×6 grid of 3d6 values; pick the best row/col/diagonal by lex-max (highest top stat) |
| `6x6gridTotal` | Same grid, but pick the array with the highest total sum |
| `6x6grid4d6` | Same lex-max grid, but each cell is rolled with 4d6-drop-lowest |

## Results

Average stat value by sorted position across 10,000 simulated characters.
`top1` = best stat, `top6` = worst stat, `all` = average across all six.

```
method                  all    top1    top2    top3    top4    top5    top6
stdarr                12.00   15.00   14.00   13.00   12.00   10.00    8.00
roll3                 10.48   14.20   12.45   11.11    9.87    8.54    6.73
roll4                 12.25   15.68   14.19   12.97   11.76   10.41    8.49
roll3_reroll_under8   11.22   14.50   12.83   11.62   10.56    9.54    8.29
roll3_reroll_1s       11.74   14.95   13.43   12.28   11.22   10.07    8.49
roll3_1s_are_6s       13.01   16.15   14.74   13.61   12.53   11.34    9.69
roll18                10.49   17.34   14.75   11.91    9.06    6.21    3.65
roll24                12.60   17.69   15.92   13.73   11.58    9.41    7.25
3up3down              12.50   16.14   14.59   13.52   11.48   10.41    8.86
6x6gridMax            11.91   16.42   14.67   12.70   11.02    9.37    7.30
6x6gridTotal          12.44   15.79   14.32   13.13   11.97   10.66    8.78
6x6grid4d6            13.55   17.32   16.09   14.48   12.95   11.33    9.13
```

## Adding Methods

New methods go in `src/methods.rs`. Register the name in `METHOD_NAMES` and add
a match arm in `roll_method`. The stats harness picks them up automatically.

## Tests

```
cargo test
```
