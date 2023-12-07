const INPUT: &str = include_str!("../input/day06.txt");

/// In part one, time is small but there are many races.
fn ways_to_win_race_p1(time: i64, distance: i64) -> i64 {
    let mut ways_to_win = 0;
    for button_time in 0..time {
        // To win: speed * remaining_time > distance
        // speed = buttom_time, remaining_time = time - button_time
        // => button_time * (time - button_time) > distance
        if button_time * (time - button_time) > distance {
            ways_to_win += 1;
        } else if ways_to_win > 0 {
            // This is a quadratic function, once solutions stop coming in we can early exit!
            break;
        }
    }
    ways_to_win
}

/// For part 2 we have a single race with a large time.
/// In this case doing math is faster !
fn ways_to_win_race_p2(time: i64, distance: i64) -> i64 {
    // Bt * T - Bt > D
    // <=> -1 * (Bt^2) + T * Bt - D > 0
    // => Quadratic equation, solve for `= 0` then calculate the interval
    //   -> We know Bt can't be +inf or -inf so the sol in between the bounds
    let d = ((time.pow(2) - 4 * distance) as f64).sqrt();
    let bounds = (
        ((time as f64 + d) / 2.).floor() as i64,
        ((time as f64 - d) / 2.).ceil() as i64,
    );
    bounds.0 - bounds.1 + 1
}

fn solve_p1(input: &str) -> i64 {
    let mut lines = input.lines();
    let times: Vec<_> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|num| num.parse().unwrap())
        .collect();
    let distances: Vec<_> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|num| num.parse().unwrap())
        .collect();
    let mut p1 = 1;

    for (time, distance) in times.into_iter().zip(distances.into_iter()) {
        p1 *= ways_to_win_race_p1(time, distance);
    }

    p1
}

fn solve_p2(input: &str) -> i64 {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();

    ways_to_win_race_p2(time, distance)
}

pub fn day06() -> (String, String) {
    let p1 = solve_p1(INPUT);
    let p2 = solve_p2(INPUT);

    (p1.to_string(), p2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {r#"
        Time:      7  15   30
        Distance:  9  40  200
    "#};

    #[test]
    fn test_part_1() {
        assert_eq!(solve_p1(TEST_INPUT), 288);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_p2(TEST_INPUT), 71503);
    }
}
