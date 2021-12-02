const INPUT: &str = include_str!("../input/day02.txt");

struct Position {
    /// Forward for both parts
    x: u32,
    /// Part 1 depth, Part 2 aim
    y_aim: u32,
    /// Part 2 depth
    part2_y: u32,
}

fn solve(input: &str) -> (String, String) {
    let mut pos = Position {
        x: 0,
        y_aim: 0,
        part2_y: 0,
    };

    input.split_terminator('\n').for_each(|s| {
        let mut split = s.split(' ');
        let dir = split.next().unwrap();
        let val: u32 = split.next().unwrap().parse().unwrap();
        match dir {
            "forward" => {
                pos.x += val;
                pos.part2_y += val * pos.y_aim;
            }
            "up" => {
                pos.y_aim -= val;
            }
            "down" => {
                pos.y_aim += val;
            }
            _ => panic!(),
        }
    });
    (
        (pos.x * pos.y_aim).to_string(),
        (pos.x * pos.part2_y).to_string(),
    )
}

pub fn day02() -> (String, String) {
    solve(INPUT)
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n";

    #[test]
    fn test() {
        assert_eq!(solve(TEST_INPUT), ("150".to_string(), "900".to_string()));
    }
}
