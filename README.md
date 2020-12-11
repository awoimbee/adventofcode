# Advent of Code 2020

[![License](https://img.shields.io/badge/License-BSD%202--Clause-orange.svg)](https://opensource.org/licenses/BSD-2-Clause)

[Advent of Code 2020](https://adventofcode.com/2020) in Rust.
This year I wanted to focus on performance.

You will also find 2019's submissions in `./2019/`.

## How to use

```bash
# run
cargo run
# run with optimizations
cargo run --release
```

## Observations

* `splitn` is faster than `split`, and indexing into slices of chars is faster than using `split`, when you know the input.

## Current results

```
Running the solutions 100 times.
#### DAY01 #### (36.57µs)
Part 1: 840324
Part 2: 170098110
#### DAY02 #### (119.74µs)
Part 1: 643
Part 2: 388
#### DAY03 #### (27.92µs)
Part 1: 207
Part 2: 2655892800
#### DAY04 #### (103.95µs)
Part 1: 250
Part 2: 158
#### DAY05 #### (73.39µs)
Part 1: 978
Part 2: 727
#### DAY06 #### (314.17µs)
Part 1: 6930
Part 2: 3585
#### DAY07 #### (802.81µs)
Part 1: 326
Part 2: 5635
#### DAY08 #### (66.82µs)
Part 1: 2051
Part 2: 2304
#### DAY09 #### (206.12µs)
Part 1: 167829540
Part 2: 28045630
#### DAY10 #### (2.29µs)
Part 1: 2240
Part 2: 99214346656768
#### DAY11 #### (9.00ms)
Part 1: 2848
Part 2:

TOTAL TIME: 10.75ms
```
