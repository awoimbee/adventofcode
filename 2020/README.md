# Advent of Code 2020

[![License](https://img.shields.io/badge/License-BSD%202--Clause-orange.svg)](https://opensource.org/licenses/BSD-2-Clause)

[Advent of Code 2020](https://adventofcode.com/2020) in Rust.
This year I wanted to focus on performance.

## How to use

```bash
# run
cargo run
# run with optimizations
cargo run --release
# run with extra optimizations
RUSTFLAGS='-Ctarget-cpu=native -Copt-level=3' cargo run --release
```

## Observations

* `splitn` is faster than `split`, and indexing into slices of chars is faster than using `split`, when you know the input.

## Current results

```
Running the solutions 100 times.
#### DAY01 #### (33.02µs)
Part 1: 840324
Part2: 170098110
#### DAY02 #### (121.69µs)
Part 1: 643
Part2: 388
#### DAY03 #### (22.55µs)
Part 1: 207
Part2: 2655892800
#### DAY04 #### (91.58µs)
Part 1: 250
Part2: 158
#### DAY05 #### (71.43µs)
Part 1: 978
Part2: 727
#### DAY06 #### (379.76µs)
Part 1: 6930
Part2: 3585
#### DAY07 #### (770.84µs)
Part 1: 326
Part2: 5635
#### DAY08 #### (62.20µs)
Part 1: 2051
Part2: 2304
#### DAY09 #### (208.45µs)
Part 1: 167829540
Part2: 28045630

TOTAL TIME: 1.76ms
```
