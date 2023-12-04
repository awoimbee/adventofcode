const INPUT: &str = include_str!("../input/day02.txt");

fn solve(input: &str, game_parameters: &[u32; 3]) -> (u32, u32) {
    let mut sum_p1 = 0;
    let mut sum_p2 = 0;

    for game in input.lines() {
        let (header, mut values) = game.split_once(':').unwrap();
        let game_number: u32 = header[5..].parse().unwrap();
        values = &values[1..];
        let mut is_p1_possible = true;
        let mut min_parameters: [u32; 3] = [0, 0, 0];
        for cube_set in values.split(';') {
            for color_num in cube_set.split(',') {
                let (num, color) = color_num.trim().split_once(' ').unwrap();
                let color_index = match color {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => unreachable!(),
                };
                let num = num.parse::<u32>().unwrap();
                if num > game_parameters[color_index] {
                    is_p1_possible = false;
                }
                min_parameters[color_index] = min_parameters[color_index].max(num);
            }
        }
        if is_p1_possible {
            sum_p1 += game_number;
        }
        sum_p2 += min_parameters.into_iter().product::<u32>();
    }

    (sum_p1, sum_p2)
}

pub fn day02() -> (String, String) {
    let (sum_p1, sum_p2) = solve(INPUT, &[12, 13, 14]);
    (sum_p1.to_string(), sum_p2.to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {r#"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "#};

    #[test]
    fn test_part_1() {
        assert_eq!(solve(TEST_INPUT, &[12, 13, 14]).0, 8);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve(TEST_INPUT, &[0, 0, 0]).1, 2286);
    }
}
