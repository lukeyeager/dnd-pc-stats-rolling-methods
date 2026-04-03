# D&D Stat Rolling Analyzer

This project analyzes different methods for rolling ability scores in D&D characters via repeated simulation experiments.

## Language

Code is written in **Rust**. We use large-scale repeated experiments (not mathematical proofs) to generate statistics, so performance is important.

## Project Goal

Compare different stat-rolling methods (e.g. 3d6 straight, 4d6 drop lowest, 2d6+6, point buy equivalents, etc.) by simulating them many times and analyzing the resulting distributions.
