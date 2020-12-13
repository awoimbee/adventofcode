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

Running the solutions 10000 times.
| DAY | Duration |      PART 1     |      Part 2     |
| :-: | :------: | :-------------: | :-------------: |
| 01  | 33.18µs  | 840324          | 170098110       |
| 02  | 112.71µs | 643             | 388             |
| 03  | 23.31µs  | 207             | 2655892800      |
| 04  | 93.40µs  | 250             | 158             |
| 05  | 72.31µs  | 978             | 727             |
| 06  | 337.25µs | 6906            | 3562            |
| 07  | 755.74µs | 326             | 5635            |
| 08  | 61.78µs  | 2051            | 2304            |
| 09  | 226.04µs | 167829540       | 28045630        |
| 10  | 2.37µs   | 2240            | 99214346656768  |
| 11  | 10.19ms  | 2412            | 2176            |
| 12  | 15.60µs  | 1106            | 107281          |
| 13  | 1.57µs   | 3997            | 500033211739354 |

TOTAL TIME: 11.93ms
