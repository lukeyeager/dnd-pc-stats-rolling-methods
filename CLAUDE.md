# D&D Stat Rolling Analyzer


TODO list for when I have more claude code credits:
* Re-order the methods: first standard array and 4d6 and 3d6 (the classics), then from simplest to most complicated
* Update the README results table - it's missing tictactoe currently


This project analyzes different methods for rolling ability scores in D&D characters via repeated simulation experiments.

## Language

Code is written in **Rust**. We use large-scale repeated experiments (not mathematical proofs) to generate statistics, so performance is important.

## Project Goal

Compare different stat-rolling methods (e.g. 3d6 straight, 4d6 drop lowest, 2d6+6, point buy equivalents, etc.) by simulating them many times and analyzing the resulting distributions.

## Building and Running

```
cargo build
cargo run -- stats
cargo run -- stats --iters 100000
```

The dev profile is configured with `opt-level = 3` (see `Cargo.toml`), so plain `cargo build/run/test` is always fully optimized.

## Code Guidelines

- **Color helpers**: Always pad strings before colorizing. ANSI escape codes add invisible bytes that confuse Rust's format-width calculations — padding after colorizing will misalign columns.
- **Normalization**: Heatmap colors are normalized per-column (each column scaled to its own min/max). Global normalization makes columns with naturally different ranges hard to compare visually.
- **Terminal color**: Uses `yansi` for xterm-256 color (`Color::Fixed(u8)`). The `colored` crate does not support 256-color — do not switch back to it. Truecolor (`--truecolor`) is not assumed.
- **Adding rolling methods**: New methods go in `src/methods.rs`. Register them in `METHOD_NAMES` and add a match arm in `roll_method`. The stats harness in `main.rs` picks them up automatically.

## Tests

Integration tests are in `tests/cli.rs` and invoke the compiled binary via `CARGO_BIN_EXE_dnd_stats`. Run them with:

```
cargo test
```

The tests verify: `list` output matches `METHOD_NAMES` exactly, `once <method>` succeeds for every method, and `stats --iters 100` completes successfully. The `list` test acts as a guard against `METHOD_NAMES` and `roll_method` falling out of sync when adding new methods.
